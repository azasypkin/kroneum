use crate::{beeper::Melody, time::Time, usb::UsbState};
use rtc::RTCHardware;

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
pub trait SystemHardware<'a> {
    type R: RTCHardware;

    /// Initializes hardware if needed.
    fn setup(&self);

    /// Turns on/off system standby mode.
    fn toggle_standby_mode(&mut self, on: bool);

    /// Performs a software reset.
    fn reset(&mut self);

    /// Returns an `RTCHardware` used to create `RTC` component.
    fn rtc<'b: 'a>(&'b self) -> Self::R;
}
