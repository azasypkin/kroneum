use crate::{systick, Peripherals};

use stm32f0x2::Interrupt;

/// Defines type of the press (short, long, very long).
#[derive(Copy, Clone, PartialEq)]
pub enum ButtonPressType {
    /// Button is not pressed.
    None,
    /// Button is keep pressed for less then a second.
    Short,
    /// Button is pressed for more than a second, but less than 2 seconds.
    Long,
}

impl ButtonPressType {
    pub fn is_none(&self) -> bool {
        match *self {
            ButtonPressType::None => true,
            _ => false,
        }
    }
}

pub struct Buttons<'a> {
    p: &'a mut Peripherals,
}

impl<'a> Buttons<'a> {
    fn new(p: &'a mut Peripherals) -> Self {
        Buttons { p }
    }

    pub fn setup(&mut self) {
        // Set priority for the `EXTI0` and `EXTI2` line to `1`.
        unsafe {
            self.p.core.NVIC.set_priority(Interrupt::EXTI0_1, 1);
            self.p.core.NVIC.set_priority(Interrupt::EXTI2_3, 1);
        }

        // Enable the interrupt in the NVIC.
        self.p.core.NVIC.enable(Interrupt::EXTI0_1);
        self.p.core.NVIC.enable(Interrupt::EXTI2_3);

        // Enable wakers.
        self.p
            .device
            .PWR
            .csr
            .modify(|_, w| w.ewup1().set_bit().ewup4().set_bit());
    }

    pub fn teardown(&mut self) {
        self.p.core.NVIC.disable(Interrupt::EXTI0_1);
        self.p.core.NVIC.disable(Interrupt::EXTI2_3);

        // Disable waker.
        self.p
            .device
            .PWR
            .csr
            .modify(|_, w| w.ewup1().clear_bit().ewup4().clear_bit());
    }

    pub fn acquire<F, R>(p: &mut Peripherals, f: F) -> R
    where
        F: FnOnce(Buttons) -> R,
    {
        f(Buttons::new(p))
    }

    pub fn interrupt(&mut self) -> (ButtonPressType, ButtonPressType) {
        let reg = &self.p.device.GPIOA.idr.read();

        let mut button_one_state = if reg.idr0().bit_is_set() {
            ButtonPressType::Short
        } else {
            ButtonPressType::None
        };

        let mut button_ten_state = if reg.idr2().bit_is_set() {
            ButtonPressType::Short
        } else {
            ButtonPressType::None
        };

        if button_one_state.is_none() && button_ten_state.is_none() {
            return (button_one_state, button_ten_state);
        }

        for i in 1u8..8u8 {
            systick::get(&mut self.p.core.SYST).delay_ms(250);
            let reg = self.p.device.GPIOA.idr.read();
            if reg.idr0().bit_is_clear() && reg.idr2().bit_is_clear() {
                break;
            }

            let (new_state, works_for_none) = match i {
                0...4 => (ButtonPressType::Short, true),
                5...8 => (ButtonPressType::Long, false),
                _ => break,
            };

            if reg.idr0().bit_is_set() && (!button_one_state.is_none() || works_for_none) {
                button_one_state = new_state;
            }

            if reg.idr2().bit_is_set() && (!button_ten_state.is_none() || works_for_none) {
                button_ten_state = new_state;
            }
        }

        (button_one_state, button_ten_state)
    }

    pub fn has_pending_interrupt(&self) -> bool {
        let reg = self.p.device.EXTI.pr.read();
        reg.pif0().bit_is_set() || reg.pif2().bit_is_set()
    }

    pub fn clear_pending_interrupt(&self) {
        // Clear exti line 0 and 2 flags.
        self.p
            .device
            .EXTI
            .pr
            .modify(|_, w| w.pif0().set_bit().pif2().set_bit());
    }
}
