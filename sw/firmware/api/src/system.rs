use crate::{beeper::Melody, time::Time, usb::UsbState};
use beeper::PWMBeeperHardware;
use flash::FlashHardware;
use rtc::RTCHardware;
use usb::USBHardware;

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
    type B: PWMBeeperHardware;
    type R: RTCHardware;
    type F: FlashHardware;
    type U: USBHardware;

    /// Initializes hardware if needed.
    fn setup(&self);

    /// Turns on/off system standby mode.
    fn toggle_standby_mode(&mut self, on: bool);

    /// Performs a software reset.
    fn reset(&mut self);

    /// Returns the `PWMBeeperHardware` used to create `PWMBeeper` component.
    fn beeper<'b: 'a>(&'b self) -> Self::B;

    /// Returns the `RTCHardware` used to create `RTC` component.
    fn rtc<'b: 'a>(&'b self) -> Self::R;

    /// Returns the `FlashHardware` used to create `Flash` component.
    fn flash<'b: 'a>(&'b self) -> Self::F;

    /// Returns the `USBHardware` used to create `USB` component.
    fn usb<'b: 'a>(&'b self) -> Self::U;
}
