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

    pub fn alarm(&self) -> Time {
        Time::from(self.hw.get_alarm())
    }

    pub fn time(&self) -> Time {
        Time::from(self.hw.get_time())
    }

    pub fn set_alarm(&mut self, time: Time) {
        self.hw.set_alarm(time.into());
    }

    pub fn set_time(&self, time: Time) {
        self.hw.set_time(time.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::RefCell;

    struct MockData {
        pub time: BCDTime,
        pub alarm: BCDTime,
    }

    struct RTCHardwareMock<'a> {
        data: RefCell<&'a mut MockData>,
    }

    impl<'a> RTCHardware for RTCHardwareMock<'a> {
        fn get_time(&self) -> BCDTime {
            BCDTime {
                hours_tens: 1,
                hours: 3,
                minutes_tens: 3,
                minutes: 4,
                seconds_tens: 5,
                seconds: 1,
            }
        }

        fn get_alarm(&self) -> BCDTime {
            BCDTime {
                hours_tens: 1,
                hours: 3,
                minutes_tens: 3,
                minutes: 5,
                seconds_tens: 5,
                seconds: 5,
            }
        }

        fn set_time(&self, bcd_time: BCDTime) {
            self.data.borrow_mut().time = bcd_time;
        }

        fn set_alarm(&mut self, bcd_time: BCDTime) {
            self.data.borrow_mut().alarm = bcd_time;
        }
    }

    fn create_rtc(mock_data: &mut MockData) -> RTC<RTCHardwareMock> {
        RTC {
            hw: RTCHardwareMock {
                data: RefCell::new(mock_data),
            },
        }
    }

    #[test]
    fn get_time() {
        let mut mock_data = MockData {
            time: BCDTime::default(),
            alarm: BCDTime::default(),
        };

        assert_eq!(
            create_rtc(&mut mock_data).time(),
            Time {
                hours: 13,
                minutes: 34,
                seconds: 51,
            }
        );
    }

    #[test]
    fn get_alarm() {
        let mut mock_data = MockData {
            time: BCDTime::default(),
            alarm: BCDTime::default(),
        };

        assert_eq!(
            create_rtc(&mut mock_data).alarm(),
            Time {
                hours: 13,
                minutes: 35,
                seconds: 55,
            }
        );
    }

    #[test]
    fn set_time() {
        let mut mock_data = MockData {
            time: BCDTime::default(),
            alarm: BCDTime::default(),
        };

        create_rtc(&mut mock_data).set_time(Time {
            hours: 13,
            minutes: 34,
            seconds: 51,
        });

        assert_eq!(
            mock_data.time,
            BCDTime {
                hours_tens: 1,
                hours: 3,
                minutes_tens: 3,
                minutes: 4,
                seconds_tens: 5,
                seconds: 1,
            }
        );
    }

    #[test]
    fn set_alarm() {
        let mut mock_data = MockData {
            time: BCDTime::default(),
            alarm: BCDTime::default(),
        };

        create_rtc(&mut mock_data).set_alarm(Time {
            hours: 13,
            minutes: 35,
            seconds: 55,
        });

        assert_eq!(
            mock_data.alarm,
            BCDTime {
                hours_tens: 1,
                hours: 3,
                minutes_tens: 3,
                minutes: 5,
                seconds_tens: 5,
                seconds: 5,
            }
        );
    }
}
