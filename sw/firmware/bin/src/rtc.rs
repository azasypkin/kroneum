use crate::hal::stm32::RTC;
use crate::system::SystemHardwareImpl;
use kroneum_api::{rtc::RTCHardware, time::BCDTime};

/// Disables or enables write protection for RTC registers.
fn toggle_write_protection(rtc: &RTC, enable_write_protection: bool) {
    let protection_keys: [u8; 2] = if enable_write_protection {
        [0xFE, 0x64]
    } else {
        [0xCA, 0x53]
    };

    for key in &protection_keys {
        rtc.wpr.write(|w| unsafe { w.key().bits(*key) });
    }
}

fn toggle_alarm(rtc: &RTC, enable: bool) {
    rtc.cr.modify(|_, w| {
        w.alraie().bit(enable);
        w.alrae().bit(enable)
    });
}

impl RTCHardware for SystemHardwareImpl {
    fn setup(&self) {
        // Enable the LSI.
        self.rcc.regs.csr.modify(|_, w| w.lsion().set_bit());
        while self.rcc.regs.csr.read().lsirdy().bit_is_clear() {}

        // Enable PWR clock.
        self.rcc.regs.apb1enr.modify(|_, w| w.pwren().set_bit());

        // Enable write in RTC domain control register.
        self.pwr.cr.modify(|_, w| w.dbp().set_bit());

        // LSI for RTC clock.
        self.rcc
            .regs
            .bdcr
            .modify(|_, w| w.rtcen().set_bit().rtcsel().bits(0b10));

        // Disable PWR clock.
        self.rcc.regs.apb1enr.modify(|_, w| w.pwren().clear_bit());

        // Unmask line 17, EXTI line 17 is connected to the RTC Alarm event.
        self.exti.imr.modify(|_, w| w.mr17().set_bit());
        // Rising edge for line 17.
        self.exti.rtsr.modify(|_, w| w.tr17().set_bit());
    }

    fn teardown(&self) {
        toggle_alarm(&self.rtc, false);

        // Disable the LSI.
        self.rcc.regs.csr.modify(|_, w| w.lsion().clear_bit());
        while self.rcc.regs.csr.read().lsirdy().bit_is_set() {}

        // Clear pending interrupt

        // Clear Alarm A flag.
        self.rtc.isr.modify(|_, w| w.alraf().clear_bit());
        // Clear EXTI line 17 flag.
        self.exti.pr.modify(|_, w| w.pif17().set_bit());
    }

    fn get_time(&self) -> BCDTime {
        let reg = self.rtc.tr.read();
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
        let reg = self.rtc.alrmar.read();
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
        toggle_write_protection(&self.rtc, false);

        // Enable init phase and wait until it is allowed to modify RTC register values.
        self.rtc.isr.modify(|_, w| w.init().set_bit());
        while self.rtc.isr.read().initf().bit_is_clear() {}

        // Set prescaler, 40kHz/128 (0x7F + 1) => 312 Hz, 312Hz/312 (0x137 + 1) => 1Hz.
        self.rtc
            .prer
            .modify(|_, w| unsafe { w.prediv_s().bits(0x137) });

        self.rtc
            .prer
            .modify(|_, w| unsafe { w.prediv_a().bits(0x7F) });

        // Configure Time register.
        self.rtc.tr.modify(|_, w| unsafe {
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
        self.rtc.isr.modify(|_, w| w.init().clear_bit());

        toggle_write_protection(&self.rtc, true);
    }

    fn set_alarm(&self, bcd_time: BCDTime) {
        toggle_write_protection(&self.rtc, false);

        // Disable alarm A to modify it.
        toggle_alarm(&self.rtc, false);

        // Wait until it is allowed to modify alarm A value.
        while self.rtc.isr.read().alrawf().bit_is_clear() {}

        self.rtc.alrmar.modify(|_, w| {
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
        toggle_alarm(&self.rtc, true);

        toggle_write_protection(&self.rtc, true);
    }
}
