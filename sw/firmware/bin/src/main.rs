#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

mod beeper;
mod buttons;
mod flash;
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
use stm32f0::stm32f0x2::{interrupt, Interrupt, Peripherals as DevicePeripherals};

static SYSTEM: Mutex<RefCell<Option<system::System<systick::SystickHardwareImpl>>>> =
    Mutex::new(RefCell::new(None));

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[entry]
fn main() -> ! {
    let mut core_peripherals = CorePeripherals::take().expect("Can not take core peripherals");
    let device_peripherals = DevicePeripherals::take().expect("Can not take device peripherals");

    free(|cs| {
        let mut system = system::System::new(
            system::SystemHardwareImpl {
                p: device_peripherals,
                scb: core_peripherals.SCB,
            },
            systick::create(core_peripherals.SYST),
        );

        system.setup();

        *SYSTEM.borrow(cs).borrow_mut() = Some(system);

        // Configure interrupts and enable.
        unsafe {
            core_peripherals.NVIC.set_priority(Interrupt::EXTI0_1, 1);
            core_peripherals.NVIC.set_priority(Interrupt::EXTI2_3, 1);
            core_peripherals.NVIC.set_priority(Interrupt::RTC, 2);
        }

        core_peripherals.NVIC.enable(Interrupt::EXTI0_1);
        core_peripherals.NVIC.enable(Interrupt::EXTI2_3);
        core_peripherals.NVIC.enable(Interrupt::RTC);
        core_peripherals.NVIC.enable(Interrupt::USB);
    });

    loop {
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn EXTI2_3() {
    interrupt_free(|system| system.on_button_press());
}

#[interrupt]
fn EXTI0_1() {
    interrupt_free(|system| system.on_button_press());
}

#[interrupt]
fn RTC() {
    interrupt_free(|system| system.on_rtc_alarm());
}

#[interrupt]
fn USB() {
    interrupt_free(|system| system.on_usb_packet());
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    panic!("hard fault (PC={})", _ef.pc);
}

fn interrupt_free<F>(f: F)
where
    F: FnOnce(&mut system::System<systick::SystickHardwareImpl>),
{
    free(|cs| {
        if let Some(s) = SYSTEM.borrow(cs).borrow_mut().as_mut() {
            f(s);
        } else {
            panic!("Can not borrow application state!");
        }
    });
}
