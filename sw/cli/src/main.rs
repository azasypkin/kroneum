mod device;
mod device_hidapi;
mod device_libusb;

use clap::{App, Arg, ArgMatches, SubCommand};
use device::{Device, DeviceContext};
use device_hidapi::DeviceContextHIDAPI;
use device_libusb::DeviceContextLibUSB;
use std::time::Duration;

fn run_command<'a, T: Device>(
    matches: ArgMatches,
    context: &'a impl DeviceContext<'a, T>,
) -> Result<(), String> {
    let device = context.open()?;

    match matches.subcommand() {
        ("beep", Some(matches)) => {
            device.beep(
                matches
                    .value_of("NUMBER")
                    .ok_or_else(|| "<NUMBER> argument is not provided.".to_string())
                    .and_then(|number_str| {
                        number_str.parse::<u8>().or_else(|err| {
                            Err(format!("Failed to parse <NUMBER> argument: {:?}", err))
                        })
                    })?,
            )?;
        }
        ("info", _) => {
            println!(
                "Kroneum ({}):\nManufacturer: {}",
                device.get_identifier(),
                device.get_manufacturer()?
            );
        }
        ("alarm", Some(matches)) => match matches.value_of("ACTION").unwrap_or_else(|| "get") {
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
        },

        ("reset", _) => {
            println!("Device is being reset...");
            device.reset()?
        }
        _ => return Err("Unknown sub-command!".to_string()),
    }

    context.close(device)
}

fn main() -> Result<(), String> {
    let matches = App::new("Kroneum CLI")
        .version("0.1.0")
        .author("Aleh Zasypkin <aleh.zasypkin@gmail.com>")
        .about("Allows to manage and configure Kroneum devices.")
        .arg(
            Arg::with_name("libusb")
                .long("libusb")
                .short("l")
                .help("Uses LibUSB instead of HIDAPI.")
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
                        .default_value("1")
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
        .subcommand(SubCommand::with_name("reset").about("Resets Kroneum device"))
        .get_matches();

    if matches.is_present("libusb") {
        run_command(matches, &DeviceContextLibUSB::create()?)
    } else {
        run_command(matches, &DeviceContextHIDAPI::create()?)
    }
}
