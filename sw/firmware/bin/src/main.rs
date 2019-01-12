#![no_main]
#![no_std]
#![feature(uniform_paths)]
#![feature(non_exhaustive)]

extern crate panic_semihosting;

mod beeper;
mod button;
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
use button::Button;
use led::{LEDColor, LED};
use rtc::RTC;
use usb::{DeviceStatus, UsbState, USB};

pub struct AppPeripherals {
    device: Peripherals,
    core: CorePeripherals,
}

struct AppState {
    p: AppPeripherals,
    usb: UsbState,
}

static STATE: Mutex<RefCell<Option<AppState>>> = Mutex::new(RefCell::new(None));

fn interrupt_free<F>(f: F) -> ()
where
    F: FnOnce(&mut AppState),
{
    free(|cs| {
        if let Some(s) = STATE.borrow(cs).borrow_mut().as_mut() {
            f(s);
        } else {
            panic!("Can not borrow application state!");
        }
    });
}

fn start_usb_clock(p: &AppPeripherals) {
    // Enable HSI48.
    p.device.RCC.cr2.modify(|_, w| w.hsi48on().set_bit());
    while p.device.RCC.cr2.read().hsi48rdy().bit_is_clear() {}

    // Enable clock recovery system from USB SOF frames.
    p.device.RCC.apb1enr.modify(|_, w| w.crsen().set_bit());

    // Before configuration, reset CRS registers to their default values.
    p.device.RCC.apb1rstr.modify(|_, w| w.crsrst().set_bit());
    p.device.RCC.apb1rstr.modify(|_, w| w.crsrst().clear_bit());

    // Configure Frequency Error Measurement.

    // Enable Automatic trimming.
    p.device.CRS.cr.modify(|_, w| w.autotrimen().set_bit());
    // Enable Frequency error counter.
    p.device.CRS.cr.modify(|_, w| w.cen().set_bit());
}

fn stop_usb_clock(p: &AppPeripherals) {
    // Disable Frequency error counter.
    p.device.CRS.cr.modify(|_, w| w.cen().clear_bit());

    // Reset CRS registers to their default values.
    p.device.RCC.apb1rstr.modify(|_, w| w.crsrst().set_bit());
    p.device.RCC.apb1rstr.modify(|_, w| w.crsrst().clear_bit());

    // Disable clock recovery system from USB SOF frames.
    p.device.RCC.apb1enr.modify(|_, w| w.crsen().clear_bit());

    // Disable HSI48.
    p.device.RCC.cr2.modify(|_, w| w.hsi48on().clear_bit());
    while p.device.RCC.cr2.read().hsi48rdy().bit_is_set() {}
}

fn setup_standby_mode(p: &mut AppPeripherals) {
    // Select STANDBY mode.
    p.device.PWR.cr.modify(|_, w| w.pdds().set_bit());

    clear_wakeup_flag(p);

    // Set SLEEPDEEP bit of Cortex-M0 System Control Register.
    p.core.SCB.set_sleepdeep();
}

fn teardown_standby_mode(p: &mut AppPeripherals) {
    // Disable STANDBY mode.
    p.device.PWR.cr.modify(|_, w| w.pdds().clear_bit());

    clear_wakeup_flag(p);

    // Clear SLEEPDEEP bit of Cortex-M0 System Control Register.
    p.core.SCB.clear_sleepdeep();
}

fn clear_wakeup_flag(p: &AppPeripherals) {
    p.device.PWR.cr.modify(|_, w| w.cwuf().set_bit());
}

