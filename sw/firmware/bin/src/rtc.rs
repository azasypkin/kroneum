use crate::{DevicePeripherals, Peripherals};
use kroneum_api::{rtc, time::BCDTime};

pub struct RTCHardwareImpl<'a> {
    p: &'a mut Peripherals,
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
            self.p
                .device
                .RTC
                .wpr
                .write(|w| unsafe { w.key().bits(*key) });
        }
    }
}

impl<'a> rtc::RTCHardware for RTCHardwareImpl<'a> {
    fn get_time(&self) -> BCDTime {
        let reg = self.p.device.RTC.tr.read();
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
        let reg = self.p.device.RTC.alrmar.read();
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
        self.p.device.RTC.isr.modify(|_, w| w.init().set_bit());
        while self.p.device.RTC.isr.read().initf().bit_is_clear() {}

        // Set prescaler, 40kHz/128 (0x7F + 1) => 312 Hz, 312Hz/312 (0x137 + 1) => 1Hz.
        self.p
            .device
            .RTC
            .prer
            .modify(|_, w| unsafe { w.prediv_s().bits(0x137) });

        self.p
            .device
            .RTC
            .prer
            .modify(|_, w| unsafe { w.prediv_a().bits(0x7F) });

        // Configure Time register.
        self.p.device.RTC.tr.modify(|_, w| unsafe {
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
        self.p.device.RTC.isr.modify(|_, w| w.init().clear_bit());

        self.toggle_write_protection(true);
    }

    fn set_alarm(&mut self, bcd_time: BCDTime) {
        self.toggle_write_protection(false);

        // Disable alarm A to modify it.
        toggle_alarm(self.p, false);

        // Wait until it is allowed to modify alarm A value.
        while self.p.device.RTC.isr.read().alrawf().bit_is_clear() {}

        self.p.device.RTC.alrmar.modify(|_, w| {
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

pub fn setup(p: &mut Peripherals) {
    // Enable the LSI.
    p.device.RCC.csr.modify(|_, w| w.lsion().set_bit());
    while p.device.RCC.csr.read().lsirdy().bit_is_clear() {}

    // Enable PWR clock.
    p.device.RCC.apb1enr.modify(|_, w| w.pwren().set_bit());

    // Enable write in RTC domain control register.
    p.device.PWR.cr.modify(|_, w| w.dbp().set_bit());

    // LSI for RTC clock.
    p.device
        .RCC
        .bdcr
        .modify(|_, w| w.rtcen().set_bit().rtcsel().bits(0b10));

    // Disable PWR clock.
    p.device.RCC.apb1enr.modify(|_, w| w.pwren().clear_bit());

    // Unmask line 17, EXTI line 17 is connected to the RTC Alarm event.
    p.device.EXTI.imr.modify(|_, w| w.mr17().set_bit());
    // Rising edge for line 17.
    p.device.EXTI.rtsr.modify(|_, w| w.tr17().set_bit());
}

pub fn teardown(p: &mut Peripherals) {
    toggle_alarm(p, false);

    // Disable the LSI.
    p.device.RCC.csr.modify(|_, w| w.lsion().clear_bit());
    while p.device.RCC.csr.read().lsirdy().bit_is_set() {}

    clear_pending_interrupt(&p.device);
}

fn clear_pending_interrupt(p: &DevicePeripherals) {
    // Clear Alarm A flag.
    p.RTC.isr.modify(|_, w| w.alraf().clear_bit());

    // Clear EXTI line 17 flag.
    p.EXTI.pr.modify(|_, w| w.pif17().set_bit());
}

fn toggle_alarm(p: &mut Peripherals, enable: bool) {
    p.device.RTC.cr.modify(|_, w| {
        if enable {
            w.alraie().set_bit();
            w.alrae().set_bit()
        } else {
            w.alraie().clear_bit();
            w.alrae().clear_bit()
        }
    });
}

pub fn acquire<F, R>(p: &mut Peripherals, f: F) -> R
where
    F: FnOnce(&mut rtc::RTC<RTCHardwareImpl>) -> R,
{
    f(&mut rtc::RTC::create(RTCHardwareImpl { p }))
}
