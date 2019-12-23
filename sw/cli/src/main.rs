mod device;
mod ui;

use clap::{App, Arg, ArgMatches, SubCommand};
use device::Device;
use kroneum_api::flash::storage_slot::StorageSlot;
use std::time::Duration;

fn process_command(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("beep", Some(matches)) => {
            Device::create()?.beep(
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
            let device = Device::create()?;
            println!("Kroneum ({})", device.get_identifier()?,);
        }
        ("alarm", Some(matches)) => match matches.value_of("ACTION").unwrap_or_else(|| "get") {
            "set" => {
                Device::create()?.set_alarm(
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
                    humantime::Duration::from(Device::create()?.get_alarm()?)
                );
            }
            _ => {}
        },
        ("flash", Some(matches)) => match matches.value_of("ACTION").unwrap() {
            "erase" => {
                Device::create()?.erase_flash()?;
                println!("Flash is erased");
            }
            operation => {
                let device = Device::create()?;
                let slot: StorageSlot = matches
                    .value_of("SLOT")
                    .ok_or_else(|| "<SLOT> argument is not provided.".to_string())
                    .and_then(|slot_str| {
                        u8::from_str_radix(&slot_str[2..], 16).or_else(|err| {
                            Err(format!("Failed to parse <SLOT> argument: {:?}", err))
                        })
                    })?
                    .into();

                match operation {
                    "write" => {
                        let value = matches
                            .value_of("VALUE")
                            .ok_or_else(|| "<VALUE> argument is not provided.".to_string())
                            .and_then(|value_str| {
                                u8::from_str_radix(value_str, 10).or_else(|err| {
                                    Err(format!("Failed to parse <VALUE> argument: {:?}", err))
                                })
                            })?;

                        device.write_flash(slot, value)?;

                        println!("Value {} is written to memory.", value);
                    }
                    "read" => println!("Value read from memory: {}", device.read_flash(slot)?),
                    _ => {}
                }

                device.read_flash(slot)?;
            }
        },

        ("reset", _) => {
            println!("Device is being reset...");
            Device::create()?.reset()?
        }

        ("ui", Some(matches)) => ui::start_server(
            matches
                .value_of("PORT")
                .ok_or_else(|| "<PORT> argument is not provided.".to_string())
                .and_then(|number_str| {
                    number_str
                        .parse::<u16>()
                        .or_else(|err| Err(format!("Failed to parse <PORT> argument: {:?}", err)))
                })?,
        )?,
        _ => return Err("Unknown sub-command!".to_string()),
    };

    Ok(())
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
        .subcommand(
            SubCommand::with_name("flash")
                .about("Manages Kroneum flash memory")
                .arg(
                    Arg::with_name("ACTION")
                        .index(1)
                        .required(true)
                        .possible_values(["read", "write", "erase"].as_ref())
                        .help("Reads from, writes to or erases Kroneum flash memory"),
                )
                .arg(
                    Arg::with_name("SLOT")
                        .index(2)
                        .required_ifs(&[("ACTION", "read"), ("ACTION", "write")])
                        .possible_values(&["0x1f", "0x2f", "0x3f", "0x4f", "0x5f"])
                        .help("Address of the memory slot."),
                )
                .arg(
                    Arg::with_name("VALUE")
                        .index(3)
                        .required_if("ACTION", "write")
                        .help("Value to write to a memory slot. Value must be an unsigned byte."),
                ),
        )
        .subcommand(SubCommand::with_name("reset").about("Resets Kroneum device"))
        .subcommand(
            SubCommand::with_name("ui")
                .about("Starts a web server with UI to manage Kroneum")
                .arg(
                    Arg::with_name("PORT")
                        .short("p")
                        .long("port")
                        .takes_value(true)
                        .default_value("8080")
                        .help("Defines a TCP port to listen on"),
                ),
        )
        .get_matches();

    process_command(matches)
}
