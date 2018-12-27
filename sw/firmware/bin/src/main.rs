#![no_main]
#![no_std]
#![feature(uniform_paths)]

extern crate panic_semihosting;

mod beeper;
mod config;
mod rtc;
mod systick;

use core::cell::RefCell;

use cortex_m::interrupt::{free, Mutex};

use cortex_m::Peripherals as CorePeripherals;
use stm32f0x2::Peripherals;

use cortex_m_rt::{entry, exception, ExceptionFrame};

use beeper::Beeper;

static CORE_PERIPHERALS: Mutex<RefCell<Option<CorePeripherals>>> = Mutex::new(RefCell::new(None));
static PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[entry]
fn main() -> ! {
    free(|cs| {
        *PERIPHERALS.borrow(cs).borrow_mut() = Some(Peripherals::take().unwrap());
        *CORE_PERIPHERALS.borrow(cs).borrow_mut() = Some(cortex_m::Peripherals::take().unwrap());
    });

    interrupt_free(|mut cp, p| {
        Beeper::configure(&p);
        Beeper::acquire(&mut cp, p, |mut beeper| {
            beeper.beep_n(3);
        });
    });

    loop {
    }
}

fn interrupt_free<F>(f: F) -> ()
    where
        F: FnOnce(&mut CorePeripherals, &Peripherals),
{
    free(|cs| {
        if let (Some(cp), Some(p)) = (
            CORE_PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
            PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
        ) {
            f(cp, p);
        } else {
            panic!("Can not borrow peripherals!");
        }
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
