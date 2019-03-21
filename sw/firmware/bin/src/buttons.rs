use kroneum_api::{
    buttons::{ButtonType, Buttons, ButtonsHardware},
    systick::{SysTick, SysTickHardware},
};
use stm32f0::stm32f0x2::Peripherals;

pub struct ButtonsHardwareImpl<'a, S: SysTickHardware> {
    p: &'a Peripherals,
    systick: &'a mut SysTick<S>,
}

impl<'a, S: SysTickHardware> ButtonsHardware for ButtonsHardwareImpl<'a, S> {
    fn setup(&self) {
        // Enable wakers.
        toggle_wakers(self.p, true);
    }

    fn teardown(&self) {
        // Disable waker.
        toggle_wakers(self.p, false);
    }

    fn is_button_pressed(&self, button_type: ButtonType) -> bool {
        let reg = &self.p.GPIOA.idr.read();
        match button_type {
            ButtonType::One => reg.idr0().bit_is_set(),
            ButtonType::Ten => reg.idr2().bit_is_set(),
        }
    }

    fn delay(&mut self, delay_ms: u32) {
        self.systick.delay_ms(delay_ms);
    }
}

fn toggle_wakers(p: &Peripherals, toggle: bool) {
    p.PWR
        .csr
        .modify(|_, w| w.ewup1().bit(toggle).ewup4().bit(toggle));
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
) -> Buttons<ButtonsHardwareImpl<'a, impl SysTickHardware>> {
    Buttons::create(ButtonsHardwareImpl { p, systick })
}
