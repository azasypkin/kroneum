use kroneum_api::{
    buttons::{ButtonType, Buttons, ButtonsHardware},
    systick::{SysTick, SysTickHardware},
};
use stm32f0::stm32f0x2::Peripherals;

pub struct ButtonsHardwareImpl<'a> {
    p: &'a Peripherals,
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
}

pub fn has_pending_interrupt(p: &Peripherals) -> bool {
    let reg = p.EXTI.pr.read();
    reg.pif0().bit_is_set() || reg.pif2().bit_is_set()
}

pub fn clear_pending_interrupt(p: &Peripherals) {
    // Clear exti line 0 and 2 flags.
    p.EXTI.pr.modify(|_, w| w.pif0().set_bit().pif2().set_bit());
}

pub fn create<'a>(
    p: &'a Peripherals,
    systick: &'a mut SysTick<impl SysTickHardware>,
) -> Buttons<'a, ButtonsHardwareImpl<'a>, impl SysTickHardware> {
    Buttons::new(ButtonsHardwareImpl { p }, systick)
}
