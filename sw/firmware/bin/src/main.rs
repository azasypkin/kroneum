#![no_main]
#![no_std]
#![feature(non_exhaustive)]

extern crate panic_semihosting;

mod beeper;
mod buttons;
mod config;
mod rtc;
mod system;
mod systick;
mod usb;

use core::cell::RefCell;
use cortex_m::{
    interrupt::{free, Mutex},
    Peripherals as CorePeripherals,
};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32f0x2::{interrupt, Peripherals as DevicePeripherals};

use system::{System, SystemState};

pub struct Peripherals {
    device: DevicePeripherals,
    core: CorePeripherals,
}

struct State {
    p: Peripherals,
    system: SystemState,
}

static STATE: Mutex<RefCell<Option<State>>> = Mutex::new(RefCell::new(None));

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[entry]
fn main() -> ! {
    free(|cs| {
        *STATE.borrow(cs).borrow_mut() = Some(State {
            p: Peripherals {
                core: CorePeripherals::take().unwrap(),
                device: DevicePeripherals::take().unwrap(),
            },
            system: SystemState::default(),
        });
    });

    interrupt_free(|state| {
        init(&state.p);

        System::acquire(&mut state.p, &mut state.system, |mut system| system.setup());
    });

    loop {
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn EXTI2_3() {
    interrupt_free(|state| {
        System::acquire(&mut state.p, &mut state.system, |mut system| {
            system.on_button_press()
        });
    });
}

#[interrupt]
fn EXTI0_1() {
    interrupt_free(|state| {
        System::acquire(&mut state.p, &mut state.system, |mut system| {
            system.on_button_press()
        });
    });
}

#[interrupt]
fn RTC() {
    interrupt_free(|state| {
        System::acquire(&mut state.p, &mut state.system, |mut system| {
            system.on_rtc_alarm();
        });
    });
}

#[interrupt]
fn USB() {
    interrupt_free(|state| {
        System::acquire(&mut state.p, &mut state.system, |mut system| {
            system.on_usb_packet();
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

fn interrupt_free<F>(f: F) -> ()
where
    F: FnOnce(&mut State),
{
    free(|cs| {
        if let Some(s) = STATE.borrow(cs).borrow_mut().as_mut() {
            f(s);
        } else {
            panic!("Can not borrow application state!");
        }
    });
}

/// Initialize the system, configure clock, GPIOs and interrupts.
fn init(p: &Peripherals) {
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

    // Enable clock for GPIO Port A, B and F.
    p.device
        .RCC
        .ahbenr
        .modify(|_, w| w.iopaen().set_bit().iopben().set_bit().iopfen().set_bit());

    // Switch PA0 (button), PA2 (button), PA7 (beeper), PA11 and PA12 (usb) to alternate function
    // mode and PA1, PA3-6 to AIN to reduce power consumption.
    let moder_af = 0b10;
    let moder_ain = 0b11;
    p.device.GPIOA.moder.modify(|_, w| unsafe {
        w.moder0()
            .bits(moder_af)
            .moder1()
            .bits(moder_ain)
            .moder2()
            .bits(moder_af)
            .moder3()
            .bits(moder_ain)
            .moder4()
            .bits(moder_ain)
            .moder5()
            .bits(moder_ain)
            .moder6()
            .bits(moder_ain)
            .moder7()
            .bits(moder_af)
            .moder11()
            .bits(moder_af)
            .moder12()
            .bits(moder_af)
    });

    // Enable AIN for GPIO B and F to reduce power consumption.
    p.device
        .GPIOB
        .moder
        .modify(|_, w| unsafe { w.moder1().bits(moder_ain).moder8().bits(moder_ain) });
    p.device
        .GPIOF
        .moder
        .modify(|_, w| unsafe { w.moder0().bits(moder_ain).moder1().bits(moder_ain) });

    p.device
        .RCC
        .ahbenr
        .modify(|_, w| w.iopben().clear_bit().iopfen().clear_bit());

    // Enable pull-down for PA0 and PA2.
    let enable_pull_down = 0b10;
    p.device.GPIOA.pupdr.modify(|_, w| unsafe {
        w.pupdr0()
            .bits(enable_pull_down)
            .pupdr2()
            .bits(enable_pull_down)
    });

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
