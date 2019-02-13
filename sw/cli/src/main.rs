mod device;
mod device_hidapi;

use clap::{App, Arg, SubCommand};
use device::Device;
use device_hidapi::DeviceHIDAPI;
use std::time::Duration;

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
        device.beep(
            matches
                .value_of("NUMBER")
                .and_then(|number_str| number_str.parse::<u8>().ok())
                .unwrap_or_else(|| 1),
        )?;
    } else if let Some(_) = matches.subcommand_matches("info") {
        println!(
            "Kroneum ({}):\nManufacturer: {}",
            device.get_identifier(),
            device.get_manufacturer()?
        );
    } else if let Some(matches) = matches.subcommand_matches("alarm") {
        match matches.value_of("ACTION").unwrap_or_else(|| "get") {
            "set" => {
                device.set_alarm(
                    matches
                        .value_of("ALARM")
                        .ok_or_else(|| "<ALARM> argument is not provided.".to_string())
                        .and_then(|alarm_str| {
                            alarm_str.parse::<humantime::Duration>().or_else(|err| {
                                Err(format!("Failed to parse <ALARM> argument: {:?}", err))
                            })
                        })
                        .map(|alarm_human| {
                            let duration: Duration = alarm_human.into();
                            duration
                        })?,
                )?;
            }
            "get" => {
                println!(
                    "Current alarm is set to: {}",
                    humantime::Duration::from(device.get_alarm()?)
                );
            }
            _ => {}
        }
    }

    Ok(())
}
