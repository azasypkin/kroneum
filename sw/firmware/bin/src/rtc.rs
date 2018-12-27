use cortex_m::Peripherals as CorePeripherals;
use stm32f0x2::Interrupt;
use stm32f0x2::Peripherals;

#[derive(Debug)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Time {
    pub fn add_seconds(&mut self, seconds: u8) {
        self.seconds += seconds;

        if self.seconds >= 60 {
            self.seconds -= 60;
            self.add_minutes(1);
        }
    }

    pub fn add_minutes(&mut self, minutes: u8) {
        self.minutes += minutes;

        if self.minutes >= 60 {
            self.minutes -= 60;
            self.add_hours(1);
        }
    }

    pub fn add_hours(&mut self, hours: u8) {
        self.hours += hours;

        if self.hours >= 24 {
            self.hours -= 24;
        }
    }
}

pub struct RTC<'a> {
    core_peripherals: &'a mut CorePeripherals,
    peripherals: &'a Peripherals,
}

impl<'a> RTC<'a> {
    fn new(core_peripherals: &'a mut CorePeripherals, peripherals: &'a Peripherals) -> RTC<'a> {
        RTC {
            core_peripherals,
            peripherals,
        }
    }

    pub fn configure(core_peripherals: &mut CorePeripherals, peripherals: &Peripherals) {
        // Enable the peripheral clock RTC.
        Self::configure_clock(peripherals);
        Self::configure_interrupts(core_peripherals, peripherals);
    }

    fn configure_clock(peripherals: &Peripherals) {
        // Enable the LSI.
        peripherals.RCC.csr.modify(|_, w| w.lsion().set_bit());

        // Wait while it is not ready.
        while peripherals.RCC.csr.read().lsirdy().bit_is_clear() {}

        // Enable PWR clock.
        peripherals.RCC.apb1enr.modify(|_, w| w.pwren().set_bit());

        // Enable write in RTC domain control register.
        peripherals.PWR.cr.modify(|_, w| w.dbp().set_bit());

        // LSI for RTC clock.
        peripherals.RCC.bdcr.modify(|_, w| {
            w.rtcen().set_bit();
            unsafe { w.rtcsel().bits(0b10) }
        });

        // Disable PWR clock.
        peripherals.RCC.apb1enr.modify(|_, w| w.pwren().clear_bit());
    }

    fn configure_interrupts(core_peripherals: &mut CorePeripherals, peripherals: &Peripherals) {
        // Unmask line 17, EXTI line 17 is connected to the RTC Alarm event.
        peripherals.EXTI.imr.modify(|_, w| w.mr17().set_bit());
        // Rising edge for line 17.
        peripherals.EXTI.rtsr.modify(|_, w| w.tr17().set_bit());
        // Set priority.
        unsafe {
            core_peripherals.NVIC.set_priority(Interrupt::RTC, 2);
        }
    }

    pub fn acquire<'b, F, R>(
        core_peripherals: &'b mut CorePeripherals,
        peripherals: &'b Peripherals,
        f: F,
    ) -> R
    where
        F: FnOnce(RTC) -> R,
    {
        f(RTC::new(core_peripherals, peripherals))
    }

    pub fn clear_pending_interrupt(&self) {
        // Clear Alarm A flag.
        self.peripherals
            .RTC
            .isr
            .modify(|_, w| w.alraf().clear_bit());

        // Clear exti line 17 flag.
        self.peripherals.EXTI.pr.modify(|_, w| w.pif17().set_bit());
    }

    pub fn configure_alarm(&mut self, time: &Time) {
        // Disable the write protection for RTC registers.
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

        // Disable alarm A to modify it.
        self.toggle_alarm(false);

        // Wait until it is allowed to modify alarm A value.
        while self.peripherals.RTC.isr.read().alrawf().bit_is_clear() {}

        // Modify alarm A mask to have an interrupt each minute.
        self.peripherals.RTC.alrmar.modify(|_, w| {
            unsafe {
                w.ht().bits(time.hours / 10);
                w.hu().bits(time.hours % 10);
                w.mnt().bits(time.minutes / 10);
                w.mnu().bits(time.minutes % 10);
                w.st().bits(time.seconds / 10);
                w.su().bits(time.seconds % 10);
            }

            w.msk1().clear_bit();
            w.msk2().set_bit();
            w.msk3().set_bit();
            w.msk4().set_bit()
        });

        // Enable alarm A and alarm A interrupt.
        self.toggle_alarm(true);

        // Enable the write protection for RTC registers.
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xFE) });
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x64) });
    }

    pub fn toggle_alarm(&mut self, enable: bool) {
        self.peripherals.RTC.cr.modify(|_, w| {
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
            self.core_peripherals.NVIC.enable(Interrupt::RTC);
        } else {
            self.core_peripherals.NVIC.disable(Interrupt::RTC);
        }
    }

    pub fn configure_time(&self, time: &Time) {
        // Disable the write protection for RTC registers.
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

        // Enable init phase.
        self.peripherals.RTC.isr.modify(|_, w| w.init().set_bit());

        // Wait until it is allowed to modify RTC register values.
        while self.peripherals.RTC.isr.read().initf().bit_is_clear() {}

        // Set prescaler, 40kHz/128 (0x7F + 1) => 312 Hz, 312Hz/312 (0x137 + 1) => 1Hz.
        self.peripherals
            .RTC
            .prer
            .modify(|_, w| unsafe { w.prediv_s().bits(0x137) });

        self.peripherals
            .RTC
            .prer
            .modify(|_, w| unsafe { w.prediv_a().bits(0x7F) });

        // Configure Time register.
        self.peripherals.RTC.tr.modify(|_, w| {
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
        self.peripherals.RTC.isr.modify(|_, w| w.init().clear_bit());

        // Enable the write protection for RTC registers.
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xFE) });
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x64) });
    }

    pub fn get_time(&self) -> Time {
        let tr = self.peripherals.RTC.tr.read();

        Time {
            hours: tr.ht().bits() * 10 + tr.hu().bits(),
            minutes: tr.mnt().bits() * 10 + tr.mnu().bits(),
            seconds: tr.st().bits() * 10 + tr.su().bits(),
        }
    }
}
