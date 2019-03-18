use crate::systick::SysTick;
use stm32f0::stm32f0x2::Peripherals;

pub struct ButtonsHardwareImpl<'a> {
    p: &'a Peripherals,
    systick: &'a mut SysTick,
}

impl<'a> kroneum_api::buttons::ButtonsHardware for ButtonsHardwareImpl<'a> {
    fn is_button_pressed(&self, button_type: kroneum_api::buttons::ButtonType) -> bool {
        let reg = &self.p.GPIOA.idr.read();
        match button_type {
            kroneum_api::buttons::ButtonType::One => reg.idr0().bit_is_set(),
            kroneum_api::buttons::ButtonType::Ten => reg.idr2().bit_is_set(),
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

pub fn setup(p: &Peripherals) {
    // Enable wakers.
    toggle_wakers(p, true);
}

pub fn _teardown(p: &Peripherals) {
    // Disable waker.
    toggle_wakers(p, false);
}

pub fn has_pending_interrupt(p: &Peripherals) -> bool {
    let reg = p.EXTI.pr.read();
    reg.pif0().bit_is_set() || reg.pif2().bit_is_set()
}

pub fn clear_pending_interrupt(p: &Peripherals) {
    // Clear exti line 0 and 2 flags.
    p.EXTI.pr.modify(|_, w| w.pif0().set_bit().pif2().set_bit());
}

pub fn acquire<F, R>(p: &Peripherals, systick: &mut SysTick, f: F) -> R
where
    F: FnOnce(&mut kroneum_api::buttons::Buttons<ButtonsHardwareImpl>) -> R,
{
    f(&mut kroneum_api::buttons::Buttons::create(
        ButtonsHardwareImpl { p, systick },
    ))
}
