use clap::{App, Arg, SubCommand};
use std::time::Duration;

const KRONEUM_VID: u16 = 0xffff;
const KRONEUM_PID: u16 = 0xffff;
const INTERFACE: u8 = 0;

struct KroneumDevice<'a> {
    descriptor: libusb::DeviceDescriptor,
    device: libusb::Device<'a>,
    handle: libusb::DeviceHandle<'a>,
    detached_kernel_driver: bool,
}

impl<'a> KroneumDevice<'a> {
    fn open(context: &'a libusb::Context) -> Result<KroneumDevice<'a>, String> {
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

        Ok(KroneumDevice {
            device,
            descriptor,
            handle,
            detached_kernel_driver,
        })
    }

    fn get_bus_number(&self) -> u8 {
        self.device.bus_number()
    }

    fn get_address(&self) -> u8 {
        self.device.address()
    }

    fn get_vendor_id(&self) -> u16 {
        self.descriptor.vendor_id()
    }

    fn get_product_id(&self) -> u16 {
        self.descriptor.product_id()
    }

    fn close(&mut self) -> Result<(), String> {
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

    fn read_lang(&self) -> Result<libusb::Language, String> {
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

    fn read_manufacturer(&self, lang: libusb::Language) -> Result<String, String> {
        self.handle
            .read_manufacturer_string(lang, &self.descriptor, Duration::from_secs(5))
            .or_else(|err| Err(format!("Failed to retrieve device manufacturer: {:?}", err)))
    }

    fn send_data(&self, data: &[u8]) -> Result<(), String> {
        self.handle
            .write_interrupt(1, data, Duration::from_secs(5))
            .map(|_| ())
            .or_else(|err| Err(format!("Failed to send data to device endpoint: {:?}", err)))
    }
}

fn main() -> Result<(), String> {
    let matches = App::new("Kroneum CLI")
        .version("0.1.0")
        .author("Aleh Zasypkin <aleh.zasypkin@gmail.com>")
        .about("Allows to manage and configure Kroneum devices.")
        .subcommand(
            SubCommand::with_name("beep")
                .about("Makes Kroneum beep <NUMBER> of times")
                .arg(
                    Arg::with_name("NUMBER")
                        .short("n")
                        .long("number")
                        .takes_value(true)
                        .help("Defines number of beeps"),
                ),
        )
        .subcommand(SubCommand::with_name("info").about("Prints information about Kroneum device"))
        .subcommand(
            SubCommand::with_name("alarm")
                .about("Manages Kroneum alarm")
                .arg(
                    Arg::with_name("ACTION")
                        .index(1)
                        .possible_values(["set", "get"].as_ref())
                        .help("Sets or gets Kroneum alarm"),
                )
                .arg(
                    Arg::with_name("ALARM")
                        .index(2)
                        .required_if("ACTION", "set")
                        .help("Alarm to set in the hh:mm:ss form."),
                ),
        )
        .get_matches();

    let mut context = libusb::Context::new()
        .or_else(|err| Err(format!("Failed to create context: {:?}", err)))?;

    context.set_log_level(libusb::LogLevel::Info);

    let mut kroneum = KroneumDevice::open(&context)?;

    if let Some(matches) = matches.subcommand_matches("beep") {
        let number_of_beeps: u8 = matches
            .value_of("NUMBER")
            .and_then(|number_str| number_str.parse::<u8>().ok())
            .unwrap_or_else(|| 1);

        kroneum.send_data([0, 0, number_of_beeps].as_ref())?;
    } else if let Some(_) = matches.subcommand_matches("info") {
        let lang = kroneum.read_lang()?;
        println!(
            "Kroneum \u{1f389}: \nBus: {:03} \nDevice: {:03} \nID: {:04x}:{:04x} \nSupported language: 0x{:04x} \nManufacturer: {}",
            kroneum.get_bus_number(),
            kroneum.get_address(),
            kroneum.get_vendor_id(),
            kroneum.get_product_id(),
            lang.lang_id(),
            kroneum.read_manufacturer(lang)?
        );
    } else if let Some(matches) = matches.subcommand_matches("alarm") {
        match matches.value_of("ACTION").unwrap_or_else(|| "get") {
            "set" => {
                let timer: Duration = matches
                    .value_of("ALARM")
                    .unwrap_or_else(|| "5s")
                    .parse::<humantime::Duration>()
                    .unwrap()
                    .into();

                let timer_as_sec = timer.as_secs();

                let hours = timer_as_sec / 3600;
                let minutes = (timer_as_sec - 3600 * hours) / 60;
                let seconds = timer_as_sec - 3600 * hours - 60 * minutes;

                kroneum.send_data([1, 0, hours as u8, minutes as u8, seconds as u8].as_ref())?;
            }
            _ => {}
        }
    }

    kroneum.close()
}
