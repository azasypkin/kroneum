use crate::{
    beeper::{Melody, PWMBeeper, PWMBeeperHardware},
    buttons::{ButtonPressType, Buttons, ButtonsHardware},
    flash::{Flash, FlashHardware},
    rtc::{RTCHardware, RTC},
    time::Time,
    usb::{command_packet::CommandPacket, USBHardware, USB},
};
use systick::{SysTick, SysTickHardware};
use usb::UsbState;

#[derive(Debug, Copy, Clone)]
enum SystemMode {
    Idle,
    Setup(u32),
    Alarm(Time, Melody),
    Config,
}

#[derive(Copy, Clone)]
pub struct SystemState {
    mode: SystemMode,
    usb_state: UsbState,
}

impl Default for SystemState {
    fn default() -> Self {
        SystemState {
            mode: SystemMode::Idle,
            usb_state: UsbState::default(),
        }
    }
}

/// Describes the SystemControl hardware management interface.
pub trait SystemHardware {
    type B: ButtonsHardware;
    type F: FlashHardware;
    type P: PWMBeeperHardware;
    type R: RTCHardware;
    type U: USBHardware;

    /// Initializes hardware if needed.
    fn setup(&self);

    /// Forces system to enter StandBy mode.
    fn enter_deep_sleep(&mut self);

    /// Forces system to exit StandBy mode.
    fn exit_deep_sleep(&mut self);

    /// Performs system software reset.
    fn reset(&mut self);

    /// Returns the `PWMBeeperHardware` used to create `PWMBeeper` component.
    fn beeper(&self) -> Self::P;

    // Returns the `ButtonsHardware` used to create `Buttons` component.
    fn buttons(&self) -> Self::B;

    /// Returns the `FlashHardware` used to create `Flash` component.
    fn flash(&self) -> Self::F;

    /// Returns the `RTCHardware` used to create `RTC` component.
    fn rtc(&self) -> Self::R;

    /// Returns the `USBHardware` used to create `USB` component.
    fn usb(&self) -> Self::U;
}

pub struct System<'a, T: SystemHardware, S: SysTickHardware> {
    hw: T,
    state: &'a mut SystemState,
    systick: SysTick<S>,
}

impl<'a, T: SystemHardware, S: SysTickHardware> System<'a, T, S> {
    pub fn new(hw: T, state: &'a mut SystemState, systick: SysTick<S>) -> Self {
        System { state, hw, systick }
    }

    /// Setups system.
    pub fn setup(&mut self) {
        self.hw.setup();

        self.buttons().setup();

        self.set_mode(SystemMode::Idle);
    }

    pub fn handle_alarm(&mut self) {
        if let SystemMode::Alarm(_, melody) = self.state.mode {
            self.beeper().play(melody);
            self.beeper().play(melody);

            self.rtc().teardown();

            // Snooze alarm for 10 seconds.
            self.set_mode(SystemMode::Alarm(Time::from_seconds(10), Melody::Beep));
        }
    }

    pub fn handle_button_press(&mut self) {
        if !self.buttons().triggered() {
            return;
        }

        let (button_i, button_x) = self.buttons().interrupt();

        match (self.state.mode, button_i, button_x) {
            (mode, ButtonPressType::Long, ButtonPressType::Long) => {
                let (button_i, button_x) = self.buttons().interrupt();

                match (mode, button_i, button_x) {
                    (SystemMode::Config, ButtonPressType::Long, ButtonPressType::Long)
                    | (SystemMode::Alarm(_, _), ButtonPressType::Long, ButtonPressType::Long) => {
                        self.set_mode(SystemMode::Idle)
                    }
                    (_, ButtonPressType::Long, ButtonPressType::Long) => {
                        self.set_mode(SystemMode::Config)
                    }
                    (SystemMode::Setup(counter), _, _) => {
                        self.set_mode(SystemMode::Alarm(Time::from_hours(counter), Melody::Alarm))
                    }
                    _ => {}
                }
            }
            (SystemMode::Idle, ButtonPressType::Long, _)
            | (SystemMode::Idle, _, ButtonPressType::Long) => {
                self.set_mode(SystemMode::Setup(0));
            }
            (SystemMode::Alarm(_, _), ButtonPressType::Long, _)
            | (SystemMode::Alarm(_, _), _, ButtonPressType::Long) => {
                self.set_mode(SystemMode::Idle);
            }
            (SystemMode::Setup(counter), ButtonPressType::Long, _)
            | (SystemMode::Setup(counter), _, ButtonPressType::Long) => {
                let time = match button_i {
                    ButtonPressType::Long => Time::from_seconds(counter as u32),
                    _ => Time::from_minutes(counter as u32),
                };

                self.set_mode(SystemMode::Alarm(time, Melody::Alarm));
            }
            (SystemMode::Setup(counter), ButtonPressType::Short, _) => {
                self.set_mode(SystemMode::Setup(counter + 1))
            }
            (SystemMode::Setup(counter), _, ButtonPressType::Short) => {
                self.set_mode(SystemMode::Setup(counter + 10))
            }
            _ => {}
        }

        self.buttons().reactivate();
    }

