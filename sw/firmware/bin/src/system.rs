use crate::{beeper, buttons, rtc, systick, usb};

use cortex_m::peripheral::SCB;
use stm32f0::stm32f0x2::Peripherals;

use kroneum_api::{
    beeper::Melody,
    buttons::ButtonPressType,
    time::Time,
    usb::{command_packet::CommandPacket, UsbState},
};

#[derive(Debug, Copy, Clone)]
pub enum SystemMode {
    Idle,
    Setup(u32),
    Alarm(Time, Melody),
    Config,
}

#[derive(Copy, Clone)]
struct SystemState {
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

pub struct System {
    p: Peripherals,
    systick: systick::SysTick,
    scb: SCB,
    state: SystemState,
}

impl System {
    pub fn new(p: Peripherals, systick: systick::SysTick, scb: SCB) -> Self {
        System {
            p,
            state: SystemState::default(),
            systick,
            scb,
        }
    }

    pub fn setup(&mut self) {
        buttons::setup(&mut self.p);

        self.set_mode(SystemMode::Idle);
    }

    pub fn set_mode(&mut self, mode: SystemMode) {
        match &mode {
            SystemMode::Idle => {
                self.toggle_standby_mode(true);

                usb::acquire(&mut self.p, &mut self.state.usb_state, |usb| usb.stop());
                usb::teardown(&mut self.p);
                rtc::teardown(&mut self.p);

                // If we are exiting `Config` or `Alarm` mode let's play special signal.
                if let SystemMode::Setup(_) = self.state.mode {
                    beeper::acquire(&mut self.p, &mut self.systick, |beeper| {
                        beeper.play(Melody::Reset)
                    });
                } else if let SystemMode::Alarm(_, _) = self.state.mode {
                    beeper::acquire(&mut self.p, &mut self.systick, |beeper| {
                        beeper.play(Melody::Reset)
                    });
                }
            }
            SystemMode::Config => {
                beeper::acquire(&mut self.p, &mut self.systick, |beeper| {
                    beeper.play(Melody::Reset)
                });

                self.toggle_standby_mode(false);

                usb::setup(&mut self.p);
                usb::acquire(&mut self.p, &mut self.state.usb_state, |usb| usb.start());
            }
            SystemMode::Setup(0) => beeper::acquire(&mut self.p, &mut self.systick, |beeper| {
                beeper.play(Melody::Setup)
            }),
            SystemMode::Setup(c) if *c > 0 => {
                beeper::acquire(&mut self.p, &mut self.systick, |beeper| beeper.beep())
            }
            SystemMode::Alarm(time, _) => {
                beeper::acquire(&mut self.p, &mut self.systick, |beeper| {
                    beeper.play(Melody::Setup)
                });

                rtc::setup(&mut self.p);
                rtc::acquire(&mut self.p, |rtc| {
                    rtc.set_time(Time::default());
                    rtc.set_alarm(*time);
                });
            }
            _ => {}
        }

        self.state.mode = mode;
    }

    pub fn on_rtc_alarm(&mut self) {
        if let SystemMode::Alarm(_, melody) = &self.state.mode {
            beeper::acquire(&mut self.p, &mut self.systick, |beeper| {
                beeper.play(*melody)
            });

            rtc::teardown(&mut self.p);

            // Snooze alarm for 10 seconds.
            self.set_mode(SystemMode::Alarm(Time::from_seconds(10), Melody::Beep));
        }
    }

    pub fn on_usb_packet(&mut self) {
        usb::acquire(&mut self.p, &mut self.state.usb_state, |usb| {
            usb.interrupt()
        });

        if let Some(command_packet) = self.state.usb_state.command {
            if let CommandPacket::Beep(num) = command_packet {
                beeper::acquire(&self.p, &mut self.systick, |beeper| beeper.beep_n(num));
            } else if let CommandPacket::SetAlarm(time) = command_packet {
                self.set_mode(SystemMode::Alarm(time, Melody::Alarm));
            } else if let CommandPacket::GetAlarm = command_packet {
                beeper::acquire(&self.p, &mut self.systick, |beeper| beeper.beep());
                let alarm = rtc::acquire(&self.p, |rtc| rtc.alarm());

                usb::acquire(&mut self.p, &mut self.state.usb_state, |usb| {
                    usb.send(&[alarm.hours, alarm.minutes, alarm.seconds, 0, 0, 0])
                });
            }
        }

        self.state.usb_state.command = None;
    }

    pub fn on_button_press(&mut self) {
        if !buttons::has_pending_interrupt(&self.p) {
            return;
        }

        let (button_i, button_x) = buttons::acquire(&mut self.p, &mut self.systick, |buttons| {
            buttons.interrupt()
        });

        match (self.state.mode, button_i, button_x) {
            (mode, ButtonPressType::Long, ButtonPressType::Long) => {
                let (button_i, button_x) =
                    buttons::acquire(&mut self.p, &mut self.systick, |buttons| {
                        buttons.interrupt()
                    });

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
            | (SystemMode::Idle, _, ButtonPressType::Long)
            | (SystemMode::Alarm(_, _), ButtonPressType::Long, _)
            | (SystemMode::Alarm(_, _), _, ButtonPressType::Long) => {
                self.set_mode(SystemMode::Setup(0))
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

        buttons::clear_pending_interrupt(&self.p);
    }

    fn toggle_standby_mode(&mut self, on: bool) {
        // Toggle STANDBY mode.
        self.p.PWR.cr.modify(|_, w| w.pdds().bit(on));

        self.p.PWR.cr.modify(|_, w| w.cwuf().set_bit());

        // Toggle SLEEPDEEP bit of Cortex-M0 System Control Register.
        if on {
            self.scb.set_sleepdeep();
        } else {
            self.scb.clear_sleepdeep();
        }
    }
}
