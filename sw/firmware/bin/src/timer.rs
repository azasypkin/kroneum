use kroneum_api::{config::CLOCK_SPEED, timer::TimerHardware};
use stm32f0::stm32f0x2::Peripherals;

pub struct TimerHardwareImpl<'a> {
    p: &'a Peripherals,
}

impl<'a> TimerHardwareImpl<'a> {
    pub fn new(p: &'a Peripherals) -> Self {
        Self { p }
    }
}

impl<'a> TimerHardware for TimerHardwareImpl<'a> {
    fn setup(&self, frequency_hz: u32, reload_value: u32) {
        self.p.RCC.apb1enr.modify(|_, w| w.tim2en().set_bit());

        // Set prescaler, the counter clock frequency (CK_CNT) is equal to f(CK_PSC) / (PSC[15:0] + 1).
        self.p
            .TIM2
            .psc
            .write(|w| unsafe { w.bits(CLOCK_SPEED / frequency_hz - 1) });
        // Set required preload value.
        self.p.TIM2.arr.write(|w| unsafe { w.bits(reload_value) });
        // Set URS to not trigger interrupt on UG event.
        self.p
            .TIM2
            .cr1
            .modify(|_, w| w.urs().set_bit().cen().set_bit());
        // Force UG event to apply prescaler immediately.
        self.p.TIM2.egr.write(|w| w.ug().set_bit());

        self.p.TIM2.sr.modify(|_, w| w.uif().clear_bit());
        self.p.TIM2.dier.modify(|_, w| w.uie().set_bit());
    }

    fn teardown(&self) {
        self.p.TIM2.sr.modify(|_, w| w.uif().clear_bit());
        self.p.TIM2.cr1.modify(|_, w| w.cen().clear_bit());
        self.p.TIM2.dier.modify(|_, w| w.uie().clear_bit());
        self.p.RCC.apb1enr.modify(|_, w| w.tim2en().clear_bit());
    }
}
