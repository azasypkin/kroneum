#![no_main]
#![no_std]

extern crate panic_reset;

use crate::hal::{
    prelude::*,
    spi::Spi,
    spi::{Mode, Phase, Polarity},
    stm32,
};
use stm32f0xx_hal as hal;

use cortex_m::Peripherals;
use cortex_m_rt::entry;
use embedded_nrf24l01::{Configuration, CrcMode, DataRate, NRF24L01};
use stm32f0xx_hal::delay::Delay;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        cortex_m::interrupt::free(move |cs| {
            let mut flash = p.FLASH;
            let mut rcc = p.RCC.configure().freeze(&mut flash);

            // Configure pins for LED.
            let gpio_f = p.GPIOF.split(&mut rcc);
            let mut led = gpio_f.pf0.into_push_pull_output(cs);

            // Configure pins for SPI.
            let gpio_a = p.GPIOA.split(&mut rcc);
            let sck_pin = gpio_a.pa5.into_alternate_af0(cs);
            let miso_pin = gpio_a.pa6.into_alternate_af0(cs);
            let mosi_pin = gpio_a.pa7.into_alternate_af0(cs);
            let csn_pin = gpio_a.pa4.into_push_pull_output(cs);
            let ce_pin = gpio_a.pa3.into_push_pull_output(cs);

            // Configure SPI with 1MHz rate
            let spi = Spi::spi1(
                p.SPI1,
                (sck_pin, miso_pin, mosi_pin),
                Mode {
                    polarity: Polarity::IdleLow,
                    phase: Phase::CaptureOnFirstTransition,
                },
                1.mhz(),
                &mut rcc,
            );

            let mut nrf = NRF24L01::new(ce_pin, csn_pin, spi).unwrap();

            nrf.set_frequency(100).unwrap();
            nrf.set_tx_addr([0x11, 0x11, 0x11, 0x11, 0x11].as_ref())
                .unwrap();
            nrf.set_auto_retransmit(0, 0).unwrap();
            nrf.set_crc(Some(CrcMode::TwoBytes)).unwrap();
            nrf.set_rf(DataRate::R250Kbps, 3).unwrap();
            nrf.set_auto_ack(&[false, false, false, false, false, false])
                .unwrap();
            nrf.set_pipes_rx_enable(&[true, false, false, false, false, false])
                .unwrap();
            nrf.set_pipes_rx_lengths(&[Some(6), Some(1), Some(1), Some(1), Some(1), Some(1)])
                .unwrap();

            nrf.flush_tx().unwrap();

            // Transfer into TX
            let mut nrf = nrf.tx().unwrap();
            let mut delay = Delay::new(cp.SYST, &rcc);
            loop {
                if let Ok(true) = nrf.can_send() {
                    nrf.flush_tx().unwrap();
                    nrf.send([1, 2, 3, 4, 5, 6].as_ref()).unwrap();
                    led.toggle().unwrap();
                } else {
                    nrf.wait_empty().unwrap();
                }

                delay.delay_ms(1_000_u16);
            }
        });
    }

    loop {
        continue;
    }
}
