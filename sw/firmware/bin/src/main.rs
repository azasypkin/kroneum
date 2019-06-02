#![deny(warnings)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

mod beeper;
mod buttons;
mod flash;
mod kroneum;
mod rtc;
mod system;
mod systick;
mod usb;

use crate::kroneum::{Kroneum, KroneumSystem};
use core::cell::RefCell;
use cortex_m::{
    interrupt::{free, Mutex},
    Peripherals as CorePeripherals,
};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;
use rand_core::SeedableRng;
use sha3::Sha3_512 as Sha512;
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

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[entry]
fn main() -> ! {
    let core = rand_chacha::ChaChaCore::from_seed([1; 32]);
    let mut block = rand_core::block::BlockRng::new(core);

    // let secret_key = SecretKey::generate(&mut block);
    // assert_eq!(secret_key.to_bytes().len(), 32);

    let keypair: Keypair = Keypair::generate::<Sha512, _>(&mut block);
    let message: &[u8] = b"This is a test of the tsunami alert system.";
    let signature: Signature = keypair.sign::<Sha512>(message);
    assert!(keypair.verify::<Sha512>(message, &signature).is_ok());

    free(|cs| {
        *KRONEUM.borrow(cs).borrow_mut() = Some(Kroneum::run(
            DevicePeripherals::take().expect("Can not take device peripherals"),
            CorePeripherals::take().expect("Can not take core peripherals"),
        ));
    });

    loop {
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

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    panic!("hard fault (PC={})", _ef.pc);
}
