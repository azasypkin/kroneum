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

pub struct RTC<T: RTCHardware> {
    hw: T,
}

impl<T: RTCHardware> RTC<T> {
    pub fn new(hw: T) -> Self {
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

    struct RTCHardwareMock<'a, 'b: 'a> {
        data: RefCell<&'a mut MockData<'b, Call, AssociatedData>>,
    }

    impl<'a, 'b: 'a> RTCHardware for RTCHardwareMock<'a, 'b> {
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

    fn create_rtc<'a, 'b: 'a>(
        mock_data: &'a mut MockData<'b, Call, AssociatedData>,
    ) -> RTC<RTCHardwareMock<'a, 'b>> {
        RTC::new(RTCHardwareMock {
            data: RefCell::new(mock_data),
        })
    }

    #[test]
    fn setup() {
        let mut mock_data = MockData::new(AssociatedData::default());

        create_rtc(&mut mock_data).setup();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Setup)])
    }

    #[test]
    fn teardown() {
        let mut mock_data = MockData::new(AssociatedData::default());

        create_rtc(&mut mock_data).teardown();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Teardown)])
    }

    #[test]
    fn get_time() {
        let mut mock_data = MockData::new(AssociatedData::default());

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
        let mut mock_data = MockData::new(AssociatedData::default());

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
        let mut mock_data = MockData::new(AssociatedData::default());

        create_rtc(&mut mock_data).set_time(Time {
            hours: 13,
            minutes: 34,
            seconds: 51,
        });

        assert_eq!(
            mock_data.data.time,
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
        let mut mock_data = MockData::new(AssociatedData::default());

        create_rtc(&mut mock_data).set_alarm(Time {
            hours: 13,
            minutes: 35,
            seconds: 55,
        });

        assert_eq!(
            mock_data.data.alarm,
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