/// Initialize the system, configure clock, GPIOs and interrupts.
fn system_init(p: &AppPeripherals) {
    // Remap PA9-10 to PA11-12 for USB.
    p.device.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
    p.device
        .SYSCFG_COMP
        .syscfg_cfgr1
        .modify(|_, w| unsafe { w.pa11_pa12_rmp().set_bit().mem_mode().bits(0) });

    // -----------Buttons----------------

    // Enable EXTI0 interrupt line for PA0.
    p.device
        .SYSCFG_COMP
        .syscfg_exticr1
        .modify(|_, w| unsafe { w.exti0().bits(0) });

    // Configure PA0 to trigger an interrupt event on the EXTI0 line on a rising edge.
    p.device.EXTI.rtsr.modify(|_, w| w.tr0().set_bit());

    // Unmask the external interrupt line EXTI0 by setting the bit corresponding to the
    // EXTI0 "bit 0" in the EXT_IMR register.
    p.device.EXTI.imr.modify(|_, w| w.mr0().set_bit());

    // ---------GPIO------------------

    // Enable clock for GPIO Port A.
    p.device.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());

    // Switch PA0 (button), PA7 (beeper), PA11 and PA12 (usb) to alternate function mode and
    // PA2, PA3 and PA4 to output.
    let moder_af = 0b10;
    let moder_out = 0b01;
    p.device.GPIOA.moder.modify(|_, w| unsafe {
        w.moder0()
            .bits(moder_af)
            .moder2()
            .bits(moder_out)
            .moder3()
            .bits(moder_out)
            .moder4()
            .bits(moder_out)
            .moder7()
            .bits(moder_af)
            .moder11()
            .bits(moder_af)
            .moder12()
            .bits(moder_af)
    });

    // Enable pull-down for PA0.
    p.device
        .GPIOA
        .pupdr
        .modify(|_, w| unsafe { w.pupdr0().bits(0b10) });

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

    // Set alternative function #2 for PA0 (WKUP1) and PA7 (TIM1_CH1N).
    let af2_wkup = 0b0010;
    let af2_tim1 = 0b0010;
    p.device
        .GPIOA
        .afrl
        .modify(|_, w| unsafe { w.afrl0().bits(af2_wkup).afrl7().bits(af2_tim1) });

    // Set alternative function #2 (USB) for PA11 and PA12.
    let af2_usb = 0b0010;
    p.device
        .GPIOA
        .afrh
        .modify(|_, w| unsafe { w.afrh11().bits(af2_usb).afrh12().bits(af2_usb) });
}

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[entry]
fn main() -> ! {
    free(|cs| {
        *STATE.borrow(cs).borrow_mut() = Some(AppState {
            p: AppPeripherals {
                core: cortex_m::Peripherals::take().unwrap(),
                device: Peripherals::take().unwrap(),
            },
            usb: UsbState::default(),
        });
    });

    interrupt_free(|state| {
        system_init(&state.p);
        Button::acquire(&mut state.p, |mut button| button.setup());
        setup_standby_mode(&mut state.p);
    });

    loop {
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn EXTI0_1() {
    interrupt_free(|state| {
        LED::acquire(&mut state.p, |mut led| led.blink(&LEDColor::Blue));

        Beeper::acquire(&mut state.p, |mut beeper| {
            beeper.setup();
            beeper.play_melody();
            beeper.teardown();
        });

        if let DeviceStatus::None = state.usb.device_status {
            start_usb_clock(&state.p);
            USB::acquire(&mut state.p, &mut state.usb, |mut usb| usb.setup());
        } else {
            USB::acquire(&mut state.p, &mut state.usb, |mut usb| usb.teardown());
            stop_usb_clock(&state.p);
        }

        RTC::acquire(&mut state.p, |mut rtc| {
            rtc.setup();

            rtc.configure_time(&rtc::Time {
                hours: 1,
                minutes: 0,
                seconds: 0,
            });

            rtc.configure_alarm(&rtc::Time {
                hours: 1,
                minutes: 0,
                seconds: 10,
            });
        });

        Button::acquire(&mut state.p, |button| {
            button.clear_pending_interrupt();
        });

        teardown_standby_mode(&mut state.p)
    });
}

#[interrupt]
fn RTC() {
    interrupt_free(|state| {
        Beeper::acquire(&mut state.p, |mut beeper| {
            beeper.setup();
            beeper.play_reset();
            beeper.teardown();
        });

        USB::acquire(&mut state.p, &mut state.usb, |mut usb| usb.teardown());

        RTC::acquire(&mut state.p, |mut rtc| {
            rtc.teardown();
            rtc.clear_pending_interrupt();
        });

        setup_standby_mode(&mut state.p);
    });
}

#[interrupt]
fn USB() {
    interrupt_free(|state| {
        USB::acquire(&mut state.p, &mut state.usb, |mut usb| {
            usb.interrupt();
        });
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
