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
    pub fn create(hw: T) -> Self {
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
mod tests {
    use super::*;

    struct MockData {
        pub reload_value: u32,
        pub ticks: u32,
    }

    impl MockData {
        pub fn new() -> Self {
            MockData {
                reload_value: 0,
                ticks: 0,
            }
        }
    }

    struct SystickHardwareMock<'a> {
        data: &'a mut MockData,
    }

    impl<'a> SysTickHardware for SystickHardwareMock<'a> {
        fn configure(&mut self, reload_value: u32) {
            self.data.reload_value = reload_value;
        }

        fn enable_counter(&mut self) {
            self.data.ticks = 0;
        }

        fn disable_counter(&mut self) {}

        fn has_wrapped(&mut self) -> bool {
            self.data.ticks += 1;
            self.data.ticks == 5
        }
    }

    fn get_systick(mock_data: &mut MockData) -> SysTick<SystickHardwareMock> {
        SysTick {
            hw: SystickHardwareMock { data: mock_data },
        }
    }

    #[test]
    #[should_panic(expected = "timeout is too large")]
    fn fails_for_large_timeout() {
        let mut mock_data = MockData::new();
        get_systick(&mut mock_data).delay_ms(5000);
    }

    #[test]
    fn handles_timeout() {
        let mut mock_data = MockData::new();

        get_systick(&mut mock_data).delay_ms(1234);
        assert_eq!(mock_data.reload_value, 9872000);
        assert_eq!(mock_data.ticks, 5);

        get_systick(&mut mock_data).delay_us(1234);
        assert_eq!(mock_data.reload_value, 9872);
        assert_eq!(mock_data.ticks, 5);
    }
}
