use crate::systick::SysTick;
use cortex_m::Peripherals as CorePeripherals;
use stm32f0x2::{Interrupt, Peripherals};

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
    core_peripherals: &'a mut CorePeripherals,
    peripherals: &'a Peripherals,
}

impl<'a> Button<'a> {
    fn new(core_peripherals: &'a mut CorePeripherals, peripherals: &'a Peripherals) -> Button<'a> {
        Button {
            core_peripherals,
            peripherals,
        }
    }

    pub fn start(&mut self) {
        // Set priority for the `EXTI0` line to `1`.
        unsafe {
            self.core_peripherals
                .NVIC
                .set_priority(Interrupt::EXTI0_1, 1);
        }

        // Enable the interrupt in the NVIC.
        self.core_peripherals.NVIC.enable(Interrupt::EXTI0_1);

        // Enable waker.
        self.peripherals.PWR.csr.modify(|_, w| w.ewup1().set_bit());
    }

    pub fn stop(&mut self) {
        self.core_peripherals.NVIC.disable(Interrupt::EXTI0_1);

        // Disable waker.
        self.peripherals
            .PWR
            .csr
            .modify(|_, w| w.ewup1().clear_bit());
    }

    pub fn acquire<'b, F, R>(
        core_peripherals: &'b mut CorePeripherals,
        peripherals: &'b Peripherals,
        f: F,
    ) -> R
    where
        F: FnOnce(Button) -> R,
    {
        f(Button::new(core_peripherals, peripherals))
    }

    pub fn get_press_type(&mut self, limit: PressType) -> PressType {
        if self.peripherals.GPIOA.idr.read().idr0().bit_is_clear() {
            return PressType::None;
        }

        let n = match limit {
            PressType::None => 0u8,
            PressType::Short => 2u8,
            PressType::Long => 6u8,
        };

        for i in 1..n + 1 {
            SysTick::delay_ms(&mut self.core_peripherals.SYST, 250);
            if self.peripherals.GPIOA.idr.read().idr0().bit_is_clear() {
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
        self.peripherals.EXTI.pr.modify(|_, w| w.pif0().set_bit());
    }
}
