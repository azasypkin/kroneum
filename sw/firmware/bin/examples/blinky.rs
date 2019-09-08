#![no_main]
#![no_std]

extern crate panic_halt;

use crate::hal::{delay::Delay, prelude::*, stm32};
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use stm32f0xx_hal as hal;

#[entry]
fn main() -> ! {
    if let (Some(mut p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
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
