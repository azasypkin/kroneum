#![no_main]
#![no_std]

extern crate panic_halt;

use crate::hal::{i2c::I2c, prelude::*, stm32};
use stm32f0xx_hal as hal;

use cortex_m_rt::entry;
use embedded_graphics::fonts::Font6x8;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;
use ssd1306::prelude::*;
use ssd1306::Builder;

#[entry]
fn main() -> ! {
    if let Some(p) = stm32::Peripherals::take() {
        cortex_m::interrupt::free(move |cs| {
            let mut flash = p.FLASH;
            let mut rcc = p.RCC.configure().freeze(&mut flash);

            let gpio = p.GPIOF.split(&mut rcc);

            // Configure pins for I2C
            let sda = gpio.pf0.into_alternate_af1(cs);
            let scl = gpio.pf1.into_alternate_af1(cs);

            let i2c = I2c::i2c1(p.I2C1, (scl, sda), 100_u32.khz(), &mut rcc);
            let mut disp: GraphicsMode<_> = Builder::new()
                .size(DisplaySize::Display128x32)
                .connect_i2c(i2c)
                .into();
            disp.init().unwrap();
            disp.flush().unwrap();

            disp.draw(
                Font6x8::render_str("Hello Kroneum!")
                    .stroke(Some(BinaryColor::On))
                    .into_iter(),
            );
            disp.draw(
                Font6x8::render_str("Hello Rust!")
                    .stroke(Some(BinaryColor::On))
                    .translate(Point::new(0, 16))
                    .into_iter(),
            );

            disp.flush().unwrap();
        });
    }

    loop {
        continue;
    }
}
