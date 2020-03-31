#![no_main]
#![no_std]

extern crate panic_reset;

use crate::hal::{i2c::I2c, prelude::*, stm32};
use stm32f0xx_hal as hal;

use cortex_m_rt::entry;
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Line,
    style::{PrimitiveStyle, TextStyleBuilder},
};
use ssd1306::{prelude::*, Builder};

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

            Text::new("Hello Kroneum!", Point::zero())
                .into_styled(
                    TextStyleBuilder::new(Font6x8)
                        .text_color(BinaryColor::On)
                        .build(),
                )
                .draw(&mut disp)
                .unwrap();

            Line::new(Point::new(0, 16), Point::new(16, 16))
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                .draw(&mut disp)
                .unwrap();

            disp.flush().unwrap();
        });
    }

    loop {
        continue;
    }
}
