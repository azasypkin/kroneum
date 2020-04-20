use adc::ADCHardware;
use beeper::PWMBeeperHardware;
use buttons::ButtonsHardware;
use flash::FlashHardware;
use radio::RadioHardware;
use rtc::RTCHardware;
use timer::TimerHardware;
use usb::USBHardware;

/// Describes the SystemControl hardware management interface.
pub trait SystemHardware:
    ADCHardware
    + ButtonsHardware
    + FlashHardware
    + PWMBeeperHardware
    + RTCHardware
    + RadioHardware
    + USBHardware
    + TimerHardware
{
    /// Forces system to enter StandBy mode.
    fn enter_deep_sleep(&mut self);

    /// Forces system to exit StandBy mode.
    fn exit_deep_sleep(&mut self);

    /// Performs system software reset.
    fn reset(&mut self);

    /// Returns a 12-byte unique device ID.
    fn device_id(&self) -> &'static [u8; 12];

    /// Returns a string with a hex-encoded unique device ID.
    fn device_id_hex(&self) -> &'static str;

    /// Returns the Flash memory size of the device in Kilobytes.
    fn flash_size_kb(&self) -> u16;
}
