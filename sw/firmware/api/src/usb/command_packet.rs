use crate::flash::storage::StorageSlot;
use crate::time::Time;

const COMMAND_BYTE_SEQUENCE_LENGTH: usize = 6;
pub type CommandByteSequence = [u8; COMMAND_BYTE_SEQUENCE_LENGTH];

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum CommandPacket {
    Unknown,
    Beep(u8),
    GetAlarm,
    Reset,
    SetAlarm(Time),
    ReadFlash(StorageSlot),
}

impl CommandPacket {
    pub fn to_bytes(self) -> CommandByteSequence {
        match self {
            CommandPacket::Beep(n_beeps) => [1, 0, n_beeps, 0, 0, 0],
            CommandPacket::SetAlarm(time) => [2, 0, time.hours, time.minutes, time.seconds, 0],
            CommandPacket::GetAlarm => [3, 0, 0, 0, 0, 0],
            CommandPacket::Reset => [4, 0, 0, 0, 0, 0],
            CommandPacket::ReadFlash(slot) => [5, 0, slot.into(), 0, 0, 0],
            CommandPacket::Unknown => [0; COMMAND_BYTE_SEQUENCE_LENGTH],
        }
    }
}

impl From<(u16, u16, u16)> for CommandPacket {
    fn from((header, data_1, data_2): (u16, u16, u16)) -> Self {
        let command_type_byte = header & 0xff;
        match command_type_byte {
            1 => CommandPacket::Beep((data_1 & 0xff) as u8),
            2 => CommandPacket::SetAlarm(Time {
                hours: (data_1 & 0xff) as u8,
                minutes: ((data_1 & 0xff00) >> 8) as u8,
                seconds: (data_2 & 0xff) as u8,
            }),
            3 => CommandPacket::GetAlarm,
            4 => CommandPacket::Reset,
            5 => CommandPacket::ReadFlash(StorageSlot::from((data_1 & 0xff) as u8)),
            _ => CommandPacket::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_command_bytes() {
        assert_eq!(
            CommandByteSequence::default(),
            [0; COMMAND_BYTE_SEQUENCE_LENGTH]
        );
    }

    #[test]
    fn beep_command() {
        assert_eq!(CommandPacket::from((1, 1, 0)), CommandPacket::Beep(1));
        assert_eq!(CommandPacket::from((1, 15, 0)), CommandPacket::Beep(15));

        assert_eq!(CommandPacket::Beep(1).to_bytes(), [1, 0, 1, 0, 0, 0]);
        assert_eq!(CommandPacket::Beep(15).to_bytes(), [1, 0, 15, 0, 0, 0]);
    }

    #[test]
    fn set_alarm_command() {
        assert_eq!(
            CommandPacket::from((2, 0x2112, 17)),
            CommandPacket::SetAlarm(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            })
        );

        assert_eq!(
            CommandPacket::from((2, 0x1221, 1)),
            CommandPacket::SetAlarm(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            })
        );

        assert_eq!(
            CommandPacket::SetAlarm(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            })
            .to_bytes(),
            [2, 0, 18, 33, 17, 0]
        );
        assert_eq!(
            CommandPacket::SetAlarm(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            })
            .to_bytes(),
            [2, 0, 33, 18, 1, 0]
        );
    }

    #[test]
    fn get_alarm_command() {
        assert_eq!(CommandPacket::from((3, 0, 0)), CommandPacket::GetAlarm);
        assert_eq!(CommandPacket::from((3, 11, 22)), CommandPacket::GetAlarm);

        assert_eq!(CommandPacket::GetAlarm.to_bytes(), [3, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn reset_command() {
        assert_eq!(CommandPacket::from((4, 0, 0)), CommandPacket::Reset);

        assert_eq!(CommandPacket::Reset.to_bytes(), [4, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn read_flash_command() {
        assert_eq!(
            CommandPacket::from((5, 0x1f, 0)),
            CommandPacket::ReadFlash(StorageSlot::One)
        );

        assert_eq!(
            CommandPacket::ReadFlash(StorageSlot::One).to_bytes(),
            [5, 0, 0x1f, 0, 0, 0]
        );
    }

    #[test]
    fn unknown_command() {
        assert_eq!(CommandPacket::from((0, 0, 0)), CommandPacket::Unknown);
        assert_eq!(CommandPacket::from((6, 0, 0)), CommandPacket::Unknown);
        assert_eq!(CommandPacket::from((10, 11, 22)), CommandPacket::Unknown);

        assert_eq!(
            CommandPacket::Unknown.to_bytes(),
            [0; COMMAND_BYTE_SEQUENCE_LENGTH]
        );
    }
}
