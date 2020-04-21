mod system_hardware;
mod system_info;
mod system_role;
mod system_state;

use adc::ADC;
use bare_metal::CriticalSection;
use beeper::PWMBeeper;
use buttons::{Buttons, ButtonsPoll};
use flash::{storage_slot::StorageSlot, Flash};
use radio::Radio;
use rtc::RTC;
use systick::{SysTick, SysTickHardware};
use timer::Timer;
use usb::USB;

pub use self::{system_hardware::SystemHardware, system_info::SystemInfo};
use self::{
    system_role::{ControllerSystemRoleHandler, SystemRole, TimerSystemRoleHandler},
    system_state::SystemState,
};

pub struct System<T: SystemHardware, S: SysTickHardware> {
    hw: T,
    state: SystemState,
    systick: SysTick<S>,
}

impl<T: SystemHardware, S: SysTickHardware> System<T, S> {
    pub fn run(hw: T, systick: SysTick<S>) -> Self {
        let mut system = System {
            state: SystemState::default(),
            hw,
            systick,
        };

        system.switch_to_role(
            system
                .flash()
                .read(StorageSlot::Configuration)
                .map_or_else(SystemRole::default, SystemRole::from),
        );

        system
    }

    pub fn handle_alarm(&mut self) {
        if let SystemRole::Timer = self.state.role {
            TimerSystemRoleHandler::on_alarm(self);
        }
    }

    pub fn handle_button_press(&mut self) {
        // If buttons weren't activated, don't do anything.
        let buttons = self.buttons();
        if !buttons.triggered() {
            return;
        }

        // If buttons are in the middle of the polling, reactivate them and let current polling complete.
        if buttons.is_polling() {
            buttons.reactivate();
            return;
        }

        self.poll_buttons();
    }

    pub fn handle_usb_packet(&mut self, cs: &CriticalSection) {
        if let SystemRole::Controller = self.state.role {
            ControllerSystemRoleHandler::on_usb_packet(self, cs);
        }
    }

    /// Handles SysTick event and stops the counter.
    pub fn handle_systick(&mut self) {
        self.systick.stop();

        let mut beeper = self.beeper();
        if beeper.is_playing() {
            beeper.resume();
        }
    }

    /// Handles timer event: stops the timer and pols the buttons.
    pub fn handle_timer(&mut self) {
        self.timer().stop();

        let buttons = self.buttons();
        if buttons.is_polling() {
            self.poll_buttons();
        }
    }

    /// Depending on the current mode and number of active asynchronous tasks system either enters
    /// deep sleep mode or exit from it. E.g. if we have timer based tasks left we should exit deep
    /// sleep to enable timers and enter it as soon as all tasks are completed.
    pub fn sleep(&mut self) {
        match (
            self.state.role,
            self.beeper().is_playing() || self.buttons().is_polling(),
        ) {
            (SystemRole::Controller, _) | (_, true) => self.hw.exit_deep_sleep(),
            _ => self.hw.enter_deep_sleep(),
        }
    }

    /// Performs system software reset.
    fn reset(&mut self) {
        self.hw.reset();
    }

    fn poll_buttons(&mut self) {
        match self.buttons().poll() {
            ButtonsPoll::Ready((button_i, button_x, _)) => {
                match self.state.role {
                    SystemRole::Timer => {
                        TimerSystemRoleHandler::on_buttons_press(self, (button_i, button_x))
                    }
                    SystemRole::Controller => {
                        ControllerSystemRoleHandler::on_buttons_press(self, (button_i, button_x))
                    }
                };
            }
            ButtonsPoll::Pending(pending_time) => self.timer().start(pending_time),
        }

        self.buttons().reactivate();
    }

    /// Switches system to a new role.
    fn switch_to_role(&mut self, role: SystemRole) {
        self.state.role_state = None;
        self.state.role = role;

        match self.state.role {
            SystemRole::Timer => self.usb().teardown(),
            SystemRole::Controller => self.usb().setup(),
        };
    }

    /// Creates an instance of `ADC` controller.
    fn adc(&self) -> ADC<T> {
        ADC::new(&self.hw)
    }

    /// Creates an instance of `RTC` controller.
    fn rtc(&self) -> RTC<T> {
        RTC::new(&self.hw)
    }

    /// Creates an instance of `ADC` controller.
    fn radio(&mut self) -> Radio<T, S> {
        Radio::new(&mut self.hw, &mut self.systick)
    }

    fn timer(&self) -> Timer<T> {
        Timer::new(&self.hw)
    }

    /// Creates an instance of `Beeper` controller.
    fn beeper(&mut self) -> PWMBeeper<T, S> {
        PWMBeeper::new(
            &self.hw,
            &mut self.systick,
            &mut self.state.peripherals_states.beeper,
        )
    }

    /// Creates an instance of `Buttons` controller.
    fn buttons(&mut self) -> Buttons<T> {
        Buttons::new(&self.hw, &mut self.state.peripherals_states.buttons)
    }

    /// Creates an instance of `Flash` controller.
    fn flash(&self) -> Flash<T> {
        Flash::new(&self.hw)
    }

    /// Creates an instance of `USB` controller.
    fn usb(&mut self) -> USB<T> {
        USB::new(&self.hw, &mut self.state.peripherals_states.usb)
    }
}

#[cfg(test)]
mod tests {
    /*use super::*;
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

    impl<'a, 'b: 'a> SystemHardware for SystemControlHardwareMock<'a, 'b> {
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
    ) -> System<SystemControlHardwareMock<'a, 'b>> {
        System::new(SystemControlHardwareMock {
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
    }*/
}
