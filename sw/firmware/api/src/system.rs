use adc::{ADCHardware, ADC};
use beeper::{melody::Melody, BeeperState, PWMBeeper, PWMBeeperHardware};
use buttons::{ButtonPressType, Buttons, ButtonsHardware, ButtonsPoll, ButtonsState};
use flash::{Flash, FlashHardware};
use rtc::{RTCHardware, RTC};
use systick::{SysTick, SysTickHardware};
use time::Time;
use timer::{Timer, TimerHardware};
use usb::{command_packet::CommandPacket, USBHardware, UsbState, USB};

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
    beeper_state: BeeperState,
    buttons_state: ButtonsState,
}

impl Default for SystemState {
    fn default() -> Self {
        SystemState {
            mode: SystemMode::Idle,
            usb_state: UsbState::default(),
            beeper_state: BeeperState::default(),
            buttons_state: ButtonsState::default(),
        }
    }
}

/// Describes the SystemControl hardware management interface.
pub trait SystemHardware:
    ADCHardware
    + ButtonsHardware
    + FlashHardware
    + PWMBeeperHardware
    + RTCHardware
    + USBHardware
    + TimerHardware
{
    /// Initializes hardware if needed.
    fn setup(&mut self);

    /// Forces system to enter StandBy mode.
    fn enter_deep_sleep(&mut self);

    /// Forces system to exit StandBy mode.
    fn exit_deep_sleep(&mut self);

    /// Performs system software reset.
    fn reset(&mut self);
}

pub struct System<T: SystemHardware, S: SysTickHardware> {
    hw: T,
    state: SystemState,
    systick: SysTick<S>,
}

impl<T: SystemHardware, S: SysTickHardware> System<T, S> {
    pub fn run(mut hw: T, systick: SysTick<S>) -> Self {
        SystemHardware::setup(&mut hw);

        let mut system = System {
            state: SystemState::default(),
            hw,
            systick,
        };

        system.set_mode(SystemMode::Idle);

        system
    }

    pub fn handle_alarm(&mut self) {
        if let SystemMode::Alarm(_, melody) = self.state.mode {
            self.beeper().play_and_repeat(melody, 2);

            self.rtc().teardown();

            // Snooze alarm for 10 seconds.
            self.set_mode(SystemMode::Alarm(Time::from_seconds(10), Melody::Beep));
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

    pub fn handle_usb_packet(&mut self) {
        self.usb().interrupt();

        if let Some(command_packet) = self.state.usb_state.command {
            if let CommandPacket::Beep(num) = command_packet {
                self.beeper().play_and_repeat(Melody::Beep, num as usize);
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
            } else if let CommandPacket::ADCRead(channel) = command_packet {
                let value = self.adc().read(channel);
                self.usb()
                    .send(&[(value & 0xff) as u8, ((value & 0xff00) >> 8) as u8]);
            }
        }

        self.state.usb_state.command = None;
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
            self.state.mode,
            self.beeper().is_playing() || self.buttons().is_polling(),
        ) {
            (SystemMode::Config, _) | (_, true) => self.hw.exit_deep_sleep(),
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
                match (self.state.mode, button_i, button_x) {
                    (SystemMode::Config, ButtonPressType::Long, ButtonPressType::Long)
                    | (SystemMode::Alarm(_, _), ButtonPressType::Long, ButtonPressType::Long) => {
                        self.set_mode(SystemMode::Idle)
                    }
                    (SystemMode::Idle, ButtonPressType::Long, ButtonPressType::Long) => {
                        self.set_mode(SystemMode::Config)
                    }
                    (SystemMode::Setup(counter), ButtonPressType::Long, ButtonPressType::Long) => {
                        self.set_mode(SystemMode::Alarm(Time::from_hours(counter), Melody::Alarm))
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
            }
            ButtonsPoll::Pending(pending_time) => self.timer().start(pending_time),
        }

        self.buttons().reactivate();
    }

    /// Switches system to a new mode.
    fn set_mode(&mut self, mode: SystemMode) {
        match &mode {
            SystemMode::Idle => {
                self.usb().teardown();
                self.rtc().teardown();

                // If we are exiting `Config`, `Setup` or `Alarm` mode let's play special signal.
                if let SystemMode::Setup(_) | SystemMode::Alarm(_, _) | SystemMode::Config =
                    self.state.mode
                {
                    self.beeper().play(Melody::Reset)
                }
            }
            SystemMode::Config => {
                self.beeper().play(Melody::Reset);
                self.usb().setup();
            }
            SystemMode::Setup(0) => self.beeper().play(Melody::Setup),
            SystemMode::Setup(c) if *c > 0 => self.beeper().play(Melody::Beep),
            SystemMode::Alarm(time, _) => {
                // We don't need to additionally beep if we transition from one Alarm mode to
                // another that means we're in a Snooze mode.
                match self.state.mode {
                    SystemMode::Alarm(_, _) => {}
                    _ => self.beeper().play(Melody::Setup),
                }

                let rtc = self.rtc();
                rtc.setup();
                rtc.set_time(Time::default());
                rtc.set_alarm(*time);
            }
            _ => {}
        }

        self.state.mode = mode;
    }

    /// Creates an instance of `ADC` controller.
    fn adc(&mut self) -> ADC<T> {
        ADC::new(&self.hw)
    }

    /// Creates an instance of `RTC` controller.
    fn rtc(&mut self) -> RTC<T> {
        RTC::new(&self.hw)
    }

    fn timer(&mut self) -> Timer<T> {
        Timer::new(&self.hw)
    }

    /// Creates an instance of `Beeper` controller.
    fn beeper(&mut self) -> PWMBeeper<T, S> {
        PWMBeeper::new(&self.hw, &mut self.systick, &mut self.state.beeper_state)
    }

    /// Creates an instance of `Buttons` controller.
    fn buttons(&mut self) -> Buttons<T> {
        Buttons::new(&self.hw, &mut self.state.buttons_state)
    }

    /// Creates an instance of `Flash` controller.
    fn flash(&mut self) -> Flash<T> {
        Flash::new(&self.hw)
    }

    /// Creates an instance of `USB` controller.
    fn usb(&mut self) -> USB<T> {
        USB::new(&self.hw, &mut self.state.usb_state)
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
