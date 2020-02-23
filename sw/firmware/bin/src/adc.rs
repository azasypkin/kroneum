use crate::system::SystemHardwareImpl;
use kroneum_api::adc::{ADCChannel, ADCHardware};

impl ADCHardware for SystemHardwareImpl {
    fn setup(&self) {
        self.rcc.regs.apb2enr.modify(|_, w| w.adcen().enabled());
        self.rcc.regs.cr2.modify(|_, w| w.hsi14on().on());
        while self.rcc.regs.cr2.read().hsi14rdy().is_not_ready() {}
    }

    fn calibrate(&self) {
        if self.adc.cr.read().aden().is_enabled() {
            self.adc.cr.modify(|_, w| w.addis().disable());
        }
        while self.adc.cr.read().aden().is_enabled() {}

        self.adc.cfgr1.modify(|_, w| w.dmaen().disabled());

        // Start calibration.
        self.adc.cr.modify(|_, w| w.adcal().start_calibration());

        // Wait until calibration is finished.
        while self.adc.cr.read().adcal().is_calibrating() {}
    }

    fn read(&self, channel: ADCChannel) -> u16 {
        // POWER UP
        if self.adc.isr.read().adrdy().is_ready() {
            self.adc.isr.modify(|_, w| w.adrdy().clear());
        }
        self.adc.cr.modify(|_, w| w.aden().enabled());
        while self.adc.isr.read().adrdy().is_not_ready() {}

        self.adc.chselr.write(|w| match channel {
            ADCChannel::Channel1 => w.chsel1().set_bit(),
            ADCChannel::Channel3 => w.chsel3().set_bit(),
            ADCChannel::Channel4 => w.chsel4().set_bit(),
            ADCChannel::Channel5 => w.chsel5().set_bit(),
            ADCChannel::Channel6 => w.chsel6().set_bit(),
            ADCChannel::Channel7 => w.chsel7().set_bit(),
        });

        self.adc.smpr.write(|w| w.smp().cycles239_5());
        self.adc
            .cfgr1
            .modify(|_, w| w.res().twelve_bit().align().right());

        self.adc.cr.modify(|_, w| w.adstart().start_conversion());
        while self.adc.isr.read().eoc().is_not_complete() {}

        let res = self.adc.dr.read().bits() as u16;

        // POWER DOWN
        self.adc.cr.modify(|_, w| w.adstp().stop_conversion());
        while self.adc.cr.read().adstp().is_stopping() {}
        self.adc.cr.modify(|_, w| w.addis().disable());
        while self.adc.cr.read().aden().is_enabled() {}

        res
    }

    fn teardown(&self) {
        self.rcc.regs.cr2.modify(|_, w| w.hsi14on().off());
        self.rcc.regs.apb2enr.modify(|_, w| w.adcen().disabled());
    }
}
