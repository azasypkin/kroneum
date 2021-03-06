mod device_identifier;

pub use self::device_identifier::DeviceInfo;

use hidapi::{HidApi, HidDevice};
use kroneum_api::{
    adc::ADCChannel,
    array::Array,
    beeper::tone::Tone,
    config::{DEVICE_PID, DEVICE_VID},
    flash::storage_slot::StorageSlot,
    system::SystemInfo,
    time::Time,
    usb::{
        command_packet::CommandPacket,
        commands::{
            ADCCommand, AlarmCommand, BeeperCommand, FlashCommand, KeyModifiers, KeyboardCommand,
            MediaKey, RadioCommand, SystemCommand,
        },
    },
};
use std::convert::TryFrom;
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
    pub fn get_info(&self) -> DeviceInfo {
        DeviceInfo {
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

    pub fn beeper_melody(&self, tones: &[Tone]) -> Result<(), String> {
        self.send_command(CommandPacket::Beeper(BeeperCommand::Melody(Array::from(
            tones,
        ))))
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

        if self
            .send_command(CommandPacket::Alarm(AlarmCommand::Set(Time::from_seconds(
                duration_sec as u32,
            ))))
            .is_ok()
        {
            Ok(())
        } else {
            Err("Failed to set alarm".to_string())
        }
    }

    pub fn read_flash(&self, slot: StorageSlot) -> Result<u8, String> {
        if let Ok(response) = self.send_command(CommandPacket::Flash(FlashCommand::Read(slot))) {
            if !response.is_empty() {
                return Ok(response[0]);
            }
        }

        Err("Failed to read Flash value".to_string())
    }

    pub fn write_flash(&self, slot: StorageSlot, value: u8) -> Result<(), String> {
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

    pub fn system_get_info(&self) -> Result<SystemInfo, String> {
        self.send_command(CommandPacket::System(SystemCommand::GetInfo))
            .map_err(|_| "Failed to get system info".to_string())
            .and_then(|response| {
                SystemInfo::try_from(Array::from(&response))
                    .map_err(|_| "Received corrupted system info".to_string())
            })
    }

    pub fn adc_read(&self, channel: ADCChannel) -> Result<u16, String> {
        info!("Reading ADC for {:?}.", channel);
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

    pub fn radio_transmit(&self, data: &[u8]) -> Result<(), String> {
        self.send_command(CommandPacket::Radio(RadioCommand::Transmit(Array::from(
            data,
        ))))
        .map(|_| ())
        .map_err(|_| "Failed to transmit data over radio".to_string())
    }

    pub fn keyboard_key(
        &self,
        modifiers: KeyModifiers,
        key_code: u8,
        delay_s: u8,
    ) -> Result<(), String> {
        self.send_command(CommandPacket::Keyboard(KeyboardCommand::Key(
            modifiers, key_code, delay_s,
        )))
        .map(|_| ())
        .map_err(|_| "Failed to send a keyboard key".to_string())
    }

    pub fn keyboard_media_key(&self, media_key: MediaKey, delay_s: u8) -> Result<(), String> {
        self.send_command(CommandPacket::Keyboard(KeyboardCommand::Media(
            media_key, delay_s,
        )))
        .map(|_| ())
        .map_err(|_| "Failed to send a keyboard media key".to_string())
    }

    fn send_command(&self, packet: CommandPacket) -> Result<Vec<u8>, String> {
        self.write(packet)
            .and_then(|_| self.read())
            .and_then(|mut response| {
                if response.is_empty() || response[0] == 0xFF {
                    error!("Failed to process packet {:?}.", response);
                    Err("Failed to process packet".to_string())
                } else {
                    info!("Successfully processed packet {:?}.", response);
                    Ok(response.drain(1..).collect())
                }
            })
    }

    fn write(&self, packet: CommandPacket) -> Result<(), String> {
        let packet_bytes = Array::from(packet);
        self.device
            .write(packet_bytes.as_ref())
            .map(|count| {
                info!(
                    "Successfully wrote {:?} ({}) bytes.",
                    packet_bytes.as_ref(),
                    count
                );
            })
            .or_else(|err| {
                error!(
                    "Failed to wrote {:?} ({}) bytes: {:?}",
                    packet_bytes.as_ref(),
                    packet_bytes.len(),
                    err
                );
                Err(format!("Failed to send data to device endpoint: {:?}", err))
            })
    }

    fn read(&self) -> Result<Vec<u8>, String> {
        let mut data = [0; 100];
        self.device
            .read_timeout(&mut data, 5000)
            .or_else(|err| {
                error!("Failed to read bytes: {:?}", err);
                Err(format!("Failed to read data to device endpoint: {:?}", err))
            })
            .map(|count| {
                info!("Successfully read {} byte(s).", count);
                data[..count].to_vec()
            })
    }
}
