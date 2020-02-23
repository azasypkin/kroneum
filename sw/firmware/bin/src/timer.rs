use crate::system::SystemHardwareImpl;
use kroneum_api::{config::CLOCK_SPEED, timer::TimerHardware};

impl TimerHardware for SystemHardwareImpl {
    fn setup(&self, frequency_hz: u32, reload_value: u32) {
        self.rcc.regs.apb1enr.modify(|_, w| w.tim2en().enabled());

        // Set prescaler, the counter clock frequency (CK_CNT) is equal to f(CK_PSC) / (PSC[15:0] + 1).
        let prescaler = (CLOCK_SPEED / frequency_hz) as u16 - 1;
        self.tim2.psc.write(|w| w.psc().bits(prescaler));
        // Set required preload value.
        self.tim2.arr.write(|w| w.arr().bits(reload_value));
        // Set URS to not trigger interrupt on UG event.
        self.tim2
            .cr1
            .modify(|_, w| w.urs().counter_only().cen().enabled());
        // Force UG event to apply prescaler immediately.
        self.tim2.egr.write(|w| w.ug().update());

        self.tim2.sr.modify(|_, w| w.uif().clear());
        self.tim2.dier.modify(|_, w| w.uie().enabled());
    }

    fn teardown(&self) {
        self.tim2.sr.modify(|_, w| w.uif().clear());
        self.tim2.cr1.modify(|_, w| w.cen().disabled());
        self.tim2.dier.modify(|_, w| w.uie().disabled());
        self.rcc.regs.apb1enr.modify(|_, w| w.tim2en().disabled());
    }
}
