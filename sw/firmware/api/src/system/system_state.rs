use super::SystemMode;
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
    pub mode: SystemMode,
    pub peripherals: PeripheralStates,
}

impl Default for SystemState {
    fn default() -> Self {
        SystemState {
            mode: SystemMode::Idle,
            peripherals: PeripheralStates {
                usb: UsbState::default(),
                beeper: BeeperState::default(),
                buttons: ButtonsState::default(),
            },
        }
    }
}
