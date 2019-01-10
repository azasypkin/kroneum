use std::time::Duration;

const KRONEUM_VID: u16 = 0xffff;
const KRONEUM_PID: u16 = 0xffff;
const INTERFACE: u8 = 0;

fn main() -> Result<(), String> {
    let mut context = libusb::Context::new()
        .or_else(|err| Err(format!("Failed to create context: {:?}", err)))?;

    context.set_log_level(libusb::LogLevel::Debug);

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

    let (device, device_descriptor) = device_and_descriptor.ok_or_else(|| {
        format!(
            "Couldn't find device with VID `0x{:04x}` and PID `0x{:04x}`.",
            KRONEUM_VID, KRONEUM_PID
        )
    })?;

    println!(
        "Found Kroneum device: bus {:03} device {:03} ID {:04x}:{:04x}",
        device.bus_number(),
        device.address(),
        device_descriptor.vendor_id(),
        device_descriptor.product_id()
    );

    let (mut device_handle, detached_kernel_driver) = device
        .open()
        .or_else(|err| Err(format!("Failed to retrieve device descriptor: {:?}", err)))
        .and_then(|mut device_handle| {
            let detached_kernel_driver = device_handle.kernel_driver_active(0).or_else(|err| {
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

    device_handle
        .claim_interface(INTERFACE)
        .or_else(|err| Err(format!("Failed to claim interface 0: {:?}", err)))?;

    let lang = device_handle
        .read_languages(Duration::from_secs(5))
        .or_else(|err| Err(format!("Failed to retrieve device languages: {:?}", err)))
        .and_then(|languages| {
            languages
                .first()
                .ok_or_else(|| format!("No languages were returned from device."))
                .map(|lang| *lang)
        })?;

    println!("Supported device language: 0x{:04x}", lang.lang_id());

    let manufacturer = device_handle
        .read_manufacturer_string(lang, &device_descriptor, Duration::from_secs(5))
        .or_else(|err| Err(format!("Failed to retrieve device manufacturer: {:?}", err)))?;

    println!("Device manufacturer: {}", manufacturer);

    //device_handle.write_interrupt(81, [1, 2, 3].as_ref(), Duration::from_secs(5)).unwrap();

    if detached_kernel_driver {
        device_handle
            .attach_kernel_driver(INTERFACE)
            .or_else(|err| Err(format!("Failed to retrieve device manufacturer: {:?}", err)))?
    }

    Ok(())
}
