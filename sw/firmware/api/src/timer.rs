/// Describes the Timer hardware management interface.
pub trait TimerHardware {
    /// Initializes hardware if needed.
    fn setup(&self, frequency_hz: u32, reload_value: u32);

    /// Releases hardware if needed.
    fn teardown(&self);
}

pub struct Timer<T: TimerHardware> {
    hw: T,
}

impl<T: TimerHardware> Timer<T> {
    pub fn new(hw: T) -> Self {
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

    pub(crate) struct TimerHardwareMock<'a, 'b: 'a> {
        data: RefCell<&'a mut MockData<'b, Call>>,
    }

    impl<'a, 'b: 'a> TimerHardware for TimerHardwareMock<'a, 'b> {
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

    pub(crate) fn create_timer<'a, 'b: 'a>(
        timer_mock: &'a mut MockData<'b, Call>,
    ) -> Timer<TimerHardwareMock<'a, 'b>> {
        Timer::new(TimerHardwareMock {
            data: RefCell::new(timer_mock),
        })
    }

    #[test]
    fn properly_handles_timeout() {
        let mut timer_mock = MockData::<Call, ()>::without_data();

        create_timer(&mut timer_mock).start(1234);
        assert_eq!(timer_mock.calls.logs(), [Some(Call::Setup((1000, 1234)))]);

        create_timer(&mut timer_mock).stop();
        assert_eq!(
            timer_mock.calls.logs(),
            [Some(Call::Setup((1000, 1234))), Some(Call::Teardown)]
        );
    }
}
