use super::super::{system_role::SystemRole, system_state::RoleState, System, SystemHardware};
use beeper::melody::Melody;
use buttons::ButtonPressType;
use systick::SysTickHardware;
use time::Time;

#[derive(Debug, Copy, Clone)]
pub enum TimerRoleMode {
    Idle,
    Setup(u32),
    Alarm(Time, Melody),
}

pub struct TimerSystemRoleHandler;
impl TimerSystemRoleHandler {
    pub fn on_buttons_press<T: SystemHardware, S: SysTickHardware>(
        system: &mut System<T, S>,
        (button_i, button_x): (ButtonPressType, ButtonPressType),
    ) {
        let current_mode = if let Some(RoleState::Timer(mode)) = system.state.role_state {
            mode
        } else {
            TimerRoleMode::Idle
        };

        match (current_mode, button_i, button_x) {
            (TimerRoleMode::Alarm(_, _), ButtonPressType::Long, ButtonPressType::Long) => {
                Self::set_mode(system, TimerRoleMode::Idle);
            }
            (TimerRoleMode::Idle, ButtonPressType::Long, ButtonPressType::Long) => {
                system.switch_to_role(SystemRole::Controller)
            }
            (TimerRoleMode::Setup(counter), ButtonPressType::Long, ButtonPressType::Long) => {
                Self::set_mode(
                    system,
                    TimerRoleMode::Alarm(Time::from_hours(counter), Melody::Alarm),
                );
            }
            (TimerRoleMode::Idle, ButtonPressType::Long, _)
            | (TimerRoleMode::Idle, _, ButtonPressType::Long) => {
                Self::set_mode(system, TimerRoleMode::Setup(0));
            }
            (TimerRoleMode::Alarm(_, _), ButtonPressType::Long, _)
            | (TimerRoleMode::Alarm(_, _), _, ButtonPressType::Long) => {
                Self::set_mode(system, TimerRoleMode::Idle);
            }
            (TimerRoleMode::Setup(counter), ButtonPressType::Long, _)
            | (TimerRoleMode::Setup(counter), _, ButtonPressType::Long) => {
                let time = match button_i {
                    ButtonPressType::Long => Time::from_seconds(counter as u32),
                    _ => Time::from_minutes(counter as u32),
                };

                Self::set_mode(system, TimerRoleMode::Alarm(time, Melody::Alarm));
            }
            (TimerRoleMode::Setup(counter), ButtonPressType::Short, _) => {
                Self::set_mode(system, TimerRoleMode::Setup(counter + 1));
            }
            (TimerRoleMode::Setup(counter), _, ButtonPressType::Short) => {
                Self::set_mode(system, TimerRoleMode::Setup(counter + 10));
            }
            _ => {}
        }
    }

    pub fn on_alarm<T: SystemHardware, S: SysTickHardware>(system: &mut System<T, S>) {
        if let Some(RoleState::Timer(TimerRoleMode::Alarm(_, melody))) = system.state.role_state {
            system.beeper().play_and_repeat(melody, 2);

            system.rtc().teardown();

            // Snooze alarm for 10 seconds.
            Self::set_mode(
                system,
                TimerRoleMode::Alarm(Time::from_seconds(10), Melody::Beep),
            );
        }
    }

    pub fn set_mode<T: SystemHardware, S: SysTickHardware>(
        system: &mut System<T, S>,
        mode: TimerRoleMode,
    ) {
        let current_mode = if let Some(RoleState::Timer(mode)) = system.state.role_state {
            mode
        } else {
            TimerRoleMode::Idle
        };

        match &mode {
            TimerRoleMode::Idle => {
                system.rtc().teardown();
                system.beeper().play(Melody::Reset);
            }
            TimerRoleMode::Setup(c) => {
                system
                    .beeper()
                    .play(if *c > 0 { Melody::Beep } else { Melody::Setup })
            }
            TimerRoleMode::Alarm(time, _) => {
                // We don't need to additionally beep if we transition from one Alarm mode to
                // another that means we're in a Snooze mode.
                if !matches!(current_mode, TimerRoleMode::Alarm(_, _)) {
                    system.beeper().play(Melody::Setup)
                }

                let rtc = system.rtc();
                rtc.setup();
                rtc.set_time(Time::default());
                rtc.set_alarm(*time);
            }
        }

        system.state.role_state = Some(RoleState::Timer(mode));
    }
}
