use crate::device::{Device, DeviceIdentifier, KRONEUM_PID, KRONEUM_VID, REPORT_SIZE};
use std::time::Duration;

const INTERFACE: u8 = 0;

pub struct DeviceLibUSB<'a> {
    descriptor: libusb::DeviceDescriptor,
    device: libusb::Device<'a>,
    handle: libusb::DeviceHandle<'a>,
    detached_kernel_driver: bool,
}

impl<'a> DeviceLibUSB<'a> {
    pub fn open(context: &'a libusb::Context) -> Result<DeviceLibUSB<'a>, String> {
        let (device, mut handle, descriptor) = context
            .devices()
            .or_else(|err| Err(format!("Failed to retrieve device list: {:?}", err)))
            .and_then(|list| {
                list.iter()
                    .map(|dev| dev.device_descriptor().map(|descriptor| (dev, descriptor)))
                    .filter_map(|dev| dev.ok())
                    .find(|(_, desc)| {
                        desc.vendor_id() == KRONEUM_VID && desc.product_id() == KRONEUM_PID
                    })
                    .ok_or_else(|| "Failed to find LibUSB device.".to_string())
            })
            .and_then(|(dev, desc)| {
                dev.open()
                    .or_else(|err| Err(format!("Failed to open device: {:?}", err)))
                    .map(|handle| (dev, handle, desc))
            })?;

        handle
            .kernel_driver_active(INTERFACE)
            .or_else(|err| {
                Err(format!(
                    "Failed to determine kernel driver status {:?}",
                    err
                ))
            })
            .and_then(|detached_kernel_driver| {
                if detached_kernel_driver {
                    handle
                        .detach_kernel_driver(INTERFACE)
                        .or_else(|err| Err(format!("Failed to detach kernel driver: {:?}", err)))
                        .map(|_| detached_kernel_driver)
                } else {
                    Ok(detached_kernel_driver)
                }
            })
            .and_then(|detached_kernel_driver| {
                handle
                    .claim_interface(INTERFACE)
                    .or_else(|err| {
                        Err(format!(
                            "Failed to claim interface {}: {:?}",
                            INTERFACE, err
                        ))
                    })
                    .map(|_| DeviceLibUSB {
                        device,
                        descriptor,
                        handle,
                        detached_kernel_driver,
                    })
            })
    }

    fn get_lang(&self) -> Result<libusb::Language, String> {
        self.handle
            .read_languages(Duration::from_secs(5))
            .or_else(|err| Err(format!("Failed to retrieve device languages: {:?}", err)))
            .and_then(|languages| {
                languages
                    .first()
                    .ok_or_else(|| "No languages were returned from device.".to_string())
                    .map(|lang| *lang)
            })
    }

    pub fn close(&mut self) -> Result<(), String> {
        self.handle
            .release_interface(INTERFACE)
            .or_else(|err| {
                Err(format!(
                    "Failed to release interface {}: {:?}",
                    INTERFACE, err
                ))
            })
            .and_then(|_| {
                if self.detached_kernel_driver {
                    self.handle
                        .attach_kernel_driver(INTERFACE)
                        .or_else(|err| Err(format!("Failed to attach kernel driver: {:?}", err)))
                } else {
                    Ok(())
                }
            })
    }
}

impl<'a> Device for DeviceLibUSB<'a> {
    fn get_identifier(&self) -> DeviceIdentifier {
        DeviceIdentifier {
            bus: self.device.bus_number(),
            address: self.device.address(),
            vendor_id: self.descriptor.vendor_id(),
            product_id: self.descriptor.product_id(),
        }
    }

    fn get_manufacturer(&self) -> Result<String, String> {
        self.get_lang().and_then(|lang| {
            self.handle
                .read_manufacturer_string(lang, &self.descriptor, Duration::from_secs(5))
                .or_else(|err| Err(format!("Failed to retrieve device manufacturer: {:?}", err)))
        })
    }

    fn write(&self, data: &[u8]) -> Result<(), String> {
        self.handle
            .write_interrupt(1, data, Duration::from_secs(5))
            .or_else(|err| Err(format!("Failed to send data to device endpoint: {:?}", err)))
            .map(|_| ())
    }

    fn read(&self) -> Result<(usize, [u8; REPORT_SIZE]), String> {
        let mut data = [0; REPORT_SIZE];
        self.handle
            .read_interrupt(0x81, &mut data, Duration::from_secs(5))
            .or_else(|err| Err(format!("Failed to read data to device endpoint: {:?}", err)))
            .map(|count| (count, data))
    }
}
