use kroneum_api::buttons::{ButtonType, ButtonsHardware};
use stm32f0::stm32f0x2::Peripherals;

pub struct ButtonsHardwareImpl<'a> {
    pub p: &'a Peripherals,
}

impl<'a> ButtonsHardware for ButtonsHardwareImpl<'a> {
    fn setup(&self) {
        // Enable wakers.
        self.p
            .PWR
            .csr
            .modify(|_, w| w.ewup1().set_bit().ewup4().set_bit());
    }

    fn teardown(&self) {
        // Disable waker.
        self.p
            .PWR
            .csr
            .modify(|_, w| w.ewup1().clear_bit().ewup4().clear_bit());
    }

    fn is_button_pressed(&self, button_type: ButtonType) -> bool {
        let reg = &self.p.GPIOA.idr.read();
        match button_type {
            ButtonType::One => reg.idr0().bit_is_set(),
            ButtonType::Ten => reg.idr2().bit_is_set(),
        }
    }

    fn is_button_triggered(&self, button_type: ButtonType) -> bool {
        let reg = &self.p.EXTI.pr.read();
        match button_type {
            ButtonType::One => reg.pif0().bit_is_set(),
            ButtonType::Ten => reg.pif2().bit_is_set(),
        }
    }

    fn reactivate_button(&self, button_type: ButtonType) {
        self.p.EXTI.pr.modify(|_, w| match button_type {
            ButtonType::One => w.pif0().set_bit(),
            ButtonType::Ten => w.pif2().set_bit(),
        });
    }
}
