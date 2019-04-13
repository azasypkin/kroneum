use crate::config;

/// Describes the SysTick hardware management interface.
pub trait SysTickHardware {
    fn configure(&mut self, reload_value: u32);
    fn enable_counter(&mut self);
    fn disable_counter(&mut self);
    fn has_wrapped(&mut self) -> bool;
}

pub struct SysTick<T: SysTickHardware> {
    hw: T,
}

impl<'a, T: 'a + SysTickHardware> SysTick<T> {
    pub fn new(hw: T) -> Self {
        SysTick { hw }
    }

    pub fn delay_us(&mut self, us: u32) {
        let rvr = us * (config::CLOCK_SPEED / 1_000_000);

        assert!(rvr < (1 << 24), "timeout is too large");

        self.hw.configure(rvr);

        self.hw.enable_counter();
        while !self.hw.has_wrapped() {}
        self.hw.disable_counter();
    }

    pub fn delay_ms(&mut self, ms: u32) {
        self.delay_us(ms * 1000);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests::MockData;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    pub(crate) enum Call {
        Delay(u32),
    }

    #[derive(Default)]
    pub(crate) struct AssociatedData {
        pub reload_value: u32,
        pub ticks: u32,
    }

    pub(crate) struct SystickHardwareMock<'a> {
        mock: &'a mut MockData<Call, AssociatedData>,
    }

    impl<'a> SysTickHardware for SystickHardwareMock<'a> {
        fn configure(&mut self, reload_value: u32) {
            self.mock.data.reload_value = reload_value;
            self.mock
                .calls
                .log_call(Call::Delay(reload_value / (config::CLOCK_SPEED / 1000)));
        }

        fn enable_counter(&mut self) {
            self.mock.data.ticks = 0;
        }

        fn disable_counter(&mut self) {}

        fn has_wrapped(&mut self) -> bool {
            self.mock.data.ticks += 1;
            self.mock.data.ticks == 5
        }
    }

    pub(crate) fn create_systick(
        systick_mock: &mut MockData<Call, AssociatedData>,
    ) -> SysTick<SystickHardwareMock> {
        SysTick {
            hw: SystickHardwareMock { mock: systick_mock },
        }
    }

    #[test]
    #[should_panic(expected = "timeout is too large")]
    fn fails_for_large_timeout() {
        let mut systick_mock = MockData::new();
        create_systick(&mut systick_mock).delay_ms(5000);
    }

    #[test]
    fn handles_timeout() {
        let mut systick_mock = MockData::new();

        create_systick(&mut systick_mock).delay_ms(1234);
        assert_eq!(systick_mock.data.reload_value, 9872000);
        assert_eq!(systick_mock.data.ticks, 5);

        create_systick(&mut systick_mock).delay_us(1234);
        assert_eq!(systick_mock.data.reload_value, 9872);
        assert_eq!(systick_mock.data.ticks, 5);
    }
}
