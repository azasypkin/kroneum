use super::system_role::{SystemRole, TimerMode};
use beeper::BeeperState;
use buttons::ButtonsState;
use usb::UsbState;

#[derive(Copy, Clone)]
pub struct PeripheralStates {
    pub usb: UsbState,
    pub beeper: BeeperState,
    pub buttons: ButtonsState,
}

#[derive(Copy, Clone)]
pub struct SystemState {
    pub role: SystemRole,
    pub peripherals: PeripheralStates,
}

impl Default for SystemState {
    fn default() -> Self {
        SystemState {
            role: SystemRole::Timer(TimerMode::Idle),
            peripherals: PeripheralStates {
                usb: UsbState::default(),
                beeper: BeeperState::default(),
                buttons: ButtonsState::default(),
            },
        }
    }
}
