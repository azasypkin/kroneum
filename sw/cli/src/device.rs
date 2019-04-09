use kroneum_api::{
    flash::storage_slot::StorageSlot,
    time::Time,
    usb::command_packet::{CommandByteSequence, CommandPacket},
};
use std::{fmt, time::Duration};

const MAX_ALARM_SECONDS: u64 = 3600 * 24;

/// Describes main parameters of the Kroneum device.
pub struct DeviceIdentifier {
    pub bus: u8,
    pub address: u8,
    pub vendor_id: u16,
    pub product_id: u16,
}

impl fmt::Display for DeviceIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bus: {:03}, Addr: {:03}, VID: {:04x}, PID: {:04x})",
            self.bus, self.address, self.vendor_id, self.product_id
        )
    }
}

pub trait Device {
    fn get_identifier(&self) -> DeviceIdentifier;
    fn get_manufacturer(&self) -> Result<String, String>;
    fn write(&self, packet: CommandPacket) -> Result<(), String>;
    fn read(&self) -> Result<(usize, CommandByteSequence), String>;

    fn beep(&self, beeps_n: u8) -> Result<(), String> {
        self.write(CommandPacket::Beep(beeps_n))
    }

    fn get_alarm(&self) -> Result<Duration, String> {
        self.write(CommandPacket::AlarmGet)
            .and_then(|_| self.read())
            .map(|(_, data)| {
                Duration::from_secs(
                    u64::from(data[0]) * 3600 + u64::from(data[1]) * 60 + u64::from(data[2]),
                )
            })
    }

    fn set_alarm(&self, duration: Duration) -> Result<(), String> {
        let duration_sec = duration.as_secs();
        if duration_sec >= MAX_ALARM_SECONDS {
            return Err("Alarm is limited to 23h 59m 59s".to_string());
        }

        self.write(CommandPacket::AlarmSet(Time::from_seconds(
            duration_sec as u32,
        )))
    }

    fn read_flash(&self, slot: StorageSlot) -> Result<u8, String> {
        if let StorageSlot::Unknown = slot {
            return Err("Unknown memory slot is provided".to_string());
        }

        self.write(CommandPacket::FlashRead(slot))
            .and_then(|_| self.read())
            .map(|(_, data)| data[0])
    }

    fn write_flash(&self, slot: StorageSlot, value: u8) -> Result<(), String> {
        if let StorageSlot::Unknown = slot {
            return Err("Unknown memory slot is provided".to_string());
        }

        if self
            .write(CommandPacket::FlashWrite(slot, value))
            .and_then(|_| self.read())
            .map(|(_, data)| if data[0] == 1 { true } else { false })?
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

    fn erase_flash(&self) -> Result<(), String> {
        self.write(CommandPacket::FlashEraseAll)
    }

    fn reset(&self) -> Result<(), String> {
        self.write(CommandPacket::Reset)
    }
}

pub trait DeviceContext<'a, D: Device, C = Self> {
    fn create() -> Result<C, String>;
    fn open(&'a self) -> Result<D, String>;
    fn close(&'a self, device: D) -> Result<(), String>;
}
