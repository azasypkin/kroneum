#![no_main]
#![no_std]

extern crate panic_halt;

use stm32f0xx_hal as hal;
use crate::hal::{prelude::*, stm32};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let Some(mut p) = stm32::Peripherals::take() {
        let mut led = cortex_m::interrupt::free(|cs| {
            let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

            let gpiof = p.GPIOF.split(&mut rcc);

            // (Re-)configure PF0 as output
            gpiof.pf0.into_push_pull_output(cs)
        });

        loop {
            // Turn PF0 on 10000 times in a row
            for _ in 0..1_000_0 {
                led.set_high();
            }
            // Then turn PF0 off 10000 times in a row
            for _ in 0..1_000_0 {
                led.set_low();
            }
        }
    }

    loop {
        continue;
    }
}
