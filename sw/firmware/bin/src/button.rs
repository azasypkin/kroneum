use crate::systick::SysTick;
use crate::AppPeripherals;

use stm32f0x2::Interrupt;

/// Defines type of the press (short, long, very long).
pub enum PressType {
    /// Button is not pressed.
    None,
    /// Button is keep pressed for less then a second.
    Short,
    /// Button is pressed for more than a second, but less than 3 seconds.
    Long,
}

pub struct Button<'a> {
    p: &'a mut AppPeripherals,
}

impl<'a> Button<'a> {
    fn new(p: &'a mut AppPeripherals) -> Self {
        Button { p }
    }

    pub fn setup(&mut self) {
        // Set priority for the `EXTI0` line to `1`.
        unsafe {
            self.p.core.NVIC.set_priority(Interrupt::EXTI0_1, 1);
        }

        // Enable the interrupt in the NVIC.
        self.p.core.NVIC.enable(Interrupt::EXTI0_1);

        // Enable waker.
        self.p.device.PWR.csr.modify(|_, w| w.ewup1().set_bit());
    }

    pub fn teardown(&mut self) {
        self.p.core.NVIC.disable(Interrupt::EXTI0_1);

        // Disable waker.
        self.p.device.PWR.csr.modify(|_, w| w.ewup1().clear_bit());
    }

    pub fn acquire<F, R>(p: &mut AppPeripherals, f: F) -> R
    where
        F: FnOnce(Button) -> R,
    {
        f(Button::new(p))
    }

    pub fn get_press_type(&mut self, limit: PressType) -> PressType {
        if self.p.device.GPIOA.idr.read().idr0().bit_is_clear() {
            return PressType::None;
        }

        let n = match limit {
            PressType::None => 0u8,
            PressType::Short => 2u8,
            PressType::Long => 6u8,
        };

        for i in 1..n + 1 {
            SysTick::delay_ms(&mut self.p.core.SYST, 250);
            if self.p.device.GPIOA.idr.read().idr0().bit_is_clear() {
                return match i {
                    1...2 => PressType::Short,
                    _ => PressType::Long,
                };
            }
        }

        limit
    }

    pub fn clear_pending_interrupt(&self) {
        // Clear exti line 0 flag.
        self.p.device.EXTI.pr.modify(|_, w| w.pif0().set_bit());
    }
}
