use crate::device::{Device, DeviceIdentifier, KRONEUM_PID, KRONEUM_VID, REPORT_SIZE};
use hidapi::{HidApi, HidDevice, HidDeviceInfo};

pub struct DeviceHIDAPI {
    device: HidDevice,
    device_info: HidDeviceInfo,
}

impl DeviceHIDAPI {
    pub fn open() -> Result<DeviceHIDAPI, String> {
        HidApi::new()
            .or_else(|err| Err(format!("Failed to create HID API adapter {:?}", err)))
            .and_then(|api| {
                api.devices()
                    .iter()
                    .find(|dev| dev.product_id == KRONEUM_PID && dev.vendor_id == KRONEUM_VID)
                    .cloned()
                    .ok_or_else(|| "Failed to find HID device.".to_string())
                    .map(|device_info| (device_info, api))
            })
            .and_then(|(device_info, api)| {
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

    fn write(&self, data: &[u8]) -> Result<(), String> {
        let mut report = vec![0];
        report.extend_from_slice(data);

        self.device
            .write(report.as_ref())
            .map(|_| ())
            .or_else(|err| Err(format!("Failed to send data to device endpoint: {:?}", err)))
    }

    fn read(&self) -> Result<(usize, [u8; REPORT_SIZE]), String> {
        let mut data = [0; REPORT_SIZE];
        self.device
            .read_timeout(&mut data, 5000)
            .or_else(|err| Err(format!("Failed to read data to device endpoint: {:?}", err)))
            .map(|count| (count, data))
    }
}
