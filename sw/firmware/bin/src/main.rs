#![no_main]
#![no_std]
#![feature(uniform_paths)]
#![feature(non_exhaustive)]

extern crate panic_semihosting;

mod beeper;
mod buttons;
mod config;
mod led;
mod rtc;
mod systick;
mod usb;

use core::cell::RefCell;
use cortex_m::{
    interrupt::{free, Mutex},
    Peripherals as CorePeripherals,
};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32f0x2::{interrupt, Peripherals};

use beeper::Beeper;
use buttons::{ButtonPressType, Buttons};
use led::{LEDColor, LED};
use rtc::{Time, RTC};
use usb::{DeviceStatus, UsbState, USB};

pub struct SystemPeripherals {
    device: Peripherals,
    core: CorePeripherals,
}

#[derive(Debug)]
enum SystemMode {
    Idle,
    Setup(u8),
    Alarm,
    Config,
}

struct SystemState {
    p: SystemPeripherals,
    mode: SystemMode,
    usb: UsbState,
}

static STATE: Mutex<RefCell<Option<SystemState>>> = Mutex::new(RefCell::new(None));

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[entry]
fn main() -> ! {
    free(|cs| {
        *STATE.borrow(cs).borrow_mut() = Some(SystemState {
            p: SystemPeripherals {
                core: cortex_m::Peripherals::take().unwrap(),
                device: Peripherals::take().unwrap(),
            },
            mode: SystemMode::Idle,
            usb: UsbState::default(),
        });
    });

    interrupt_free(|state| {
        system_init(&state.p);
        Buttons::acquire(&mut state.p, |mut buttons| buttons.setup());
        setup_standby_mode(&mut state.p);
    });

    loop {
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn EXTI2_3() {
    interrupt_free(|state| {
        let has_pending_interrupt =
            Buttons::acquire(&mut state.p, |buttons| buttons.has_pending_interrupt());
        if !has_pending_interrupt {
            return;
        }

        on_press(state);
    });
}

#[interrupt]
fn EXTI0_1() {
    interrupt_free(|state| {
        let has_pending_interrupt =
            Buttons::acquire(&mut state.p, |buttons| buttons.has_pending_interrupt());
        if !has_pending_interrupt {
            return;
        }

        on_press(state);
    });
}

#[interrupt]
fn RTC() {
    interrupt_free(|state| {
        Beeper::acquire(&mut state.p, |mut beeper| {
            beeper.setup();
            beeper.play_melody();
            beeper.teardown();
        });

        RTC::acquire(&mut state.p, |mut rtc| {
            rtc.teardown();
            rtc.clear_pending_interrupt();
        });

        state.mode = SystemMode::Idle;

        setup_standby_mode(&mut state.p);
    });
}

#[interrupt]
fn USB() {
    interrupt_free(|state| {
        USB::acquire(&mut state.p, &mut state.usb, |mut usb| usb.interrupt());
    });
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    loop {}
}

fn on_press(state: &mut SystemState) {
    let (button_i, button_x) = Buttons::acquire(&mut state.p, |mut buttons| buttons.interrupt());

    match (&state.mode, button_i, button_x) {
        (mode @ _, ButtonPressType::Long, ButtonPressType::Long) => {
            Beeper::acquire(&mut state.p, |mut beeper| {
                beeper.setup();
                beeper.play_setup();
                beeper.teardown();
            });

            let (button_i, button_x) =
                Buttons::acquire(&mut state.p, |mut buttons| buttons.interrupt());

            match (mode, button_i, button_x) {
                (SystemMode::Config, ButtonPressType::Long, ButtonPressType::Long) => {
                    Beeper::acquire(&mut state.p, |mut beeper| {
                        beeper.setup();
                        beeper.beep_n(5);
                        beeper.teardown();
                    });

                    USB::acquire(&mut state.p, &mut state.usb, |mut usb| usb.teardown());
                    setup_standby_mode(&mut state.p);

                    state.mode = SystemMode::Idle;
                }
                (_, ButtonPressType::Long, ButtonPressType::Long) => {
                    Beeper::acquire(&mut state.p, |mut beeper| {
                        beeper.setup();
                        beeper.beep_n(5);
                        beeper.teardown();
                    });

                    USB::acquire(&mut state.p, &mut state.usb, |mut usb| usb.setup());
                    teardown_standby_mode(&mut state.p);

                    state.mode = SystemMode::Config;
                }
                (SystemMode::Alarm, _, _) | (SystemMode::Idle, _, _) => {
                    state.mode = SystemMode::Setup(0)
                }
                (SystemMode::Setup(counter), _, _) => {
                    // Hour alarm.
                    Beeper::acquire(&mut state.p, |mut beeper| {
                        beeper.setup();
                        beeper.play_reset();
                        beeper.beep_n(*counter);
                        beeper.teardown();
                    });

                    RTC::acquire(&mut state.p, |mut rtc| {
                        rtc.setup();

                        let mut time = Time::default();
                        rtc.configure_time(&time);

                        time.add_hours(*counter);

                        rtc.configure_alarm(&time);
                    });

                    state.mode = SystemMode::Alarm;
                }
                _ => {}
            }
        }
        (SystemMode::Idle, ButtonPressType::Long, _)
        | (SystemMode::Idle, _, ButtonPressType::Long)
        | (SystemMode::Alarm, ButtonPressType::Long, _)
        | (SystemMode::Alarm, _, ButtonPressType::Long) => {
            Beeper::acquire(&mut state.p, |mut beeper| {
                beeper.setup();
                beeper.play_setup();
                beeper.teardown();
            });

            state.mode = SystemMode::Setup(0)
        }
        (SystemMode::Setup(counter), ButtonPressType::Long, _)
        | (SystemMode::Setup(counter), _, ButtonPressType::Long) => {
            Beeper::acquire(&mut state.p, |mut beeper| {
                beeper.setup();
                beeper.play_reset();
                beeper.beep_n(*counter);
                beeper.teardown();
            });

            RTC::acquire(&mut state.p, |mut rtc| {
                rtc.setup();

                let mut time = Time::default();
                rtc.configure_time(&time);

                match button_i {
                    ButtonPressType::Long => time.add_seconds(*counter),
                    _ => time.add_minutes(*counter),
                };

                rtc.configure_alarm(&time);
            });

            state.mode = SystemMode::Alarm;
        }
        (SystemMode::Setup(counter), ButtonPressType::Short, _) => {
            Beeper::acquire(&mut state.p, |mut beeper| {
                beeper.setup();
                beeper.beep();
                beeper.teardown();
            });

            state.mode = SystemMode::Setup(counter + 1);
        }
        (SystemMode::Setup(counter), _, ButtonPressType::Short) => {
            Beeper::acquire(&mut state.p, |mut beeper| {
                beeper.setup();
                beeper.beep();
                beeper.teardown();
            });

            state.mode = SystemMode::Setup(counter + 10);
        }
        _ => {}
    }

    let system_mode = &state.mode;
    LED::acquire(&mut state.p, |mut led| {
        match system_mode {
            SystemMode::Idle => led.blink(&LEDColor::Red),
            SystemMode::Setup(_) => led.blink(&LEDColor::Blue),
            SystemMode::Alarm => led.blink(&LEDColor::Green),
            SystemMode::Config => {
                led.blink(&LEDColor::Blue);
                led.blink(&LEDColor::Green);
                led.blink(&LEDColor::Red);
            }
        };
    });

    Buttons::acquire(&mut state.p, |button| button.clear_pending_interrupt());
}

fn interrupt_free<F>(f: F) -> ()
where
    F: FnOnce(&mut SystemState),
{
    free(|cs| {
        if let Some(s) = STATE.borrow(cs).borrow_mut().as_mut() {
            f(s);
        } else {
            panic!("Can not borrow application state!");
        }
    });
}

fn setup_standby_mode(p: &mut SystemPeripherals) {
    // Select STANDBY mode.
    p.device.PWR.cr.modify(|_, w| w.pdds().set_bit());

    clear_wakeup_flag(p);

    // Set SLEEPDEEP bit of Cortex-M0 System Control Register.
    p.core.SCB.set_sleepdeep();
}

fn teardown_standby_mode(p: &mut SystemPeripherals) {
    // Disable STANDBY mode.
    p.device.PWR.cr.modify(|_, w| w.pdds().clear_bit());

    clear_wakeup_flag(p);

    // Clear SLEEPDEEP bit of Cortex-M0 System Control Register.
    p.core.SCB.clear_sleepdeep();
}

fn clear_wakeup_flag(p: &SystemPeripherals) {
    p.device.PWR.cr.modify(|_, w| w.cwuf().set_bit());
}

/// Initialize the system, configure clock, GPIOs and interrupts.
fn system_init(p: &SystemPeripherals) {
    // Remap PA9-10 to PA11-12 for USB.
    p.device.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
    p.device
        .SYSCFG_COMP
        .syscfg_cfgr1
        .modify(|_, w| unsafe { w.pa11_pa12_rmp().set_bit().mem_mode().bits(0) });

    // -----------Buttons----------------

    // Enable EXTI0 interrupt line for PA0 and EXTI2 for PA2.
    p.device
        .SYSCFG_COMP
        .syscfg_exticr1
        .modify(|_, w| unsafe { w.exti0().bits(0).exti2().bits(0) });

    // Configure PA0/PA2 to trigger an interrupt event on the EXTI0/EXTI2 line on a rising edge.
    p.device
        .EXTI
        .rtsr
        .modify(|_, w| w.tr0().set_bit().tr2().set_bit());

    // Unmask the external interrupt line EXTI0\EXTI2 by setting the bit corresponding to the
    // EXTI0\EXTI2 "bit 0/2" in the EXT_IMR register.
    p.device
        .EXTI
        .imr
        .modify(|_, w| w.mr0().set_bit().mr2().set_bit());

    // ---------GPIO------------------

    // Enable clock for GPIO Port A.
    p.device.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());

    // Switch PA0 (button), PA2 (button), PA7 (beeper), PA11 and PA12 (usb) to alternate function
    // mode and PA3, PA4 and PA5 to output.
    let moder_af = 0b10;
    let moder_out = 0b01;
    p.device.GPIOA.moder.modify(|_, w| unsafe {
        w.moder0()
            .bits(moder_af)
            .moder2()
            .bits(moder_af)
            .moder3()
            .bits(moder_out)
            .moder4()
            .bits(moder_out)
            .moder5()
            .bits(moder_out)
            .moder7()
            .bits(moder_af)
            .moder11()
            .bits(moder_af)
            .moder12()
            .bits(moder_af)
    });

    // Enable pull-down for PA0 and PA2.
    p.device
        .GPIOA
        .pupdr
        .modify(|_, w| unsafe { w.pupdr0().bits(0b10).pupdr2().bits(0b10) });

    // Set "high" output speed for PA7, PA11 and PA12.
    let speed_high = 0b11;
    p.device.GPIOA.ospeedr.modify(|_, w| unsafe {
        w.ospeedr7()
            .bits(speed_high)
            .ospeedr11()
            .bits(speed_high)
            .ospeedr12()
            .bits(speed_high)
    });

    // Set alternative function #2 for PA0 (WKUP1), PA2 (WKUP4) and PA7 (TIM1_CH1N).
    let af2_wkup = 0b0010;
    let af2_tim1 = 0b0010;
    p.device.GPIOA.afrl.modify(|_, w| unsafe {
        w.afrl0()
            .bits(af2_wkup)
            .afrl2()
            .bits(af2_wkup)
            .afrl7()
            .bits(af2_tim1)
    });

    // Set alternative function #2 (USB) for PA11 and PA12.
    let af2_usb = 0b0010;
    p.device
        .GPIOA
        .afrh
        .modify(|_, w| unsafe { w.afrh11().bits(af2_usb).afrh12().bits(af2_usb) });
}
