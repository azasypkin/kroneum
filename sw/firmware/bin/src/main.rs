#![deny(warnings)]
#![allow(clippy::missing_safety_doc)]
#![no_main]
#![no_std]

extern crate panic_reset;

mod adc;
mod beeper;
mod buttons;
mod flash;
mod rtc;
mod system;
mod systick;
mod timer;
mod usb;

use crate::hal::{stm32, stm32::interrupt};
use crate::{system::SystemHardwareImpl, systick::SystickHardwareImpl};
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use kroneum_api::{system::System, systick::SysTick};
use stm32f0xx_hal as hal;

static SYSTEM: Mutex<RefCell<Option<System<SystemHardwareImpl, SystickHardwareImpl>>>> =
    Mutex::new(RefCell::new(None));
fn get_system<F>(f: F)
where
    F: FnOnce(&mut System<SystemHardwareImpl, SystickHardwareImpl>),
{
    cortex_m::interrupt::free(|cs| {
        if let Some(system) = SYSTEM.borrow(cs).borrow_mut().as_mut() {
            f(system);
        }
    });
}

#[entry]
fn main() -> ! {
    if let (Some(device), Some(core)) = (stm32::Peripherals::take(), cortex_m::Peripherals::take())
    {
        cortex_m::interrupt::free(|cs| {
            *SYSTEM.borrow(cs).borrow_mut() = Some(System::run(
                system::SystemHardwareImpl::init(device, core.SCB, core.NVIC, &cs),
                SysTick::new(SystickHardwareImpl::new(core.SYST)),
            ));
        });
    }

    loop {
        get_system(|system| system.sleep());
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn EXTI2_3() {
    get_system(|system| system.handle_button_press());
}

#[interrupt]
fn EXTI0_1() {
    get_system(|system| system.handle_button_press());
}

#[interrupt]
fn RTC() {
    get_system(|system| system.handle_alarm());
}

#[interrupt]
fn USB() {
    get_system(|system| system.handle_usb_packet());
}

#[interrupt]
fn TIM2() {
    get_system(|system| system.handle_timer());
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
    get_system(|system| system.handle_systick());
}
