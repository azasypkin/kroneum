use crate::system::SystemHardwareImpl;
use kroneum_api::{config::CLOCK_SPEED, timer::TimerHardware};

impl TimerHardware for SystemHardwareImpl {
    fn setup(&self, frequency_hz: u32, reload_value: u32) {
        self.p.RCC.apb1enr.modify(|_, w| w.tim2en().enabled());

        // Set prescaler, the counter clock frequency (CK_CNT) is equal to f(CK_PSC) / (PSC[15:0] + 1).
        let prescaler = (CLOCK_SPEED / frequency_hz) as u16 - 1;
        self.p.TIM2.psc.write(|w| w.psc().bits(prescaler));
        // Set required preload value.
        self.p.TIM2.arr.write(|w| w.arr().bits(reload_value));
        // Set URS to not trigger interrupt on UG event.
        self.p
            .TIM2
            .cr1
            .modify(|_, w| w.urs().counter_only().cen().enabled());
        // Force UG event to apply prescaler immediately.
        self.p.TIM2.egr.write(|w| w.ug().update());

        self.p.TIM2.sr.modify(|_, w| w.uif().clear());
        self.p.TIM2.dier.modify(|_, w| w.uie().enabled());
    }

    fn teardown(&self) {
        self.p.TIM2.sr.modify(|_, w| w.uif().clear());
        self.p.TIM2.cr1.modify(|_, w| w.cen().disabled());
        self.p.TIM2.dier.modify(|_, w| w.uie().disabled());
        self.p.RCC.apb1enr.modify(|_, w| w.tim2en().disabled());
    }
}
