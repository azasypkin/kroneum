use crate::device::{Device, DeviceContext, DeviceIdentifier};
use kroneum_api::{
    config::{DEVICE_PID, DEVICE_VID},
    usb::command_packet::{CommandByteSequence, CommandPacket},
};
use std::time::Duration;

const INTERFACE: u8 = 0;

pub struct DeviceContextLibUSB {
    context: libusb::Context,
}

impl<'a: 'b, 'b> DeviceContext<'a, DeviceLibUSB<'b>> for DeviceContextLibUSB {
    fn create() -> Result<Self, String> {
        libusb::Context::new()
            .or_else(|err| Err(format!("Failed to create LibUSB context {:?}", err)))
            .map(|context| DeviceContextLibUSB { context })
    }

    fn open(&'a self) -> Result<DeviceLibUSB<'b>, String> {
        DeviceLibUSB::open(&self.context)
    }

    fn close(&'a self, mut device: DeviceLibUSB<'b>) -> Result<(), String> {
        device.close()
    }
}

pub struct DeviceLibUSB<'a> {
    descriptor: libusb::DeviceDescriptor,
    device: libusb::Device<'a>,
    handle: libusb::DeviceHandle<'a>,
    detached_kernel_driver: bool,
}

impl<'a> DeviceLibUSB<'a> {
    fn open(context: &'a libusb::Context) -> Result<Self, String> {
        let (device, mut handle, descriptor) = context
            .devices()
            .or_else(|err| Err(format!("Failed to retrieve device list: {:?}", err)))
            .and_then(|list| {
                list.iter()
                    .map(|dev| dev.device_descriptor().map(|descriptor| (dev, descriptor)))
                    .filter_map(|dev| dev.ok())
                    .find(|(_, desc)| {
                        desc.vendor_id() == DEVICE_VID && desc.product_id() == DEVICE_PID
                    })
                    .ok_or_else(|| "Failed to find LibUSB device.".to_string())
            })
            .and_then(|(dev, desc)| {
                dev.open()
                    .or_else(|err| Err(format!("Failed to open device: {:?}", err)))
                    .map(|handle| (dev, handle, desc))
            })?;

        let detached_kernel_driver = if context.supports_detach_kernel_driver() {
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
                            .or_else(|err| {
                                Err(format!("Failed to detach kernel driver: {:?}", err))
                            })
                            .map(|_| detached_kernel_driver)
                    } else {
                        Ok(detached_kernel_driver)
                    }
                })?
        } else {
            false
        };

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

    fn close(&mut self) -> Result<(), String> {
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

    fn write(&self, packet: CommandPacket) -> Result<(), String> {
        self.handle
            .write_interrupt(1, &packet.to_bytes(), Duration::from_secs(5))
            .or_else(|err| Err(format!("Failed to send data to device endpoint: {:?}", err)))
            .map(|_| ())
    }

    fn read(&self) -> Result<(usize, CommandByteSequence), String> {
        let mut data = CommandByteSequence::default();
        self.handle
            .read_interrupt(0x81, &mut data, Duration::from_secs(5))
            .or_else(|err| Err(format!("Failed to read data to device endpoint: {:?}", err)))
            .map(|count| (count, data))
    }
}
