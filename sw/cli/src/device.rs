mod device_identifier;

pub use self::device_identifier::DeviceIdentifier;

use hidapi::{HidApi, HidDevice, HidDeviceInfo};
use kroneum_api::{
    adc::ADCChannel,
    array::Array,
    beeper::tone::Tone,
    config::{DEVICE_PID, DEVICE_VID},
    flash::storage_slot::StorageSlot,
    time::Time,
    usb::command_packet::CommandPacket,
};
use std::time::Duration;

const MAX_ALARM_SECONDS: u64 = 3600 * 24;

pub struct Device {
    device: HidDevice,
    device_info: HidDeviceInfo,
}

impl Device {
    pub fn create() -> Result<Self, String> {
        let api = HidApi::new()
            .or_else(|err| Err(format!("Failed to create HID API adapter {:?}", err)))?;

        api.devices()
            .iter()
            .find(|dev| dev.product_id == DEVICE_PID && dev.vendor_id == DEVICE_VID)
            .cloned()
            .ok_or_else(|| "Failed to find HID device.".to_string())
            .and_then(|device_info| {
                api.open(DEVICE_VID, DEVICE_PID)
                    .or_else(|err| Err(format!("Failed to open HID device {:?}", err)))
                    .map(|device| Device {
                        device_info,
                        device,
                    })
            })
    }
}

impl Device {
    pub fn get_identifier(&self) -> Result<DeviceIdentifier, String> {
        self.device_info
            .manufacturer_string
            .clone()
            .ok_or_else(|| "Failed to retrieve device manufacturer.".to_string())
            .map(|manufacturer| DeviceIdentifier {
                bus: 0,
                address: 0,
                vendor_id: self.device_info.vendor_id,
                product_id: self.device_info.product_id,
                manufacturer,
            })
    }

    pub fn write(&self, packet: CommandPacket) -> Result<(), String> {
        self.device
            .write(Array::from(packet).as_ref())
            .map(|_| ())
            .or_else(|err| Err(format!("Failed to send data to device endpoint: {:?}", err)))
    }

    pub fn read(&self) -> Result<Vec<u8>, String> {
        let mut data = [0; 100];
        self.device
            .read_timeout(&mut data, 5000)
            .or_else(|err| Err(format!("Failed to read data to device endpoint: {:?}", err)))
            .map(|count| data[..count].to_vec())
    }

    pub fn beep(&self, beeps_n: u8) -> Result<(), String> {
        self.write(CommandPacket::Beep(beeps_n))
    }

    pub fn play_melody(&self, tones: Array<Tone>) -> Result<(), String> {
        self.write(CommandPacket::Melody(tones))
    }

    pub fn get_alarm(&self) -> Result<Duration, String> {
        self.write(CommandPacket::AlarmGet)
            .and_then(|_| self.read())
            .map(|data| {
                Duration::from_secs(
                    u64::from(data[0]) * 3600 + u64::from(data[1]) * 60 + u64::from(data[2]),
                )
            })
    }

    pub fn set_alarm(&self, duration: Duration) -> Result<(), String> {
        let duration_sec = duration.as_secs();
        if duration_sec >= MAX_ALARM_SECONDS {
            return Err("Alarm is limited to 23h 59m 59s".to_string());
        }

        self.write(CommandPacket::AlarmSet(Time::from_seconds(
            duration_sec as u32,
        )))
    }

    pub fn read_flash(&self, slot: StorageSlot) -> Result<u8, String> {
        if let StorageSlot::Unknown = slot {
            return Err("Unknown memory slot is provided".to_string());
        }

        self.write(CommandPacket::FlashRead(slot))
            .and_then(|_| self.read())
            .map(|data| data[0])
    }

    pub fn write_flash(&self, slot: StorageSlot, value: u8) -> Result<(), String> {
        if let StorageSlot::Unknown = slot {
            return Err("Unknown memory slot is provided".to_string());
        }

        if self
            .write(CommandPacket::FlashWrite(slot, value))
            .and_then(|_| self.read())
            .map(|data| data[0] == 1)?
        {
            Ok(())
        } else {
            Err(format!(
                "Could not write value {} to a memory slot {:#X}",
                value,
                Into::<u8>::into(slot)
            ))
        }
    }

    pub fn erase_flash(&self) -> Result<(), String> {
        self.write(CommandPacket::FlashEraseAll)
    }

    pub fn reset(&self) -> Result<(), String> {
        self.write(CommandPacket::Reset)
    }

    pub fn echo(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        self.write(CommandPacket::Echo(Array::from(data)))
            .and_then(|_| self.read())
    }

    pub fn read_adc(&self, channel: ADCChannel) -> Result<u16, String> {
        self.write(CommandPacket::ADCRead(channel))
            .and_then(|_| self.read())
            .map(|data| (data[0] as u16) | ((data[1] as u16) << 8))
    }
}
