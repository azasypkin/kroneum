#![no_main]
#![no_std]
#![feature(uniform_paths)]
#![feature(non_exhaustive)]

extern crate panic_semihosting;

mod beeper;
mod config;
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
use usb::USB;

static CORE_PERIPHERALS: Mutex<RefCell<Option<CorePeripherals>>> = Mutex::new(RefCell::new(None));
static PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

fn system_init(peripherals: &Peripherals) {
    peripherals.RCC.cr.modify(|_, w| {
        w.hsion().set_bit();

        w.hseon().clear_bit();
        w.csson().clear_bit();
        w.pllon().clear_bit();
        w.hsebyp().clear_bit();

        w
    });

    peripherals.RCC.cr2.modify(|_, w| w.hsi14on().clear_bit());

    peripherals.RCC.cfgr.modify(|_, w| unsafe {
        w.sw().bits(0b0);
        w.hpre().bits(0b0);
        w.ppre().bits(0b0);
        w.mco().bits(0b0);

        w.pllsrc().bits(0b0);
        w.pllxtpre().clear_bit();
        w.pllmul().bits(0b0);

        w
    });

    peripherals
        .RCC
        .cfgr2
        .modify(|_, w| unsafe { w.prediv().bits(0b0) });
    peripherals.RCC.cfgr3.modify(|_, w| {
        unsafe {
            w.usart1sw().bits(0b0);
        }

        w.i2c1sw().clear_bit();
        w.cecsw().clear_bit();
        w.adcsw().clear_bit();

        w
    });
}

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[entry]
fn main() -> ! {
    free(|cs| {
        let peripherals = Peripherals::take().unwrap();
        let mut core_peripherals = cortex_m::Peripherals::take().unwrap();

        system_init(&peripherals);

        USB::configure(&peripherals, &mut core_peripherals);
        Beeper::configure(&peripherals);

        *PERIPHERALS.borrow(cs).borrow_mut() = Some(peripherals);
        *CORE_PERIPHERALS.borrow(cs).borrow_mut() = Some(core_peripherals);
    });

    interrupt_free(|mut cp, p| {
        USB::acquire(&mut cp, p, |mut usb| {
            usb.reset();
        });

        Beeper::acquire(&mut cp, p, |mut beeper| {
            beeper.beep_n(3);
        });
    });

    loop {}
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

#[interrupt]
fn USB() {
    interrupt_free(|mut cp, p| {
        USB::acquire(&mut cp, p, |mut usb| {
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
