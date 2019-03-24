use crate::device::{Device, DeviceContext, DeviceIdentifier, KRONEUM_PID, KRONEUM_VID};
use hidapi::{HidApi, HidDevice, HidDeviceInfo};
use kroneum_api::usb::command_packet::{CommandByteSequence, CommandPacket};

pub struct DeviceContextHIDAPI {
    api: HidApi,
}

impl<'a> DeviceContext<'a, DeviceHIDAPI> for DeviceContextHIDAPI {
    fn create() -> Result<Self, String> {
        HidApi::new()
            .or_else(|err| Err(format!("Failed to create HID API adapter {:?}", err)))
            .map(|api| DeviceContextHIDAPI { api })
    }

    fn open(&self) -> Result<DeviceHIDAPI, String> {
        DeviceHIDAPI::open(&self.api)
    }

    fn close(&self, _: DeviceHIDAPI) -> Result<(), String> {
        Ok(())
    }
}

pub struct DeviceHIDAPI {
    device: HidDevice,
    device_info: HidDeviceInfo,
}

impl DeviceHIDAPI {
    pub fn open(api: &HidApi) -> Result<Self, String> {
        api.devices()
            .iter()
            .find(|dev| dev.product_id == KRONEUM_PID && dev.vendor_id == KRONEUM_VID)
            .cloned()
            .ok_or_else(|| "Failed to find HID device.".to_string())
            .and_then(|device_info| {
                api.open(KRONEUM_VID, KRONEUM_PID)
                    .or_else(|err| Err(format!("Failed to open HID device {:?}", err)))
                    .map(|device| DeviceHIDAPI {
                        device_info,
                        device,
                    })
            })
    }
}

impl Device for DeviceHIDAPI {
    fn get_identifier(&self) -> DeviceIdentifier {
        DeviceIdentifier {
            bus: 0,
            address: 0,
            vendor_id: self.device_info.vendor_id,
            product_id: self.device_info.product_id,
        }
    }

    fn get_manufacturer(&self) -> Result<String, String> {
        self.device_info
            .manufacturer_string
            .clone()
            .ok_or_else(|| "Failed to retrieve device manufacturer.".to_string())
    }

    fn write(&self, packet: CommandPacket) -> Result<(), String> {
        self.device
            .write(&packet.to_bytes())
            .map(|_| ())
            .or_else(|err| Err(format!("Failed to send data to device endpoint: {:?}", err)))
    }

    fn read(&self) -> Result<(usize, CommandByteSequence), String> {
        let mut data = CommandByteSequence::default();
        self.device
            .read_timeout(&mut data, 5000)
            .or_else(|err| Err(format!("Failed to read data to device endpoint: {:?}", err)))
            .map(|count| (count, data))
    }
}
