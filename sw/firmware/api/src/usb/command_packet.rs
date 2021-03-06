use super::commands::{
    ADCCommand, AlarmCommand, BeeperCommand, FlashCommand, KeyboardCommand, RadioCommand,
    SystemCommand,
};
use array::Array;
use core::convert::TryFrom;
use usb::usb_error::USBError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CommandPacket {
    Beeper(BeeperCommand),
    ADC(ADCCommand),
    Alarm(AlarmCommand),
    Flash(FlashCommand),
    System(SystemCommand),
    Radio(RadioCommand),
    Keyboard(KeyboardCommand),
}

impl From<CommandPacket> for Array<u8> {
    fn from(packet: CommandPacket) -> Self {
        match packet {
            CommandPacket::Beeper(command) => {
                let mut array = Array::from(command);
                array.unshift(0x1);
                array
            }
            CommandPacket::Alarm(command) => {
                let mut array = Array::from(command);
                array.unshift(0x2);
                array
            }
            CommandPacket::System(command) => {
                let mut array = Array::from(command);
                array.unshift(0x3);
                array
            }
            CommandPacket::Flash(command) => {
                let mut array = Array::from(command);
                array.unshift(0x4);
                array
            }
            CommandPacket::ADC(command) => {
                let mut array = Array::from(command);
                array.unshift(0x5);
                array
            }
            CommandPacket::Radio(command) => {
                let mut array = Array::from(command);
                array.unshift(0x6);
                array
            }
            CommandPacket::Keyboard(command) => {
                let mut array = Array::from(command);
                array.unshift(0x7);
                array
            }
        }
    }
}

impl TryFrom<Array<u8>> for CommandPacket {
    type Error = USBError;

    fn try_from(mut value: Array<u8>) -> Result<Self, Self::Error> {
        match value.shift() {
            Some(0x1) => Ok(CommandPacket::Beeper(BeeperCommand::try_from(value)?)),
            Some(0x2) => Ok(CommandPacket::Alarm(AlarmCommand::try_from(value)?)),
            Some(0x3) => Ok(CommandPacket::System(SystemCommand::try_from(value)?)),
            Some(0x4) => Ok(CommandPacket::Flash(FlashCommand::try_from(value)?)),
            Some(0x5) => Ok(CommandPacket::ADC(ADCCommand::try_from(value)?)),
            Some(0x6) => Ok(CommandPacket::Radio(RadioCommand::try_from(value)?)),
            Some(0x7) => Ok(CommandPacket::Keyboard(KeyboardCommand::try_from(value)?)),
            _ => Err(USBError::InvalidCommand),
        }
    }
}

impl TryFrom<&[u8]> for CommandPacket {
    type Error = USBError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Array::from(slice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adc::ADCChannel;
    use beeper::{note::Note, tone::Tone};
    use flash::storage_slot::StorageSlot;
    use time::Time;
    use usb::commands::KeyModifiers;

    #[test]
    fn beeper_command() {
        assert_eq!(
            CommandPacket::try_from([1, 1, 1].as_ref()),
            Ok(CommandPacket::Beeper(BeeperCommand::Beep(1)))
        );
        assert_eq!(
            CommandPacket::try_from([1, 1, 15].as_ref()),
            Ok(CommandPacket::Beeper(BeeperCommand::Beep(15)))
        );

        assert_eq!(
            Array::from(CommandPacket::Beeper(BeeperCommand::Beep(1))).as_ref(),
            [1, 1, 1]
        );
        assert_eq!(
            Array::from(CommandPacket::Beeper(BeeperCommand::Beep(15))).as_ref(),
            [1, 1, 15]
        );

        let mut array: Array<Tone> = Array::new();
        array.push(Tone::new(Note::A5 as u8, 100));
        array.push(Tone::new(Note::B5 as u8, 50));
        assert_eq!(
            CommandPacket::try_from([1, 2, 0xA5, 100, 0xC5, 50].as_ref()),
            Ok(CommandPacket::Beeper(BeeperCommand::Melody(array)))
        );

        assert_eq!(
            Array::from(CommandPacket::Beeper(BeeperCommand::Melody(array))).as_ref(),
            [1, 2, 0xA5, 100, 0xC5, 50]
        );
    }

    #[test]
    fn alarm_command() {
        assert_eq!(
            CommandPacket::try_from([2, 1].as_ref()),
            Ok(CommandPacket::Alarm(AlarmCommand::Get))
        );

        assert_eq!(
            Array::from(CommandPacket::Alarm(AlarmCommand::Get)).as_ref(),
            [2, 1]
        );

        assert_eq!(
            CommandPacket::try_from([2, 2, 18, 33, 17].as_ref()),
            Ok(CommandPacket::Alarm(AlarmCommand::Set(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            })))
        );

        assert_eq!(
            CommandPacket::try_from([2, 2, 33, 18, 1].as_ref()),
            Ok(CommandPacket::Alarm(AlarmCommand::Set(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            })))
        );

