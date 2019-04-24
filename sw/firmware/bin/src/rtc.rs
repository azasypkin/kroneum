use kroneum_api::{rtc::RTCHardware, time::BCDTime};
use stm32f0::stm32f0x2::Peripherals;

pub struct RTCHardwareImpl<'a> {
    pub p: &'a Peripherals,
}

impl<'a> RTCHardwareImpl<'a> {
    /// Disables or enables write protection for RTC registers.
    fn toggle_write_protection(&self, enable_write_protection: bool) {
        let protection_keys: [u8; 2] = if enable_write_protection {
            [0xFE, 0x64]
        } else {
            [0xCA, 0x53]
        };

        for key in &protection_keys {
            self.p.RTC.wpr.write(|w| unsafe { w.key().bits(*key) });
        }
    }
}

impl<'a> RTCHardware for RTCHardwareImpl<'a> {
    fn setup(&self) {
        // Enable the LSI.
        self.p.RCC.csr.modify(|_, w| w.lsion().set_bit());
        while self.p.RCC.csr.read().lsirdy().bit_is_clear() {}

        // Enable PWR clock.
        self.p.RCC.apb1enr.modify(|_, w| w.pwren().set_bit());

        // Enable write in RTC domain control register.
        self.p.PWR.cr.modify(|_, w| w.dbp().set_bit());

        // LSI for RTC clock.
        self.p
            .RCC
            .bdcr
            .modify(|_, w| w.rtcen().set_bit().rtcsel().bits(0b10));

        // Disable PWR clock.
        self.p.RCC.apb1enr.modify(|_, w| w.pwren().clear_bit());

        // Unmask line 17, EXTI line 17 is connected to the RTC Alarm event.
        self.p.EXTI.imr.modify(|_, w| w.mr17().set_bit());
        // Rising edge for line 17.
        self.p.EXTI.rtsr.modify(|_, w| w.tr17().set_bit());
    }

    fn teardown(&self) {
        toggle_alarm(self.p, false);

        // Disable the LSI.
        self.p.RCC.csr.modify(|_, w| w.lsion().clear_bit());
        while self.p.RCC.csr.read().lsirdy().bit_is_set() {}

        // Clear pending interrupt

        // Clear Alarm A flag.
        self.p.RTC.isr.modify(|_, w| w.alraf().clear_bit());
        // Clear EXTI line 17 flag.
        self.p.EXTI.pr.modify(|_, w| w.pif17().set_bit());
    }

    fn get_time(&self) -> BCDTime {
        let reg = self.p.RTC.tr.read();
        BCDTime {
            hours_tens: reg.ht().bits(),
            hours: reg.hu().bits(),
            minutes_tens: reg.mnt().bits(),
            minutes: reg.mnu().bits(),
            seconds_tens: reg.st().bits(),
            seconds: reg.su().bits(),
        }
    }

    fn get_alarm(&self) -> BCDTime {
        let reg = self.p.RTC.alrmar.read();
        BCDTime {
            hours_tens: reg.ht().bits(),
            hours: reg.hu().bits(),
            minutes_tens: reg.mnt().bits(),
            minutes: reg.mnu().bits(),
            seconds_tens: reg.st().bits(),
            seconds: reg.su().bits(),
        }
    }

    fn set_time(&self, bcd_time: BCDTime) {
        self.toggle_write_protection(false);

        // Enable init phase and wait until it is allowed to modify RTC register values.
        self.p.RTC.isr.modify(|_, w| w.init().set_bit());
        while self.p.RTC.isr.read().initf().bit_is_clear() {}

        // Set prescaler, 40kHz/128 (0x7F + 1) => 312 Hz, 312Hz/312 (0x137 + 1) => 1Hz.
        self.p
            .RTC
            .prer
            .modify(|_, w| unsafe { w.prediv_s().bits(0x137) });

        self.p
            .RTC
            .prer
            .modify(|_, w| unsafe { w.prediv_a().bits(0x7F) });

        // Configure Time register.
        self.p.RTC.tr.modify(|_, w| unsafe {
            w.ht()
                .bits(bcd_time.hours_tens)
                .hu()
                .bits(bcd_time.hours)
                .mnt()
                .bits(bcd_time.minutes_tens)
                .mnu()
                .bits(bcd_time.minutes)
                .st()
                .bits(bcd_time.seconds_tens)
                .su()
                .bits(bcd_time.seconds)
        });

        // Disable init phase.
        self.p.RTC.isr.modify(|_, w| w.init().clear_bit());

        self.toggle_write_protection(true);
    }

    fn set_alarm(&self, bcd_time: BCDTime) {
        self.toggle_write_protection(false);

        // Disable alarm A to modify it.
        toggle_alarm(self.p, false);

        // Wait until it is allowed to modify alarm A value.
        while self.p.RTC.isr.read().alrawf().bit_is_clear() {}

        self.p.RTC.alrmar.modify(|_, w| {
            unsafe {
                w.ht()
                    .bits(bcd_time.hours_tens)
                    .hu()
                    .bits(bcd_time.hours)
                    .mnt()
                    .bits(bcd_time.minutes_tens)
                    .mnu()
                    .bits(bcd_time.minutes)
                    .st()
                    .bits(bcd_time.seconds_tens)
                    .su()
                    .bits(bcd_time.seconds);
            }

            // Seconds, minutes and hours matter, but not day.
            w.msk1()
                .clear_bit()
                .msk2()
                .clear_bit()
                .msk3()
                .clear_bit()
                .msk4()
                .set_bit()
        });

        // Enable alarm A and alarm A interrupt.
        toggle_alarm(self.p, true);

        self.toggle_write_protection(true);
    }
}

fn toggle_alarm(p: &Peripherals, enable: bool) {
    p.RTC.cr.modify(|_, w| {
        w.alraie().bit(enable);
        w.alrae().bit(enable)
    });
}
