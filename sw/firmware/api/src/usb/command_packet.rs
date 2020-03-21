use super::commands::{
    ADCCommand, AlarmCommand, BeeperCommand, FlashCommand, RadioCommand, SystemCommand,
};
use array::Array;

impl From<CommandPacket> for Array<u8> {
    fn from(packet: CommandPacket) -> Self {
        let array: Array<u8> = match packet {
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
            CommandPacket::Unknown => Array::from([0].as_ref()),
        };

        array.into()
    }
}

impl Into<CommandPacket> for Array<u8> {
    fn into(mut self) -> CommandPacket {
        let command_type_byte = self.shift();
        match command_type_byte {
            Some(0x1) => CommandPacket::Beeper(self.into()),
            Some(0x2) => CommandPacket::Alarm(self.into()),
            Some(0x3) => CommandPacket::System(self.into()),
            Some(0x4) => CommandPacket::Flash(self.into()),
            Some(0x5) => CommandPacket::ADC(self.into()),
            Some(0x6) => CommandPacket::Radio(self.into()),
            _ => CommandPacket::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CommandPacket {
    Beeper(BeeperCommand),
    ADC(ADCCommand),
    Alarm(AlarmCommand),
    Flash(FlashCommand),
    System(SystemCommand),
    Radio(RadioCommand),
    Unknown,
}

impl From<&[u8]> for CommandPacket {
    fn from(slice: &[u8]) -> Self {
        Array::from(slice).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adc::ADCChannel;
    use beeper::{note::Note, tone::Tone};
    use flash::storage_slot::StorageSlot;
    use time::Time;

    #[test]
    fn beeper_command() {
        assert_eq!(
            CommandPacket::from([1, 1, 1].as_ref()),
            CommandPacket::Beeper(BeeperCommand::Beep(1))
        );
        assert_eq!(
            CommandPacket::from([1, 1, 15].as_ref()),
            CommandPacket::Beeper(BeeperCommand::Beep(15))
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
            CommandPacket::from([1, 2, 0xA5, 100, 0xC5, 50].as_ref()),
            CommandPacket::Beeper(BeeperCommand::Melody(array))
        );

        assert_eq!(
            Array::from(CommandPacket::Beeper(BeeperCommand::Melody(array))).as_ref(),
            [1, 2, 0xA5, 100, 0xC5, 50]
        );
    }

    #[test]
    fn alarm_command() {
        assert_eq!(
            CommandPacket::from([2, 1].as_ref()),
            CommandPacket::Alarm(AlarmCommand::Get)
        );

        assert_eq!(
            Array::from(CommandPacket::Alarm(AlarmCommand::Get)).as_ref(),
            [2, 1]
        );

        assert_eq!(
            CommandPacket::from([2, 2, 18, 33, 17].as_ref()),
            CommandPacket::Alarm(AlarmCommand::Set(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            }))
        );

        assert_eq!(
            CommandPacket::from([2, 2, 33, 18, 1].as_ref()),
            CommandPacket::Alarm(AlarmCommand::Set(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            }))
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
            CommandPacket::from([3, 1].as_ref()),
            CommandPacket::System(SystemCommand::Reset)
        );

        assert_eq!(
            Array::from(CommandPacket::System(SystemCommand::Reset)).as_ref(),
            [3, 1]
        );

        let array = Array::from([1, 2, 3, 10].as_ref());
        assert_eq!(
            CommandPacket::from([3, 2, 1, 2, 3, 10].as_ref()),
            CommandPacket::System(SystemCommand::Echo(array))
        );

        assert_eq!(
            Array::from(CommandPacket::System(SystemCommand::Echo(array))).as_ref(),
            [3, 2, 1, 2, 3, 10]
        );
    }

    #[test]
    fn flash_command() {
        assert_eq!(
            CommandPacket::from([4, 1, 0x1f].as_ref()),
            CommandPacket::Flash(FlashCommand::Read(StorageSlot::One))
        );

        assert_eq!(
            Array::from(CommandPacket::Flash(FlashCommand::Read(StorageSlot::One))).as_ref(),
            [4, 1, 0x1f]
        );

        assert_eq!(
            CommandPacket::from([4, 2, 0x1f, 5].as_ref()),
            CommandPacket::Flash(FlashCommand::Write(StorageSlot::One, 5))
        );

        assert_eq!(
            Array::from(CommandPacket::Flash(FlashCommand::Write(
                StorageSlot::One,
                5
            )))
            .as_ref(),
            [4, 2, 0x1f, 5]
        );

        assert_eq!(
            CommandPacket::from([4, 3].as_ref()),
            CommandPacket::Flash(FlashCommand::EraseAll)
        );

        assert_eq!(
            Array::from(CommandPacket::Flash(FlashCommand::EraseAll)).as_ref(),
            [4, 3]
        );
    }

    #[test]
    fn adc_command() {
        assert_eq!(
            CommandPacket::from([5, 1, 1].as_ref()),
            CommandPacket::ADC(ADCCommand::Read(ADCChannel::Channel1))
        );
        assert_eq!(
            CommandPacket::from([5, 1, 3].as_ref()),
            CommandPacket::ADC(ADCCommand::Read(ADCChannel::Channel3))
        );
        assert_eq!(
            CommandPacket::from([5, 1, 7].as_ref()),
            CommandPacket::ADC(ADCCommand::Read(ADCChannel::Channel7))
        );

        assert_eq!(
            CommandPacket::from([5, 0].as_ref()),
            CommandPacket::ADC(ADCCommand::Unknown)
        );
        assert_eq!(
            CommandPacket::from([5, 2].as_ref()),
            CommandPacket::ADC(ADCCommand::Unknown)
        );
        assert_eq!(
            CommandPacket::from([5, 8].as_ref()),
            CommandPacket::ADC(ADCCommand::Unknown)
        );

        assert_eq!(
            Array::from(CommandPacket::ADC(ADCCommand::Read(ADCChannel::Channel5))).as_ref(),
            [5, 1, 5]
        );
    }

    #[test]
    fn radio_command() {
        assert_eq!(
            CommandPacket::from([6, 1, 2].as_ref()),
            CommandPacket::Radio(RadioCommand::Transmit(Array::from([2].as_ref())))
        );
        assert_eq!(
            CommandPacket::from([6, 2].as_ref()),
            CommandPacket::Radio(RadioCommand::Receive)
        );
        assert_eq!(
            CommandPacket::from([6, 3].as_ref()),
            CommandPacket::Radio(RadioCommand::Status)
        );

        assert_eq!(
            CommandPacket::from([6, 0].as_ref()),
            CommandPacket::Radio(RadioCommand::Unknown)
        );
        assert_eq!(
            CommandPacket::from([6, 4].as_ref()),
            CommandPacket::Radio(RadioCommand::Unknown)
        );
        assert_eq!(
            CommandPacket::from([6, 5].as_ref()),
            CommandPacket::Radio(RadioCommand::Unknown)
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
    fn unknown_command() {
        assert_eq!(CommandPacket::from([0].as_ref()), CommandPacket::Unknown);
        assert_eq!(CommandPacket::from([7].as_ref()), CommandPacket::Unknown);
        assert_eq!(
            CommandPacket::from([8, 9, 10].as_ref()),
            CommandPacket::Unknown
        );

        assert_eq!(Array::from(CommandPacket::Unknown).as_ref(), [0]);
    }
}
