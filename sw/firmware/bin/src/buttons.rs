use crate::{systick, DevicePeripherals, Peripherals};
use kroneum_api::buttons;

pub struct ButtonsHardwareImpl<'a> {
    p: &'a mut Peripherals,
}

impl<'a> buttons::ButtonsHardware for ButtonsHardwareImpl<'a> {
    fn is_button_pressed(&self, button_type: buttons::ButtonType) -> bool {
        let reg = &self.p.device.GPIOA.idr.read();
        match button_type {
            buttons::ButtonType::One => reg.idr0().bit_is_set(),
            buttons::ButtonType::Ten => reg.idr2().bit_is_set(),
        }
    }

    fn delay(&mut self, delay_ms: u32) {
        systick::get(&mut self.p.systick).delay_ms(delay_ms);
    }
}

pub fn setup(p: &mut Peripherals) {
    // Enable wakers.
    p.device
        .PWR
        .csr
        .modify(|_, w| w.ewup1().set_bit().ewup4().set_bit());
}

pub fn _teardown(p: &mut Peripherals) {
    // Disable waker.
    p.device
        .PWR
        .csr
        .modify(|_, w| w.ewup1().clear_bit().ewup4().clear_bit());
}

pub fn has_pending_interrupt(p: &DevicePeripherals) -> bool {
    let reg = p.EXTI.pr.read();
    reg.pif0().bit_is_set() || reg.pif2().bit_is_set()
}

pub fn clear_pending_interrupt(p: &DevicePeripherals) {
    // Clear exti line 0 and 2 flags.
    p.EXTI.pr.modify(|_, w| w.pif0().set_bit().pif2().set_bit());
}

pub fn acquire<F, R>(p: &mut Peripherals, f: F) -> R
where
    F: FnOnce(&mut buttons::Buttons<ButtonsHardwareImpl>) -> R,
{
    f(&mut buttons::Buttons::create(ButtonsHardwareImpl { p }))
}
