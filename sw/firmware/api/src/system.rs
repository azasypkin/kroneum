use crate::{beeper::Melody, time::Time, usb::UsbState};

#[derive(Debug, Copy, Clone)]
pub enum SystemMode {
    Idle,
    Setup(u32),
    Alarm(Time, Melody),
    Config,
}

#[derive(Copy, Clone)]
pub struct SystemState {
    pub mode: SystemMode,
    pub usb_state: UsbState,
}

impl Default for SystemState {
    fn default() -> Self {
        SystemState {
            mode: SystemMode::Idle,
            usb_state: UsbState::default(),
        }
    }
}

/// Describes the System hardware management interface.
pub trait SystemHardware {
    /// Initializes hardware if needed.
    fn setup(&self);

    /// Turns on/off system standby mode.
    fn toggle_standby_mode(&mut self, on: bool);

    /// Performs a software reset.
    fn reset(&mut self);
}
