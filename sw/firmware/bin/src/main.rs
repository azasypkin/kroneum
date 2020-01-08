#![deny(warnings)]
#![allow(clippy::missing_safety_doc)]
#![no_main]
#![no_std]

extern crate panic_reset;

mod beeper;
mod buttons;
mod flash;
mod kroneum;
mod rtc;
mod system;
mod systick;
mod timer;
mod usb;

use crate::kroneum::{Kroneum, KroneumSystem};
use core::cell::RefCell;
use cortex_m::{
    interrupt::{free, Mutex},
    Peripherals as CorePeripherals,
};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32f0::stm32f0x2::{interrupt, Peripherals as DevicePeripherals};

static KRONEUM: Mutex<RefCell<Option<Kroneum>>> = Mutex::new(RefCell::new(None));
fn get_system<F>(f: F)
where
    F: FnOnce(KroneumSystem),
{
    free(|cs| {
        f(KRONEUM
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .expect("Can not borrow application state!")
            .system());
    });
}

#[entry]
fn main() -> ! {
    free(|cs| {
        *KRONEUM.borrow(cs).borrow_mut() = Some(Kroneum::run(
            DevicePeripherals::take().expect("Can not take device peripherals"),
            CorePeripherals::take().expect("Can not take core peripherals"),
        ));
    });

    loop {
        get_system(|mut system| system.sleep());
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn EXTI2_3() {
    get_system(|mut system| system.handle_button_press());
}

#[interrupt]
fn EXTI0_1() {
    get_system(|mut system| system.handle_button_press());
}

#[interrupt]
fn RTC() {
    get_system(|mut system| system.handle_alarm());
}

#[interrupt]
fn USB() {
    get_system(|mut system| system.handle_usb_packet());
}

#[interrupt]
fn TIM2() {
    get_system(|mut system| system.handle_timer());
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    panic!("hard fault (PC={})", _ef.pc);
}

#[exception]
fn SysTick() {
    get_system(|mut system| system.handle_systick());
}
