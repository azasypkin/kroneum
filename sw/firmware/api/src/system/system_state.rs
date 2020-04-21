use super::system_role::{SystemRole, TimerRoleMode};
use beeper::BeeperState;
use buttons::ButtonsState;
use usb::UsbState;

#[derive(Copy, Clone)]
pub struct SystemState {
    pub role: SystemRole,
    pub role_state: Option<RoleState>,
    pub peripherals_states: PeripheralsStates,
}

impl Default for SystemState {
    fn default() -> Self {
        SystemState {
            role: SystemRole::default(),
            role_state: None,
            peripherals_states: PeripheralsStates {
                usb: UsbState::default(),
                beeper: BeeperState::default(),
                buttons: ButtonsState::default(),
            },
        }
    }
}

#[derive(Copy, Clone)]
pub struct PeripheralsStates {
    pub usb: UsbState,
    pub beeper: BeeperState,
    pub buttons: ButtonsState,
}

#[derive(Debug, Copy, Clone)]
pub enum RoleState {
    Timer(TimerRoleMode),
}
