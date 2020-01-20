/// Describes the Timer hardware management interface.
pub trait TimerHardware {
    /// Initializes hardware if needed.
    fn setup(&self, frequency_hz: u32, reload_value: u32);

    /// Releases hardware if needed.
    fn teardown(&self);
}

pub struct Timer<'a, T: TimerHardware> {
    hw: &'a T,
}

impl<'a, T: TimerHardware> Timer<'a, T> {
    pub fn new(hw: &'a T) -> Self {
        Timer { hw }
    }

    pub fn start(&mut self, ms: u32) {
        // 1000 is 1kHz frequency of the timer (counts every 1ms).
        self.hw.setup(1_000, ms);
    }

    pub fn stop(&mut self) {
        self.hw.teardown();
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests::MockData;
    use core::cell::RefCell;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub(crate) enum Call {
        Setup((u32, u32)),
        Teardown,
    }

    pub(crate) struct TimerHardwareMock<'a> {
        data: RefCell<MockData<'a, Call>>,
    }

    impl<'a> TimerHardware for TimerHardwareMock<'a> {
        fn setup(&self, frequency: u32, reload_value: u32) {
            self.data
                .borrow_mut()
                .calls
                .log_call(Call::Setup((frequency, reload_value)));
        }

        fn teardown(&self) {
            self.data.borrow_mut().calls.log_call(Call::Teardown);
        }
    }

    #[test]
    fn properly_handles_timeout() {
        let timer_hw_mock = TimerHardwareMock {
            data: RefCell::new(MockData::<Call, ()>::without_data()),
        };

        Timer::new(&timer_hw_mock).start(1234);
        assert_eq!(
            timer_hw_mock.data.borrow().calls.logs(),
            [Some(Call::Setup((1000, 1234)))]
        );

        Timer::new(&timer_hw_mock).stop();
        assert_eq!(
            timer_hw_mock.data.borrow().calls.logs(),
            [Some(Call::Setup((1000, 1234))), Some(Call::Teardown)]
        );
    }
}
