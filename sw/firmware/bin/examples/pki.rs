#![no_main]
#![no_std]

extern crate panic_halt;

use crate::hal::{delay::Delay, prelude::*, stm32};
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use stm32f0xx_hal as hal;
use ed25519_dalek::{Keypair, SecretKey};
use ed25519_dalek::Signature;
use rand_core::SeedableRng;
use sha3::Sha3_512 as Sha512;

#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let core = rand_chacha::ChaChaCore::from_seed([1; 32]);
        let mut block = rand_core::block::BlockRng::new(core);

        let secret_key = SecretKey::generate(&mut block);
        assert_eq!(secret_key.to_bytes().len(), 32);

        let keypair: Keypair = Keypair::generate::<Sha512, _>(&mut block);
        let message: &[u8] = b"This is a test of the tsunami alert system.";
        let signature: Signature = keypair.sign::<Sha512>(message);
        assert!(keypair.verify::<Sha512>(message, &signature).is_ok());

        cortex_m::interrupt::free(move |cs| {
            let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

            let gpio = p.GPIOF.split(&mut rcc);

            // (Re-)configure PF0 as output
            let mut led = gpio.pf0.into_push_pull_output(cs);

            // Get delay provider
            let mut delay = Delay::new(cp.SYST, &rcc);

            loop {
                led.toggle();
                delay.delay_ms(1_000_u16);
            }
        });
    }

    loop {
        continue;
    }
}
