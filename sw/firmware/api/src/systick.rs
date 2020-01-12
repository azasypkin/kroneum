use config;

/// Describes the SysTick hardware management interface.
pub trait SysTickHardware {
    fn configure(&mut self, reload_value: u32);
    fn enable_counter(&mut self);
    fn disable_counter(&mut self);
    fn has_wrapped(&mut self) -> bool;
    fn enable_interrupt(&mut self);
    fn disable_interrupt(&mut self);
}

pub struct SysTick<T: SysTickHardware> {
    hw: T,
}

impl<T: SysTickHardware> SysTick<T> {
    pub fn new(hw: T) -> Self {
        SysTick { hw }
    }

    /// Blocks execution for the specified number of milliseconds.
    pub fn delay(&mut self, ms: u32) {
        self.hw.configure(Self::get_reload_value(ms));
        self.hw.enable_counter();

        while !self.hw.has_wrapped() {}

        self.hw.disable_counter();
    }

    /// Starts asynchronous counter that will trigger SysTick interrupt whenever it's ready.
    pub fn start(&mut self, ms: u32) {
        self.hw.configure(Self::get_reload_value(ms));
        self.hw.enable_interrupt();
        self.hw.enable_counter();
    }

    /// Stops asynchronous counting.
    pub fn stop(&mut self) {
        self.hw.disable_interrupt();
        self.hw.disable_counter();
    }

    fn get_reload_value(ms: u32) -> u32 {
        let us = ms * 1000;

        let rvr = us * (config::CLOCK_SPEED / 1_000_000);
        assert!(rvr < (1 << 24), "timeout is too large");

        rvr
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests::MockData;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub(crate) enum Call {
        Delay(u32),
        EnableInterrupt,
        DisableInterrupt,
        EnableCounter,
        DisableCounter,
    }

    #[derive(Default)]
    pub(crate) struct AssociatedData {
        pub reload_value: u32,
        pub ticks: u32,
    }

    pub(crate) struct SystickHardwareMock<'a, 'b: 'a> {
        mock: &'a mut MockData<'b, Call, AssociatedData>,
    }

    impl<'a, 'b: 'a> SysTickHardware for SystickHardwareMock<'a, 'b> {
        fn configure(&mut self, reload_value: u32) {
            self.mock.data.reload_value = reload_value;

            let ticks = reload_value / (config::CLOCK_SPEED / 1000);
            self.mock.calls.log_call(Call::Delay(ticks));
        }

        fn enable_counter(&mut self) {
            self.mock.data.ticks = 0;
            self.mock.calls.log_call(Call::EnableCounter);
        }

        fn disable_counter(&mut self) {
            self.mock.calls.log_call(Call::DisableCounter);
        }

        fn has_wrapped(&mut self) -> bool {
            self.mock.data.ticks += 1;
            self.mock.data.ticks == 5
        }

        fn enable_interrupt(&mut self) {
            self.mock.calls.log_call(Call::EnableInterrupt);
        }

        fn disable_interrupt(&mut self) {
            self.mock.calls.log_call(Call::DisableInterrupt);
        }
    }

    pub(crate) fn create_systick<'a, 'b: 'a>(
        systick_mock: &'a mut MockData<'b, Call, AssociatedData>,
    ) -> SysTick<SystickHardwareMock<'a, 'b>> {
        SysTick {
            hw: SystickHardwareMock { mock: systick_mock },
        }
    }

    #[test]
    #[should_panic(expected = "timeout is too large")]
    fn delay_fails_for_large_timeout() {
        let mut systick_mock = MockData::new(AssociatedData::default());
        create_systick(&mut systick_mock).delay(5000);
    }

    #[test]
    fn delay_handles_timeout() {
        let mut systick_mock = MockData::new(AssociatedData::default());

        create_systick(&mut systick_mock).delay(1234);
        assert_eq!(systick_mock.data.reload_value, 9872000);
        assert_eq!(systick_mock.data.ticks, 5);

        create_systick(&mut systick_mock).delay(1);
        assert_eq!(systick_mock.data.reload_value, 8000);
        assert_eq!(systick_mock.data.ticks, 5);
    }

    #[test]
    #[should_panic(expected = "timeout is too large")]
    fn start_fails_for_large_timeout() {
        let mut systick_mock = MockData::new(AssociatedData::default());
        create_systick(&mut systick_mock).start(5000);
    }

    #[test]
    fn start_runs_counter() {
        let mut systick_mock = MockData::new(AssociatedData::default());

        create_systick(&mut systick_mock).start(1234);
        assert_eq!(systick_mock.data.reload_value, 9872000);
        assert_eq!(systick_mock.data.ticks, 0);

        assert_eq!(
            systick_mock.calls.logs(),
            [
                Some(Call::Delay(1234)),
                Some(Call::EnableInterrupt),
                Some(Call::EnableCounter)
            ]
        );
    }

    #[test]
    fn stop_stops_counter() {
        let mut systick_mock = MockData::new(AssociatedData::default());

        create_systick(&mut systick_mock).stop();

        assert_eq!(
            systick_mock.calls.logs(),
            [Some(Call::DisableInterrupt), Some(Call::DisableCounter)]
        );
    }
}
