/// Describes the SystemControl hardware management interface.
pub trait SystemControlHardware {
    /// Forces system to enter StandBy mode.
    fn enter_standby_mode(&mut self);

    /// Forces system to exit StandBy mode.
    fn exit_standby_mode(&mut self);

    /// Performs system software reset.
    fn reset(&mut self);
}

pub struct SystemControl<T: SystemControlHardware> {
    hw: T,
}

impl<T: SystemControlHardware> SystemControl<T> {
    pub fn new(hw: T) -> Self {
        SystemControl { hw }
    }

    /// Forces system to enter StandBy mode.
    pub fn enter_standby_mode(&mut self) {
        self.hw.enter_standby_mode();
    }

    /// Forces system to exit StandBy mode.
    pub fn exit_standby_mode(&mut self) {
        self.hw.exit_standby_mode();
    }

    /// Performs system software reset.
    pub fn reset(&mut self) {
        self.hw.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::MockData;
    use core::cell::RefCell;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    enum Call {
        EnterStandByMode,
        ExitStandByMode,
        Reset,
    }

    struct SystemControlHardwareMock<'a, 'b: 'a> {
        data: RefCell<&'a mut MockData<'b, Call>>,
    }

    impl<'a, 'b: 'a> SystemControlHardware for SystemControlHardwareMock<'a, 'b> {
        fn enter_standby_mode(&mut self) {
            self.data
                .borrow_mut()
                .calls
                .log_call(Call::EnterStandByMode);
        }

        fn exit_standby_mode(&mut self) {
            self.data.borrow_mut().calls.log_call(Call::ExitStandByMode);
        }

        fn reset(&mut self) {
            self.data.borrow_mut().calls.log_call(Call::Reset);
        }
    }

    fn create_system_control<'a, 'b: 'a>(
        mock_data: &'a mut MockData<'b, Call>,
    ) -> SystemControl<SystemControlHardwareMock<'a, 'b>> {
        SystemControl::new(SystemControlHardwareMock {
            data: RefCell::new(mock_data),
        })
    }

    #[test]
    fn enter_standby_mode() {
        let mut mock_data = MockData::<Call, ()>::without_data();

        create_system_control(&mut mock_data).enter_standby_mode();

        assert_eq!(mock_data.calls.logs(), [Some(Call::EnterStandByMode)])
    }

    #[test]
    fn exit_standby_mode() {
        let mut mock_data = MockData::<Call, ()>::without_data();

        create_system_control(&mut mock_data).exit_standby_mode();

        assert_eq!(mock_data.calls.logs(), [Some(Call::ExitStandByMode)])
    }

    #[test]
    fn reset() {
        let mut mock_data = MockData::<Call, ()>::without_data();

        create_system_control(&mut mock_data).reset();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Reset)])
    }
}
