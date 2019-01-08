use crate::systick::SysTick;
use cortex_m::Peripherals as CorePeripherals;
use stm32f0x2::Interrupt;
use stm32f0x2::Peripherals;

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

    pub fn configure(peripherals: &Peripherals, core_peripherals: &mut CorePeripherals) {
        // Enable EXTI0 interrupt line for PA0.
        peripherals
            .SYSCFG_COMP
            .syscfg_exticr1
            .modify(|_, w| unsafe { w.exti0().bits(0) });

        // Configure PA0 to trigger an interrupt event on the EXTI0 line on a rising edge.
        peripherals.EXTI.rtsr.modify(|_, w| w.tr0().set_bit());

        // Unmask the external interrupt line EXTI0 by setting the bit corresponding to the
        // EXTI0 "bit 0" in the EXT_IMR register.
        peripherals.EXTI.imr.modify(|_, w| w.mr0().set_bit());

        // Enable clock for GPIO Port A.
        peripherals.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());

        // Pull-down.
        peripherals
            .GPIOA
            .pupdr
            .modify(|_, w| unsafe { w.pupdr0().bits(0b10) });

        // Switch PA0 to alternate function mode.
        peripherals.GPIOA.moder.modify(|_, w| unsafe {
            w.moder0().bits(0b10);
            w.moder2().bits(0b01);
            w.moder3().bits(0b01);
            w.moder4().bits(0b01)
        });

        // Set alternative function #2.
        peripherals
            .GPIOA
            .afrl
            .modify(|_, w| unsafe { w.afrl0().bits(0b0010) });

        // Set priority for the `EXTI0` line to `1`.
        unsafe {
            core_peripherals.NVIC.set_priority(Interrupt::EXTI0_1, 1);
        }
        // Enable the interrupt in the NVIC.
        core_peripherals.NVIC.enable(Interrupt::EXTI0_1);

        // Enable waker
        //peripherals.PWR.csr.modify(|_, w| w.ewup1().set_bit());
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

        let n: u8 = match limit {
            PressType::None => 0,
            PressType::Short => 2,
            PressType::Long => 6,
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
        self.peripherals.EXTI.pr.modify(|_, w| {
            return w.pif0().set_bit();
        });
    }
}