        assert_eq!(
            Array::from(CommandPacket::Alarm(AlarmCommand::Set(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            })))
            .as_ref(),
            [2, 2, 18, 33, 17]
        );

        assert_eq!(
            Array::from(CommandPacket::Alarm(AlarmCommand::Set(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            })))
            .as_ref(),
            [2, 2, 33, 18, 1]
        );
    }

    #[test]
    fn system_command() {
        assert_eq!(
            CommandPacket::try_from([3, 1].as_ref()),
            Ok(CommandPacket::System(SystemCommand::Reset))
        );

        assert_eq!(
            Array::from(CommandPacket::System(SystemCommand::Reset)).as_ref(),
            [3, 1]
        );

        let array = Array::from(&[1, 2, 3, 10]);
        assert_eq!(
            CommandPacket::try_from([3, 2, 1, 2, 3, 10].as_ref()),
            Ok(CommandPacket::System(SystemCommand::Echo(array)))
        );

        assert_eq!(
            Array::from(CommandPacket::System(SystemCommand::Echo(array))).as_ref(),
            [3, 2, 1, 2, 3, 10]
        );
    }

    #[test]
    fn flash_command() {
        assert_eq!(
            CommandPacket::try_from([4, 1, 0xaf].as_ref()),
            Ok(CommandPacket::Flash(FlashCommand::Read(
                StorageSlot::Configuration
            )))
        );

        assert_eq!(
            Array::from(CommandPacket::Flash(FlashCommand::Read(
                StorageSlot::Configuration
            )))
            .as_ref(),
            [4, 1, 0xaf]
        );

        assert_eq!(
            CommandPacket::try_from([4, 2, 0xaf, 5].as_ref()),
            Ok(CommandPacket::Flash(FlashCommand::Write(
                StorageSlot::Configuration,
                5
            )))
        );

        assert_eq!(
            Array::from(CommandPacket::Flash(FlashCommand::Write(
                StorageSlot::Configuration,
                5
            )))
            .as_ref(),
            [4, 2, 0xaf, 5]
        );

        assert_eq!(
            CommandPacket::try_from([4, 3].as_ref()),
            Ok(CommandPacket::Flash(FlashCommand::EraseAll))
        );

        assert_eq!(
            Array::from(CommandPacket::Flash(FlashCommand::EraseAll)).as_ref(),
            [4, 3]
        );
    }

    #[test]
    fn adc_command() {
        assert_eq!(
            CommandPacket::try_from([5, 1, 1].as_ref()),
            Ok(CommandPacket::ADC(ADCCommand::Read(ADCChannel::Channel1)))
        );
        assert_eq!(
            CommandPacket::try_from([5, 1, 3].as_ref()),
            Ok(CommandPacket::ADC(ADCCommand::Read(ADCChannel::Channel3)))
        );
        assert_eq!(
            CommandPacket::try_from([5, 1, 7].as_ref()),
            Ok(CommandPacket::ADC(ADCCommand::Read(ADCChannel::Channel7)))
        );

        assert_eq!(
            CommandPacket::try_from([5, 0].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            CommandPacket::try_from([5, 2].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            CommandPacket::try_from([5, 8].as_ref()),
            Err(USBError::InvalidCommand)
        );

        assert_eq!(
            Array::from(CommandPacket::ADC(ADCCommand::Read(ADCChannel::Channel5))).as_ref(),
            [5, 1, 5]
        );
    }

    #[test]
    fn radio_command() {
        assert_eq!(
            CommandPacket::try_from([6, 1, 2].as_ref()),
            Ok(CommandPacket::Radio(RadioCommand::Transmit(Array::from(
                [2].as_ref()
            ))))
        );
        assert_eq!(
            CommandPacket::try_from([6, 2].as_ref()),
            Ok(CommandPacket::Radio(RadioCommand::Receive))
        );
        assert_eq!(
            CommandPacket::try_from([6, 3].as_ref()),
            Ok(CommandPacket::Radio(RadioCommand::Status))
        );

        assert_eq!(
            CommandPacket::try_from([6, 0].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            CommandPacket::try_from([6, 4].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            CommandPacket::try_from([6, 5].as_ref()),
            Err(USBError::InvalidCommand)
        );

        assert_eq!(
            Array::from(CommandPacket::Radio(RadioCommand::Transmit(Array::from(
                [2].as_ref()
            ))))
            .as_ref(),
            [6, 1, 2]
        );
    }

    #[test]
    fn keyboard_command() {
        assert_eq!(
            CommandPacket::try_from([7, 1, 1, 2, 3].as_ref()),
            Ok(CommandPacket::Keyboard(KeyboardCommand::Key(
                KeyModifiers {
                    left_ctrl: true,
                    left_shift: false,
                    left_alt: false,
                    left_gui: false,
                    right_ctrl: false,
                    right_shift: false,
                    right_alt: false,
                    right_gui: false
                },
                2,
                3
            )))
        );
        assert_eq!(
            CommandPacket::try_from([7, 1, 3, 2, 1].as_ref()),
            Ok(CommandPacket::Keyboard(KeyboardCommand::Key(
                KeyModifiers {
                    left_ctrl: true,
                    left_shift: true,
                    left_alt: false,
                    left_gui: false,
                    right_ctrl: false,
                    right_shift: false,
                    right_alt: false,
                    right_gui: false
                },
                2,
                1
            )))
        );
        assert_eq!(
            CommandPacket::try_from([7, 1, 2, 3, 1].as_ref()),
            Ok(CommandPacket::Keyboard(KeyboardCommand::Key(
                KeyModifiers {
                    left_ctrl: false,
                    left_shift: true,
                    left_alt: false,
                    left_gui: false,
                    right_ctrl: false,
                    right_shift: false,
                    right_alt: false,
                    right_gui: false
                },
                3,
                1
            )))
        );

        assert_eq!(
            CommandPacket::try_from([7].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            CommandPacket::try_from([7, 1].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            CommandPacket::try_from([7, 1, 2].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            CommandPacket::try_from([7, 1, 2, 3].as_ref()),
            Err(USBError::InvalidCommand)
        );

        assert_eq!(
            Array::from(CommandPacket::Keyboard(KeyboardCommand::Key(
                KeyModifiers {
                    left_ctrl: true,
                    left_shift: false,
                    left_alt: false,
                    left_gui: false,
                    right_ctrl: false,
                    right_shift: false,
                    right_alt: false,
                    right_gui: false
                },
                2,
                3
            )))
            .as_ref(),
            [7, 1, 1, 2, 3]
        );
    }

    #[test]
    fn invalid_command() {
        assert_eq!(
            CommandPacket::try_from([0].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            CommandPacket::try_from([8].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            CommandPacket::try_from([9, 10, 11].as_ref()),
            Err(USBError::InvalidCommand)
        );
    }
}
