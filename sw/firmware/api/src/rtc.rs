use time::{BCDTime, Time};

/// Describes the RTC hardware management interface.
pub trait RTCHardware {
    /// Initializes hardware if needed.
    fn setup(&self);

    /// Releases hardware if needed.
    fn teardown(&self);

    /// Retrieves current RTC time in BCD format.
    fn get_time(&self) -> BCDTime;

    /// Retrieves current RTC Alarm time in BCD format.
    fn get_alarm(&self) -> BCDTime;

    /// Sets RTC current time in BCD format.
    fn set_time(&self, bcd_time: BCDTime);

    /// Sets RTC Alarm in BCD format.
    fn set_alarm(&self, bcd_time: BCDTime);
}

pub struct RTC<'a, T: RTCHardware> {
    hw: &'a T,
}

impl<'a, T: RTCHardware> RTC<'a, T> {
    pub fn new(hw: &'a T) -> Self {
        RTC { hw }
    }

    /// Setups RTC hardware.
    pub fn setup(&self) {
        self.hw.setup()
    }

    /// Tears down RTC hardware.
    pub fn teardown(&self) {
        self.hw.teardown()
    }

    pub fn alarm(&self) -> Time {
        Time::from(self.hw.get_alarm())
    }

    pub fn time(&self) -> Time {
        Time::from(self.hw.get_time())
    }

    pub fn set_alarm(&self, time: Time) {
        self.hw.set_alarm(time.into());
    }

    pub fn set_time(&self, time: Time) {
        self.hw.set_time(time.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::MockData;
    use core::cell::RefCell;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    enum Call {
        Setup,
        Teardown,
    }

    #[derive(Default)]
    pub(crate) struct AssociatedData {
        pub time: BCDTime,
        pub alarm: BCDTime,
    }

    struct RTCHardwareMock<'a> {
        data: RefCell<MockData<'a, Call, AssociatedData>>,
    }

    impl<'a> RTCHardware for RTCHardwareMock<'a> {
        fn setup(&self) {
            self.data.borrow_mut().calls.log_call(Call::Setup);
        }

        fn teardown(&self) {
            self.data.borrow_mut().calls.log_call(Call::Teardown);
        }

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
            self.data.borrow_mut().data.time = bcd_time;
        }

        fn set_alarm(&self, bcd_time: BCDTime) {
            self.data.borrow_mut().data.alarm = bcd_time;
        }
    }

    #[test]
    fn setup() {
        let rtc_hw_mock = RTCHardwareMock {
            data: RefCell::new(MockData::new(AssociatedData::default())),
        };

        RTC::new(&rtc_hw_mock).setup();

        assert_eq!(rtc_hw_mock.data.borrow().calls.logs(), [Some(Call::Setup)]);
    }

    #[test]
    fn teardown() {
        let rtc_hw_mock = RTCHardwareMock {
            data: RefCell::new(MockData::new(AssociatedData::default())),
        };

        RTC::new(&rtc_hw_mock).teardown();

        assert_eq!(
            rtc_hw_mock.data.borrow().calls.logs(),
            [Some(Call::Teardown)]
        );
    }

    #[test]
    fn get_time() {
        let rtc_hw_mock = RTCHardwareMock {
            data: RefCell::new(MockData::new(AssociatedData::default())),
        };

        assert_eq!(
            RTC::new(&rtc_hw_mock).time(),
            Time {
                hours: 13,
                minutes: 34,
                seconds: 51,
            }
        );
    }

    #[test]
    fn get_alarm() {
        let rtc_hw_mock = RTCHardwareMock {
            data: RefCell::new(MockData::new(AssociatedData::default())),
        };

        assert_eq!(
            RTC::new(&rtc_hw_mock).alarm(),
            Time {
                hours: 13,
                minutes: 35,
                seconds: 55,
            }
        );
    }

    #[test]
    fn set_time() {
        let rtc_hw_mock = RTCHardwareMock {
            data: RefCell::new(MockData::new(AssociatedData::default())),
        };

        RTC::new(&rtc_hw_mock).set_time(Time {
            hours: 13,
            minutes: 34,
            seconds: 51,
        });

        assert_eq!(
            rtc_hw_mock.data.borrow().data.time,
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
        let rtc_hw_mock = RTCHardwareMock {
            data: RefCell::new(MockData::new(AssociatedData::default())),
        };

        RTC::new(&rtc_hw_mock).set_alarm(Time {
            hours: 13,
            minutes: 35,
            seconds: 55,
        });

        assert_eq!(
            rtc_hw_mock.data.borrow().data.alarm,
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
