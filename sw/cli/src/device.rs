mod device_identifier;

pub use self::device_identifier::DeviceIdentifier;

use hidapi::{HidApi, HidDevice};
use kroneum_api::{
    adc::ADCChannel,
    array::Array,
    beeper::tone::Tone,
    config::{DEVICE_PID, DEVICE_VID},
    flash::storage_slot::StorageSlot,
    time::Time,
    usb::{
        command_packet::CommandPacket,
        commands::{
            ADCCommand, AlarmCommand, BeeperCommand, FlashCommand, RadioCommand, SystemCommand,
        },
    },
};
use std::time::Duration;

const MAX_ALARM_SECONDS: u64 = 3600 * 24;

pub struct Device {
    device: HidDevice,
    manufacturer: String,
}

impl Device {
    pub fn create() -> Result<Self, String> {
        let api = HidApi::new()
            .or_else(|err| Err(format!("Failed to create HID API adapter {:?}", err)))?;

        let device_info = api
            .device_list()
            .find(|dev| dev.product_id() == DEVICE_PID && dev.vendor_id() == DEVICE_VID)
            .ok_or_else(|| "Failed to find HID device.".to_string())?;

        let manufacturer = device_info
            .manufacturer_string()
            .unwrap_or_else(|| "")
            .to_string();

        api.open(DEVICE_VID, DEVICE_PID)
            .or_else(|err| Err(format!("Failed to open HID device {:?}", err)))
            .map(|device| Device {
                device,
                manufacturer,
            })
    }
}

impl Device {
    pub fn get_identifier(&self) -> DeviceIdentifier {
        DeviceIdentifier {
            bus: 0,
            address: 0,
            vendor_id: DEVICE_VID,
            product_id: DEVICE_PID,
            manufacturer: self.manufacturer.clone(),
        }
    }

    pub fn beeper_beep(&self, n_beeps: u8) -> Result<(), String> {
        self.send_command(CommandPacket::Beeper(BeeperCommand::Beep(n_beeps)))
            .map(|_| ())
            .map_err(|_| "Failed to beep".to_string())
    }

    pub fn beeper_melody(&self, tones: Array<Tone>) -> Result<(), String> {
        self.send_command(CommandPacket::Beeper(BeeperCommand::Melody(tones)))
            .map(|_| ())
            .map_err(|_| "Failed to play melody".to_string())
    }

    pub fn get_alarm(&self) -> Result<Duration, String> {
        if let Ok(response) = self.send_command(CommandPacket::Alarm(AlarmCommand::Get)) {
            if response.len() == 3 {
                return Ok(Duration::from_secs(
                    u64::from(response[0]) * 3600
                        + u64::from(response[1]) * 60
                        + u64::from(response[2]),
                ));
            }
        }

        Err("Failed to get alarm time".to_string())
    }

    pub fn set_alarm(&self, duration: Duration) -> Result<(), String> {
        let duration_sec = duration.as_secs();
        if duration_sec >= MAX_ALARM_SECONDS {
            return Err("Alarm is limited to 23h 59m 59s".to_string());
        }

        if let Ok(_) = self.send_command(CommandPacket::Alarm(AlarmCommand::Set(
            Time::from_seconds(duration_sec as u32),
        ))) {
            Ok(())
        } else {
            Err("Failed to set alarm".to_string())
        }
    }

    pub fn read_flash(&self, slot: StorageSlot) -> Result<u8, String> {
        if let StorageSlot::Unknown = slot {
            return Err("Unknown memory slot is provided".to_string());
        }

        if let Ok(response) = self.send_command(CommandPacket::Flash(FlashCommand::Read(slot))) {
            if response.len() > 0 {
                return Ok(response[0]);
            }
        }

        Err("Failed to read Flash value".to_string())
    }

    pub fn write_flash(&self, slot: StorageSlot, value: u8) -> Result<(), String> {
        if let StorageSlot::Unknown = slot {
            return Err("Unknown memory slot is provided".to_string());
        }

        let packet = CommandPacket::Flash(FlashCommand::Write(slot, value));
        self.send_command(packet).map(|_| ()).map_err(|_| {
            format!(
                "Could not write value {} to a memory slot {:#X}",
                value,
                Into::<u8>::into(slot)
            )
        })
    }

    pub fn erase_flash(&self) -> Result<(), String> {
        let packet = CommandPacket::Flash(FlashCommand::EraseAll);
        self.send_command(packet)
            .map(|_| ())
            .map_err(|_| "Could not erase flash".to_string())
    }

    pub fn system_reset(&self) -> Result<(), String> {
        self.send_command(CommandPacket::System(SystemCommand::Reset))
            .map(|_| ())
            .map_err(|_| "Failed to reset device".to_string())
    }

    pub fn system_echo(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        self.send_command(CommandPacket::System(SystemCommand::Echo(Array::from(
            data,
        ))))
        .map_err(|_| "Failed to send/receive echo data".to_string())
    }

    pub fn adc_read(&self, channel: ADCChannel) -> Result<u16, String> {
        self.send_command(CommandPacket::ADC(ADCCommand::Read(channel)))
            .map(|response| (response[0] as u16) | ((response[1] as u16) << 8))
            .map_err(|_| "Failed to read ADC value".to_string())
    }

    pub fn radio_status(&self) -> Result<Vec<u8>, String> {
        self.send_command(CommandPacket::Radio(RadioCommand::Status))
            .map_err(|_| "Failed to retrieve radio status".to_string())
    }

    pub fn radio_receive(&self) -> Result<Vec<u8>, String> {
        self.send_command(CommandPacket::Radio(RadioCommand::Receive))
            .map_err(|_| "Failed to receive data over radio".to_string())
    }

    pub fn radio_transmit(&self, data: Array<u8>) -> Result<(), String> {
        self.send_command(CommandPacket::Radio(RadioCommand::Transmit(data)))
            .map(|_| ())
            .map_err(|_| "Failed to transmit data over radio".to_string())
    }

    fn send_command(&self, packet: CommandPacket) -> Result<Vec<u8>, String> {
        self.write(packet)
            .and_then(|_| self.read())
            .and_then(|mut response| {
                if response.len() == 0 || response[0] == 0xFF {
                    Err("Failed to process packet".to_string())
                } else {
                    Ok(response.drain(1..).collect())
                }
            })
    }

    fn write(&self, packet: CommandPacket) -> Result<(), String> {
        self.device
            .write(Array::from(packet).as_ref())
            .map(|_| ())
            .or_else(|err| Err(format!("Failed to send data to device endpoint: {:?}", err)))
    }

    fn read(&self) -> Result<Vec<u8>, String> {
        let mut data = [0; 100];
        self.device
            .read_timeout(&mut data, 5000)
            .or_else(|err| Err(format!("Failed to read data to device endpoint: {:?}", err)))
            .map(|count| data[..count].to_vec())
    }
}
