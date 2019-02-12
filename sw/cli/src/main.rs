mod device;
mod device_hidapi;

use clap::{App, Arg, SubCommand};
use device::Device;
use device_hidapi::DeviceHIDAPI;
use std::time::Duration;

// 24 hours.
const MAX_ALARM_SECONDS: u32 = 3600 * 24;

fn on_get_info(device: &impl Device) -> Result<(), String> {
    println!(
        "Kroneum ({}):\nManufacturer: {}",
        device.get_identifier(),
        device.get_manufacturer()?
    );

    Ok(())
}

fn on_beep(device: &impl Device, number_of_beeps: u8) -> Result<(), String> {
    device.send_data([0, 0, number_of_beeps].as_ref())
}

fn on_set_alarm(device: &impl Device, alarm_in_seconds: u32) -> Result<(), String> {
    if alarm_in_seconds >= MAX_ALARM_SECONDS {
        return Err("Alarm is limited to 23h 59m 59s".to_string());
    }

    let hours = alarm_in_seconds / 3600;
    let minutes = (alarm_in_seconds - 3600 * hours) / 60;
    let seconds = alarm_in_seconds - 3600 * hours - 60 * minutes;

    device.send_data([1, 0, hours as u8, minutes as u8, seconds as u8].as_ref())
}

fn on_get_alarm(device: &impl Device) -> Result<(), String> {
    device.send_data([2].as_ref())?;
    let (_, data) = device.read_data()?;

    println!(
        "Current alarm is set to: {}h {}m {}s",
        data[0], data[1], data[2]
    );

    Ok(())
}

fn main() -> Result<(), String> {
    let matches = App::new("Kroneum CLI")
        .version("0.1.0")
        .author("Aleh Zasypkin <aleh.zasypkin@gmail.com>")
        .about("Allows to manage and configure Kroneum devices.")
        .arg(
            Arg::with_name("hidapi")
                .long("hid-api")
                .short("h")
                .help("Uses HIDAPI instead of LibUSB.")
                .takes_value(false),
        )
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
                        .default_value("5s")
                        .required_if("ACTION", "set")
                        .help("Alarm to set in the hh:mm:ss form."),
                ),
        )
        .get_matches();

    let device = DeviceHIDAPI::open()?;

    if let Some(matches) = matches.subcommand_matches("beep") {
        let number_of_beeps: u8 = matches
            .value_of("NUMBER")
            .and_then(|number_str| number_str.parse::<u8>().ok())
            .unwrap_or_else(|| 1);

        on_beep(&device, number_of_beeps)?;
    } else if let Some(_) = matches.subcommand_matches("info") {
        on_get_info(&device)?;
    } else if let Some(matches) = matches.subcommand_matches("alarm") {
        match matches.value_of("ACTION").unwrap_or_else(|| "get") {
            "set" => {
                let timer: Duration = matches
                    .value_of("ALARM")
                    .unwrap()
                    .parse::<humantime::Duration>()
                    .unwrap()
                    .into();

                on_set_alarm(&device, timer.as_secs() as u32)?;
            }
            "get" => {
                on_get_alarm(&device)?;
            }
            _ => {}
        }
    }

    Ok(())
}
