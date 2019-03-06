use crate::{
    beeper,
    buttons::{ButtonPressType, Buttons},
    rtc::{Time, RTC},
    usb::{CommandPacket, UsbState, USB},
    Peripherals,
};

#[derive(Debug, Copy, Clone)]
pub enum SystemMode {
    Idle,
    Setup(u8),
    Alarm(Time),
    Config,
}

#[derive(Copy, Clone)]
pub struct SystemState {
    pub mode: SystemMode,
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

pub struct System<'a> {
    p: &'a mut Peripherals,
    state: &'a mut SystemState,
}

impl<'a> System<'a> {
    pub fn new(p: &'a mut Peripherals, state: &'a mut SystemState) -> Self {
        System { p, state }
    }

    pub fn acquire<'b, F, R>(p: &'b mut Peripherals, state: &'b mut SystemState, f: F) -> R
    where
        F: FnOnce(System) -> R,
    {
        f(System::new(p, state))
    }

    pub fn setup(&mut self) {
        Buttons::acquire(&mut self.p, |mut buttons| buttons.setup());

        self.set_mode(SystemMode::Idle);
    }

    pub fn set_mode(&mut self, mode: SystemMode) {
        match &mode {
            SystemMode::Idle => {
                self.toggle_standby_mode(true);

                USB::acquire(&mut self.p, &mut self.state.usb_state, |mut usb| {
                    usb.teardown()
                });

                // If we are exiting `Config` mode let's play special signal.
                if let SystemMode::Setup(_) = self.state.mode {
                    beeper::acquire(&mut self.p, |beeper| beeper.play_reset());
                }
            }
            SystemMode::Config => {
                beeper::acquire(&mut self.p, |beeper| beeper.play_reset());

                self.toggle_standby_mode(false);

                USB::acquire(&mut self.p, &mut self.state.usb_state, |mut usb| {
                    usb.setup()
                });
            }
            SystemMode::Setup(0) => beeper::acquire(&mut self.p, |beeper| beeper.play_setup()),
            SystemMode::Setup(c) if *c > 0 => beeper::acquire(&mut self.p, |beeper| beeper.beep()),
            SystemMode::Alarm(time) => {
                beeper::acquire(&mut self.p, |beeper| beeper.play_setup());

                RTC::acquire(&mut self.p, |mut rtc| {
                    rtc.setup();
                    rtc.configure_time(&Time::default());
                    rtc.configure_alarm(&time);
                });
            }
            _ => {}
        }

        self.state.mode = mode;
    }

    pub fn on_rtc_alarm(&mut self) {
        beeper::acquire(&mut self.p, |beeper| beeper.play_melody());

        RTC::acquire(&mut self.p, |mut rtc| rtc.teardown());

        self.set_mode(SystemMode::Idle);
    }

    pub fn on_usb_packet(&mut self) {
        USB::acquire(&mut self.p, &mut self.state.usb_state, |mut usb| {
            usb.interrupt(|p, command_packet| {
                if let CommandPacket::Beep(num) = command_packet {
                    beeper::acquire(p, |beeper| beeper.beep_n(num));
                } else if let CommandPacket::SetAlarm(time) = command_packet {
                    RTC::acquire(p, |mut rtc| {
                        rtc.setup();
                        rtc.configure_time(&Time::default());
                        rtc.configure_alarm(&time);
                    });
                } else if let CommandPacket::GetAlarm = command_packet {
                    beeper::acquire(p, |beeper| beeper.beep_n(1));
                    let alarm = RTC::acquire(p, |rtc| rtc.get_alarm());
                    return Some([alarm.hours, alarm.minutes, alarm.seconds, 0, 0, 0]);
                }

                None
            });
        });
    }

    pub fn on_button_press(&mut self) -> bool {
        let has_pending_interrupt =
            Buttons::acquire(&mut self.p, |buttons| buttons.has_pending_interrupt());
        if !has_pending_interrupt {
            return false;
        }

        let (button_i, button_x) = Buttons::acquire(&mut self.p, |mut buttons| buttons.interrupt());

        match (self.state.mode.clone(), button_i, button_x) {
            (mode @ _, ButtonPressType::Long, ButtonPressType::Long) => {
                self.set_mode(SystemMode::Setup(0));

                let (button_i, button_x) =
                    Buttons::acquire(&mut self.p, |mut buttons| buttons.interrupt());

                match (mode, button_i, button_x) {
                    (SystemMode::Config, ButtonPressType::Long, ButtonPressType::Long) => {
                        self.set_mode(SystemMode::Idle)
                    }
                    (_, ButtonPressType::Long, ButtonPressType::Long) => {
                        self.set_mode(SystemMode::Config)
                    }
                    (SystemMode::Setup(counter), _, _) => {
                        self.set_mode(SystemMode::Alarm(Time::from_hours(counter)))
                    }
                    _ => {}
                }
            }
            (SystemMode::Idle, ButtonPressType::Long, _)
            | (SystemMode::Idle, _, ButtonPressType::Long)
            | (SystemMode::Alarm(_), ButtonPressType::Long, _)
            | (SystemMode::Alarm(_), _, ButtonPressType::Long) => {
                self.set_mode(SystemMode::Setup(0))
            }
            (SystemMode::Setup(counter), ButtonPressType::Long, _)
            | (SystemMode::Setup(counter), _, ButtonPressType::Long) => {
                let time = match button_i {
                    ButtonPressType::Long => Time::from_seconds(counter as u32),
                    _ => Time::from_minutes(counter as u32),
                };

                self.set_mode(SystemMode::Alarm(time));
            }
            (SystemMode::Setup(counter), ButtonPressType::Short, _) => {
                self.set_mode(SystemMode::Setup(counter + 1))
            }
            (SystemMode::Setup(counter), _, ButtonPressType::Short) => {
                self.set_mode(SystemMode::Setup(counter + 10))
            }
            _ => {}
        }

        Buttons::acquire(&mut self.p, |buttons| buttons.clear_pending_interrupt());

        true
    }

    fn toggle_standby_mode(&mut self, on: bool) {
        // Toggle STANDBY mode.
        self.p.device.PWR.cr.modify(|_, w| w.pdds().bit(on));

        self.p.device.PWR.cr.modify(|_, w| w.cwuf().set_bit());

        // Toggle SLEEPDEEP bit of Cortex-M0 System Control Register.
        if on {
            self.p.core.SCB.set_sleepdeep();
        } else {
            self.p.core.SCB.clear_sleepdeep();
        }
    }
}
