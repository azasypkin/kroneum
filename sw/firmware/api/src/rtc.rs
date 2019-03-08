use crate::time::{BCDTime, Time};

/// Describes the RTC hardware management interface.
pub trait RTCHardware {
    /// Retrieves current RTC time in BCD format.
    fn get_time(&self) -> BCDTime;

    /// Retrieves current RTC Alarm time in BCD format.
    fn get_alarm(&self) -> BCDTime;

    /// Sets RTC current time in BCD format.
    fn set_time(&self, bcd_time: BCDTime);

    /// Sets RTC Alarm in BCD format.
    fn set_alarm(&mut self, bcd_time: BCDTime);
}

pub struct RTC<T: RTCHardware> {
    hw: T,
}

impl<T: RTCHardware> RTC<T> {
    pub fn create(hw: T) -> Self {
        RTC { hw }
    }

    pub fn get_alarm(&self) -> Time {
        Time::from(&self.hw.get_alarm())
    }

    pub fn get_time(&self) -> Time {
        Time::from(&self.hw.get_alarm())
    }

    pub fn set_alarm(&mut self, time: &Time) {
        self.hw.set_alarm(time.into());
    }

    pub fn set_time(&self, time: &Time) {
        self.hw.set_time(time.into());
    }
}
