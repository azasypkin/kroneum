use crate::system::SystemHardwareImpl;
use embedded_hal::digital::v2::InputPin;
use kroneum_api::buttons::{ButtonType, ButtonsHardware};

impl ButtonsHardware for SystemHardwareImpl {
    fn is_button_pressed(&self, button_type: ButtonType) -> bool {
        match button_type {
            ButtonType::One => match self.pa0.is_high() {
                Ok(value) => value,
                // This `Result` is infallible, but `never` type isn't stabilized yet.
                Err(_) => false,
            },
            ButtonType::Ten => match self.pa2.is_high() {
                Ok(value) => value,
                // This `Result` is infallible, but `never` type isn't stabilized yet.
                Err(_) => false,
            },
        }
    }

    fn is_button_triggered(&self, button_type: ButtonType) -> bool {
        let reg = &self.exti.pr.read();
        match button_type {
            ButtonType::One => reg.pif0().bit_is_set(),
            ButtonType::Ten => reg.pif2().bit_is_set(),
        }
    }

    fn reactivate_button(&self, button_type: ButtonType) {
        self.exti.pr.modify(|_, w| match button_type {
            ButtonType::One => w.pif0().set_bit(),
            ButtonType::Ten => w.pif2().set_bit(),
        });
    }
}
