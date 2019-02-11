use std::fmt;
use std::time::Duration;

const KRONEUM_VID: u16 = 0xffff;
const KRONEUM_PID: u16 = 0xffff;
const INTERFACE: u8 = 0;

/// Describes main parameters of the Kroneum device.
pub(crate) struct DeviceIdentifier {
    bus: u8,
    address: u8,
    vendor_id: u16,
    product_id: u16,
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

pub(crate) struct Device<'a> {
    descriptor: libusb::DeviceDescriptor,
    device: libusb::Device<'a>,
    handle: libusb::DeviceHandle<'a>,
    detached_kernel_driver: bool,
}

impl<'a> Device<'a> {
    pub fn open(context: &'a libusb::Context) -> Result<Device<'a>, String> {
        let devices = context
            .devices()
            .or_else(|err| Err(format!("Failed to retrieve device list: {:?}", err)))?;

        let mut device_and_descriptor: Option<(libusb::Device, libusb::DeviceDescriptor)> = None;
        for device in devices.iter() {
            let device_descriptor = device
                .device_descriptor()
                .or_else(|err| Err(format!("Failed to retrieve device descriptor: {:?}", err)))?;

            if device_descriptor.vendor_id() == KRONEUM_VID
                && device_descriptor.product_id() == KRONEUM_PID
            {
                device_and_descriptor.replace((device, device_descriptor));
                break;
            }
        }

        let (device, descriptor) = device_and_descriptor.ok_or_else(|| {
            format!(
                "Couldn't find device with VID `0x{:04x}` and PID `0x{:04x}`.",
                KRONEUM_VID, KRONEUM_PID
            )
        })?;

        let (mut handle, detached_kernel_driver) = device
            .open()
            .or_else(|err| Err(format!("Failed to open device: {:?}", err)))
            .and_then(|mut device_handle| {
                let detached_kernel_driver =
                    device_handle.kernel_driver_active(0).or_else(|err| {
                        Err(format!(
                            "Failed to determine kernel driver status {:?}",
                            err
                        ))
                    })?;

                if detached_kernel_driver {
                    device_handle
                        .detach_kernel_driver(INTERFACE)
                        .or_else(|err| Err(format!("Failed to detach kernel driver: {:?}", err)))?;
                }

                Ok((device_handle, detached_kernel_driver))
            })?;

        handle
            .claim_interface(INTERFACE)
            .or_else(|err| Err(format!("Failed to claim interface 0: {:?}", err)))?;

        Ok(Device {
            device,
            descriptor,
            handle,
            detached_kernel_driver,
        })
    }

    pub fn get_identifier(&self) -> DeviceIdentifier {
        DeviceIdentifier {
            bus: self.device.bus_number(),
            address: self.device.address(),
            vendor_id: self.descriptor.vendor_id(),
            product_id: self.descriptor.product_id(),
        }
    }

    pub fn close(&mut self) -> Result<(), String> {
        self.handle
            .release_interface(INTERFACE)
            .or_else(|err| Err(format!("Failed to release interface 0: {:?}", err)))?;

        if self.detached_kernel_driver {
            self.handle
                .attach_kernel_driver(INTERFACE)
                .or_else(|err| Err(format!("Failed to attach kernel driver: {:?}", err)))
        } else {
            Ok(())
        }
    }

    pub fn read_lang(&self) -> Result<libusb::Language, String> {
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

    pub fn read_manufacturer(&self, lang: libusb::Language) -> Result<String, String> {
        self.handle
            .read_manufacturer_string(lang, &self.descriptor, Duration::from_secs(5))
            .or_else(|err| Err(format!("Failed to retrieve device manufacturer: {:?}", err)))
    }

    pub fn send_data(&self, data: &[u8]) -> Result<(), String> {
        self.handle
            .write_interrupt(1, data, Duration::from_secs(5))
            .map(|_| ())
            .or_else(|err| Err(format!("Failed to send data to device endpoint: {:?}", err)))
    }

    pub fn read_data(&self) -> Result<(usize, [u8;6]), String> {
        let mut data = [1;6];
        let count = self.handle
            .read_interrupt(0x81, &mut data, Duration::from_secs(5))
            .or_else(|err| Err(format!("Failed to read data to device endpoint: {:?}", err)))?;

        Ok((count, data))
    }
}
