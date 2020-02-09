use crate::system::SystemHardwareImpl;
use kroneum_api::adc::{ADCChannel, ADCHardware};

impl ADCHardware for SystemHardwareImpl {
    fn setup(&self) {
        self.p.RCC.apb2enr.modify(|_, w| w.adcen().enabled());
        self.p.RCC.cr2.modify(|_, w| w.hsi14on().on());
        while self.p.RCC.cr2.read().hsi14rdy().is_not_ready() {}
    }

    fn calibrate(&self) {
        if self.p.ADC.cr.read().aden().is_enabled() {
            self.p.ADC.cr.modify(|_, w| w.addis().disable());
        }
        while self.p.ADC.cr.read().aden().is_enabled() {}

        self.p.ADC.cfgr1.modify(|_, w| w.dmaen().disabled());

        // Start calibration.
        self.p.ADC.cr.modify(|_, w| w.adcal().start_calibration());

        // Wait until calibration is finished.
        while self.p.ADC.cr.read().adcal().is_calibrating() {}
    }

    fn read(&self, channel: ADCChannel) -> u16 {
        // POWER UP
        if self.p.ADC.isr.read().adrdy().is_ready() {
            self.p.ADC.isr.modify(|_, w| w.adrdy().clear());
        }
        self.p.ADC.cr.modify(|_, w| w.aden().enabled());
        while self.p.ADC.isr.read().adrdy().is_not_ready() {}

        self.p.ADC.chselr.write(|w| match channel {
            ADCChannel::Channel1 => w.chsel1().set_bit(),
            ADCChannel::Channel3 => w.chsel3().set_bit(),
            ADCChannel::Channel4 => w.chsel4().set_bit(),
            ADCChannel::Channel5 => w.chsel5().set_bit(),
            ADCChannel::Channel6 => w.chsel6().set_bit(),
            ADCChannel::Channel7 => w.chsel7().set_bit(),
        });

        self.p.ADC.smpr.write(|w| w.smp().cycles239_5());
        self.p
            .ADC
            .cfgr1
            .modify(|_, w| w.res().twelve_bit().align().right());

        self.p.ADC.cr.modify(|_, w| w.adstart().start_conversion());
        while self.p.ADC.isr.read().eoc().is_not_complete() {}

        let res = self.p.ADC.dr.read().bits() as u16;

        // POWER DOWN
        self.p.ADC.cr.modify(|_, w| w.adstp().stop_conversion());
        while self.p.ADC.cr.read().adstp().is_stopping() {}
        self.p.ADC.cr.modify(|_, w| w.addis().disable());
        while self.p.ADC.cr.read().aden().is_enabled() {}

        res
    }

    fn teardown(&self) {
        self.p.RCC.cr2.modify(|_, w| w.hsi14on().off());
        self.p.RCC.apb2enr.modify(|_, w| w.adcen().disabled());
    }
}
