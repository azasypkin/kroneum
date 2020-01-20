use crate::system::SystemHardwareImpl;
use kroneum_api::buttons::{ButtonType, ButtonsHardware};

impl ButtonsHardware for SystemHardwareImpl {
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
