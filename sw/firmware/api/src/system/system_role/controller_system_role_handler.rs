use super::super::{system_role::SystemRole, System, SystemHardware, SystemInfo};
use array::Array;
use bare_metal::CriticalSection;
use beeper::melody::Melody;
use buttons::ButtonPressType;
use systick::SysTickHardware;
use usb::{
    command_packet::CommandPacket,
    commands::{
        ADCCommand, AlarmCommand, BeeperCommand, FlashCommand, KeyboardCommand, RadioCommand,
        SystemCommand,
    },
    endpoint::DeviceEndpoint,
};

pub struct ControllerSystemRoleHandler;
impl ControllerSystemRoleHandler {
    pub fn on_usb_packet<T: SystemHardware, S: SysTickHardware>(
        system: &mut System<T, S>,
        cs: &CriticalSection,
    ) {
        system.usb().interrupt();

        match system.state.peripherals_states.usb.command {
            Some(CommandPacket::Beeper(command)) => {
                match command {
                    BeeperCommand::Beep(n_beeps) => {
                        system
                            .beeper()
                            .play_and_repeat(Melody::Beep, n_beeps as usize);
                    }
                    BeeperCommand::Melody(tones) => {
                        system.beeper().play(Melody::Custom(tones));
                    }
                };

                system.usb().send(DeviceEndpoint::System, &[0x00]);
            }
            Some(CommandPacket::Alarm(command)) => {
                if let AlarmCommand::Get = command {
                    let alarm = system.rtc().alarm();
                    system.usb().send(
                        DeviceEndpoint::System,
                        &[0x00, alarm.hours, alarm.minutes, alarm.seconds],
                    );
                } else if let AlarmCommand::Set(_time) = command {
                    // We should send OK response before we enter Alarm mode and USB will be disabled.
                    system.usb().send(DeviceEndpoint::System, &[0x00]);
                    system.systick.delay(100);
                    system.usb().send(DeviceEndpoint::System, &[0xFF]);
                } else {
                    system.usb().send(DeviceEndpoint::System, &[0xFF]);
                }
            }
            Some(CommandPacket::System(command)) => {
                if let SystemCommand::Echo(mut echo_data) = command {
                    echo_data.unshift(0x00);
                    system
                        .usb()
                        .send(DeviceEndpoint::System, echo_data.as_ref());
                } else if let SystemCommand::Reset = command {
                    // We should send OK response before we reset.
                    system.usb().send(DeviceEndpoint::System, &[0x00]);
                    system.systick.delay(100);
                    system.reset();
                } else if let SystemCommand::GetInfo = command {
                    let mut array: Array<u8> = (SystemInfo {
                        id: *system.hw.device_id(),
                        flash_size_kb: system.hw.flash_size_kb(),
                    })
                    .into();
                    array.unshift(0x00);
                    system.usb().send(DeviceEndpoint::System, array.as_ref());
                } else {
                    system.usb().send(DeviceEndpoint::System, &[0xFF]);
                }
            }
            Some(CommandPacket::Flash(command)) => {
                let response = match command {
                    FlashCommand::Read(storage_slot) => Ok(Array::from(
                        [system.flash().read(storage_slot).unwrap_or_else(|| 0)].as_ref(),
                    )),
                    FlashCommand::Write(storage_slot, value) => system
                        .flash()
                        .write(storage_slot, value)
                        .map(|_| Array::new()),
                    FlashCommand::EraseAll => {
                        system.flash().erase_all();
                        Ok(Array::new())
                    }
                };

                match response {
                    Ok(mut array) => {
                        array.unshift(0x00);
                        system.usb().send(DeviceEndpoint::System, array.as_ref());
                    }
                    Err(_) => system.usb().send(DeviceEndpoint::System, &[0xFF]),
                };
            }
            Some(CommandPacket::ADC(command)) => {
                let response = match command {
                    ADCCommand::Read(channel) => {
                        let value = system.adc().read(channel);
                        Array::from(&[0x00, (value & 0xff) as u8, ((value & 0xff00) >> 8) as u8])
                    }
                };

                system.usb().send(DeviceEndpoint::System, response.as_ref());
            }
            Some(CommandPacket::Radio(command)) => {
                let response = match command {
                    RadioCommand::Transmit(data) => {
                        system.radio().transmit(cs, data).map(|_| Array::new())
                    }
                    RadioCommand::Receive => system.radio().receive(cs),
                    RadioCommand::Status => system.radio().status(cs),
                };

                match response {
                    Ok(mut array) => {
                        array.unshift(0x00);
                        system.usb().send(DeviceEndpoint::System, array.as_ref());
                    }
                    Err(_) => system.usb().send(DeviceEndpoint::System, &[0xFF]),
                };
            }
            Some(CommandPacket::Keyboard(command)) => match command {
                KeyboardCommand::Key(modifiers, key_code, delay) => {
                    if delay > 0 {
                        system.systick.delay(delay as u32 * 1000);
                    }

                    system.usb().send(
                        DeviceEndpoint::Keyboard,
                        &[0x01, modifiers.into(), 0, key_code, 0, 0, 0, 0, 0],
                    );
                    system.systick.delay(10);
                    system
                        .usb()
                        .send(DeviceEndpoint::Keyboard, &[0x01, 0, 0, 0, 0, 0, 0, 0, 0]);

                    system.usb().send(DeviceEndpoint::System, &[0x00]);
                }
                KeyboardCommand::Media(key_code, delay) => {
                    if delay > 0 {
                        system.systick.delay(delay as u32 * 1000);
                    }

                    system
                        .usb()
                        .send(DeviceEndpoint::Keyboard, &[0x02, key_code as u8]);
                    system.systick.delay(10);
                    system.usb().send(DeviceEndpoint::Keyboard, &[0x02, 0x0]);

                    system.usb().send(DeviceEndpoint::System, &[0x00]);
                }
            },
            _ => {}
        }

        system.state.peripherals_states.usb.command = None;
    }

    pub fn on_buttons_press<T: SystemHardware, S: SysTickHardware>(
        system: &mut System<T, S>,
        buttons_press_type: (ButtonPressType, ButtonPressType),
    ) {
        if let (ButtonPressType::Long, ButtonPressType::Long) = buttons_press_type {
            system.beeper().play(Melody::Reset);

            system.usb().teardown();

            system.switch_to_role(SystemRole::Timer);
        }
    }
}
