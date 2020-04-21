mod controller_system_role_handler;
mod timer_system_role_handler;

pub use self::{
    controller_system_role_handler::ControllerSystemRoleHandler,
    timer_system_role_handler::{TimerRoleMode, TimerSystemRoleHandler},
};

#[derive(Debug, Copy, Clone)]
pub enum SystemRole {
    Timer = 0x0,
    Controller = 0x1,
}

impl Default for SystemRole {
    fn default() -> Self {
        SystemRole::Timer
    }
}

impl From<u8> for SystemRole {
    fn from(value: u8) -> Self {
        match value {
            0x1 => SystemRole::Controller,
            _ => SystemRole::default(),
        }
    }
}