    pub fn handle_usb_packet(&mut self) {
        self.usb().interrupt();

        if let Some(command_packet) = self.state.usb_state.command {
            if let CommandPacket::Beep(num) = command_packet {
                self.beeper().beep_n(num);
            } else if let CommandPacket::Melody(tones) = command_packet {
                self.beeper().play(Melody::Custom(tones));
            } else if let CommandPacket::AlarmSet(time) = command_packet {
                self.set_mode(SystemMode::Alarm(time, Melody::Alarm));
            } else if let CommandPacket::AlarmGet = command_packet {
                let alarm = self.rtc().alarm();
                self.usb()
                    .send(&[alarm.hours, alarm.minutes, alarm.seconds]);
            } else if let CommandPacket::Reset = command_packet {
                self.reset();
            } else if let CommandPacket::FlashRead(slot) = command_packet {
                let value = self.flash().read(slot).unwrap_or_else(|| 0);
                self.usb().send(&[value]);
            } else if let CommandPacket::FlashWrite(slot, value) = command_packet {
                let status = if self.flash().write(slot, value).is_ok() {
                    1
                } else {
                    0
                };
                self.usb().send(&[status]);
            } else if let CommandPacket::FlashEraseAll = command_packet {
                self.flash().erase_all();
            } else if let CommandPacket::Echo(array) = command_packet {
                self.usb().send(array.as_ref());
            }
        }

        self.state.usb_state.command = None;
    }

    /// Performs system software reset.
    fn reset(&mut self) {
        self.hw.reset();
    }

    /// Creates an instance of `RTC` controller.
    fn rtc(&mut self) -> RTC<T::R> {
        RTC::new(self.hw.rtc())
    }

    /// Creates an instance of `Beeper` controller.
    fn beeper(&mut self) -> PWMBeeper<T::P, S> {
        PWMBeeper::new(self.hw.beeper(), &mut self.systick)
    }

    /// Creates an instance of `Buttons` controller.
    fn buttons(&mut self) -> Buttons<T::B, S> {
        Buttons::new(self.hw.buttons(), &mut self.systick)
    }

    /// Creates an instance of `Flash` controller.
    fn flash(&mut self) -> Flash<T::F> {
        Flash::new(self.hw.flash())
    }

    /// Creates an instance of `USB` controller.
    fn usb(&mut self) -> USB<T::U> {
        USB::new(self.hw.usb(), &mut self.state.usb_state)
    }

    /// Switches system to a new mode.
    fn set_mode(&mut self, mode: SystemMode) {
        match &mode {
            SystemMode::Idle => {
                self.usb().teardown();
                self.rtc().teardown();

                // If we are exiting `Config` or `Alarm` mode let's play special signal.
                if let SystemMode::Setup(_) = self.state.mode {
                    self.beeper().play(Melody::Reset);
                } else if let SystemMode::Alarm(_, _) = self.state.mode {
                    self.beeper().play(Melody::Reset);
                } else if let SystemMode::Config = self.state.mode {
                    self.beeper().play(Melody::Reset);
                }

                self.hw.enter_deep_sleep();
            }
            SystemMode::Config => {
                self.hw.exit_deep_sleep();

                self.beeper().play(Melody::Reset);

                self.usb().setup();
            }
            SystemMode::Setup(0) => self.beeper().play(Melody::Setup),
            SystemMode::Setup(c) if *c > 0 => self.beeper().beep(),
            SystemMode::Alarm(time, _) => {
                self.beeper().play(Melody::Setup);

                let rtc = self.rtc();
                rtc.setup();
                rtc.set_time(Time::default());
                rtc.set_alarm(*time);
            }
            _ => {}
        }

        self.state.mode = mode;
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
