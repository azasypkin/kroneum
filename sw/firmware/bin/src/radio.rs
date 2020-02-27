use crate::hal::stm32::SPI1;
use crate::system::SystemHardwareImpl;
use cortex_m::interrupt::CriticalSection;
use embedded_hal::blocking::spi::Transfer;
use embedded_hal::digital::v2::OutputPin;
use kroneum_api::{array::Array, radio::RadioHardware};
use stm32f0xx_hal::gpio::{
    gpioa::{PA3, PA4, PA5, PA6, PA7},
    Alternate, Output, PushPull, AF0,
};
use stm32f0xx_hal::spi::{Mode, Phase, Polarity, Spi};
use stm32f0xx_hal::time::U32Ext;

pub struct SPIBus {
    spi: Spi<SPI1, PA5<Alternate<AF0>>, PA6<Alternate<AF0>>, PA7<Alternate<AF0>>>,
    ce: PA3<Output<PushPull>>,
    csn: PA4<Output<PushPull>>,
}

impl RadioHardware for SystemHardwareImpl {
    fn setup(&mut self, cs: &CriticalSection) {
        // Check if all SPI pins are available currently.
        match (
            self.spi.take(),
            self.pa3.take(),
            self.pa4.take(),
            self.pa5.take(),
            self.pa6.take(),
            self.pa7.take(),
        ) {
            (Some(spi), Some(pa3), Some(pa4), Some(pa5), Some(pa6), Some(pa7)) => {
                // Configure SPI with 1MHz rate
                let mut spi_bus = SPIBus {
                    spi: Spi::spi1(
                        spi,
                        (
                            pa5.into_alternate_af0(cs),
                            pa6.into_alternate_af0(cs),
                            pa7.into_alternate_af0(cs),
                        ),
                        Mode {
                            polarity: Polarity::IdleLow,
                            phase: Phase::CaptureOnFirstTransition,
                        },
                        1.mhz(),
                        &mut self.rcc,
                    ),
                    ce: pa3.into_push_pull_output(cs),
                    csn: pa4.into_push_pull_output(cs),
                };

                // We can never have error here (error types are Infallible).
                spi_bus.ce.set_low().unwrap();
                spi_bus.csn.set_high().unwrap();

                self.spi_bus = Some(spi_bus);
            }
            (spi, pa3, pa4, pa5, pa6, pa7) => {
                self.spi = spi;
                self.pa3 = pa3;
                self.pa4 = pa4;
                self.pa5 = pa5;
                self.pa6 = pa6;
                self.pa7 = pa7;
            }
        }
    }

    fn transfer(&mut self, mut payload: Array<u8>) -> Result<Array<u8>, ()> {
        self.spi_bus.as_mut().ok_or_else(|| ()).and_then(|bus| {
            // We can never have error here (error type is Infallible).
            bus.csn.set_low().unwrap();

            let transfer_result = bus.spi.transfer(payload.as_mut());

            // We can never have error here (error type is Infallible).
            bus.csn.set_high().unwrap();

            transfer_result.map_err(|_| ())?;

            Ok(payload)
        })
    }

    fn enable_chip(&mut self) -> Result<(), ()> {
        // We can never have error when toggling pin state (error type is Infallible).
        self.spi_bus
            .as_mut()
            .ok_or_else(|| ())
            .map(|bus| bus.ce.set_high().unwrap())
    }

    fn disable_chip(&mut self) -> Result<(), ()> {
        // We can never have error when toggling pin state (error type is Infallible).
        self.spi_bus
            .as_mut()
            .ok_or_else(|| ())
            .map(|bus| bus.ce.set_low().unwrap())
    }

    fn teardown(&mut self, cs: &CriticalSection) {
        if let Some(mut bus) = self.spi_bus.take() {
            // We can never have error when toggling pin state (error type is Infallible).
            bus.ce.set_low().unwrap();

            let (spi, (pa5, pa6, pa7)) = bus.spi.release();
            self.spi = Some(spi);
            self.pa3 = Some(bus.ce.into_analog(cs));
            self.pa4 = Some(bus.csn.into_analog(cs));
            self.pa5 = Some(pa5.into_analog(cs));
            self.pa6 = Some(pa6.into_analog(cs));
            self.pa7 = Some(pa7.into_analog(cs));
        }
    }
}
