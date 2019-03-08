use crate::Peripherals;
use kroneum_api::time::Time;
use stm32f0x2::Interrupt;

pub struct RTC<'a> {
    p: &'a mut Peripherals,
}

impl<'a> RTC<'a> {
    fn new(p: &'a mut Peripherals) -> Self {
        RTC { p }
    }

    pub fn setup(&mut self) {
        // Enable the LSI.
        self.p.device.RCC.csr.modify(|_, w| w.lsion().set_bit());
        while self.p.device.RCC.csr.read().lsirdy().bit_is_clear() {}

        // Enable PWR clock.
        self.p.device.RCC.apb1enr.modify(|_, w| w.pwren().set_bit());

        // Enable write in RTC domain control register.
        self.p.device.PWR.cr.modify(|_, w| w.dbp().set_bit());

        // LSI for RTC clock.
        self.p
            .device
            .RCC
            .bdcr
            .modify(|_, w| unsafe { w.rtcen().set_bit().rtcsel().bits(0b10) });

        // Disable PWR clock.
        self.p
            .device
            .RCC
            .apb1enr
            .modify(|_, w| w.pwren().clear_bit());

        // Unmask line 17, EXTI line 17 is connected to the RTC Alarm event.
        self.p.device.EXTI.imr.modify(|_, w| w.mr17().set_bit());
        // Rising edge for line 17.
        self.p.device.EXTI.rtsr.modify(|_, w| w.tr17().set_bit());
        // Set priority.
        unsafe {
            self.p.core.NVIC.set_priority(Interrupt::RTC, 2);
        }
    }

    pub fn teardown(&mut self) {
        self.toggle_alarm(false);

        // Disable the LSI.
        self.p.device.RCC.csr.modify(|_, w| w.lsion().clear_bit());
        while self.p.device.RCC.csr.read().lsirdy().bit_is_set() {}

        self.clear_pending_interrupt();
    }

    pub fn acquire<'b, F, R>(p: &'a mut Peripherals, f: F) -> R
    where
        F: FnOnce(RTC) -> R,
    {
        f(RTC::new(p))
    }

    pub fn clear_pending_interrupt(&self) {
        // Clear Alarm A flag.
        self.p.device.RTC.isr.modify(|_, w| w.alraf().clear_bit());

        // Clear exti line 17 flag.
        self.p.device.EXTI.pr.modify(|_, w| w.pif17().set_bit());
    }

    pub fn configure_alarm(&mut self, time: &Time) {
        // Disable the write protection for RTC registers.
        self.p.device.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
        self.p.device.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

        // Disable alarm A to modify it.
        self.toggle_alarm(false);

        // Wait until it is allowed to modify alarm A value.
        while self.p.device.RTC.isr.read().alrawf().bit_is_clear() {}

        self.p.device.RTC.alrmar.modify(|_, w| {
            unsafe {
                w.ht().bits(time.hours / 10);
                w.hu().bits(time.hours % 10);
                w.mnt().bits(time.minutes / 10);
                w.mnu().bits(time.minutes % 10);
                w.st().bits(time.seconds / 10);
                w.su().bits(time.seconds % 10);
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
        self.toggle_alarm(true);

        // Enable the write protection for RTC registers.
        self.p.device.RTC.wpr.write(|w| unsafe { w.bits(0xFE) });
        self.p.device.RTC.wpr.write(|w| unsafe { w.bits(0x64) });
    }

    pub fn toggle_alarm(&mut self, enable: bool) {
        self.p.device.RTC.cr.modify(|_, w| {
            if enable {
                w.alraie().set_bit();
                w.alrae().set_bit()
            } else {
                w.alraie().clear_bit();
                w.alrae().clear_bit()
            }
        });

        // Enable/disable RTC_IRQn in the NVIC.
        if enable {
            self.p.core.NVIC.enable(Interrupt::RTC);
        } else {
            self.p.core.NVIC.disable(Interrupt::RTC);
        }
    }

    pub fn configure_time(&self, time: &Time) {
        // Disable the write protection for RTC registers.
        self.p.device.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
        self.p.device.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

        // Enable init phase.
        self.p.device.RTC.isr.modify(|_, w| w.init().set_bit());

        // Wait until it is allowed to modify RTC register values.
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
        self.p.device.RTC.tr.modify(|_, w| {
            unsafe {
                w.ht().bits(time.hours / 10);
                w.hu().bits(time.hours % 10);
                w.mnt().bits(time.minutes / 10);
                w.mnu().bits(time.minutes % 10);
                w.st().bits(time.seconds / 10);
                w.su().bits(time.seconds % 10);
            }

            w
        });

        // Disable init phase.
        self.p.device.RTC.isr.modify(|_, w| w.init().clear_bit());

        // Enable the write protection for RTC registers.
        self.p.device.RTC.wpr.write(|w| unsafe { w.bits(0xFE) });
        self.p.device.RTC.wpr.write(|w| unsafe { w.bits(0x64) });
    }

    pub fn get_alarm(&self) -> Time {
        let reg = self.p.device.RTC.alrmar.read();

        Time {
            hours: reg.ht().bits() * 10 + reg.hu().bits(),
            minutes: reg.mnt().bits() * 10 + reg.mnu().bits(),
            seconds: reg.st().bits() * 10 + reg.su().bits(),
        }
    }

    pub fn get_time(&self) -> Time {
        let reg = self.p.device.RTC.tr.read();

        Time {
            hours: reg.ht().bits() * 10 + reg.hu().bits(),
            minutes: reg.mnt().bits() * 10 + reg.mnu().bits(),
            seconds: reg.st().bits() * 10 + reg.su().bits(),
        }
    }
}
