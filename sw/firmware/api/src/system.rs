use crate::{beeper::Melody, time::Time, usb::UsbState};
use beeper::PWMBeeperHardware;
use buttons::ButtonsHardware;
use flash::FlashHardware;
use rtc::RTCHardware;
use system_control::SystemControlHardware;
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
    type B: ButtonsHardware + 'a;
    type F: FlashHardware + 'a;
    type P: PWMBeeperHardware + 'a;
    type R: RTCHardware + 'a;
    type S: SystemControlHardware + 'a;
    type U: USBHardware + 'a;

    /// Initializes hardware if needed.
    fn setup(&'a self);

    /// Returns the `PWMBeeperHardware` used to create `PWMBeeper` component.
    fn beeper(&'a self) -> Self::P;

    // Returns the `ButtonsHardware` used to create `Buttons` component.
    fn buttons(&'a self) -> Self::B;

    /// Returns the `RTCHardware` used to create `RTC` component.
    fn rtc(&'a self) -> Self::R;

    /// Returns the `SystemControlHardware` used to create `SystemControl` component.
    fn system_control(&'a mut self) -> Self::S;

    /// Returns the `FlashHardware` used to create `Flash` component.
    fn flash(&'a self) -> Self::F;

    /// Returns the `USBHardware` used to create `USB` component.
    fn usb(&'a self) -> Self::U;
}
