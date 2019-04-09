use crate::flash::storage_slot::StorageSlot;
use crate::time::Time;

const COMMAND_BYTE_SEQUENCE_LENGTH: usize = 6;
pub type CommandByteSequence = [u8; COMMAND_BYTE_SEQUENCE_LENGTH];

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum CommandPacket {
    Beep(u8),
    AlarmGet,
    AlarmSet(Time),
    FlashRead(StorageSlot),
    FlashWrite(StorageSlot, u8),
    FlashEraseAll,
    Reset,
    Unknown,
}

impl CommandPacket {
    pub fn to_bytes(self) -> CommandByteSequence {
        match self {
            CommandPacket::Beep(n_beeps) => [1, 0, n_beeps, 0, 0, 0],
            CommandPacket::AlarmSet(time) => [2, 0, time.hours, time.minutes, time.seconds, 0],
            CommandPacket::AlarmGet => [3, 0, 0, 0, 0, 0],
            CommandPacket::Reset => [4, 0, 0, 0, 0, 0],
            CommandPacket::FlashRead(slot) => [5, 0, slot.into(), 0, 0, 0],
            CommandPacket::FlashWrite(slot, value) => [6, 0, slot.into(), value, 0, 0],
            CommandPacket::FlashEraseAll => [7, 0, 0, 0, 0, 0],
            CommandPacket::Unknown => [0; COMMAND_BYTE_SEQUENCE_LENGTH],
        }
    }
}

impl From<(u16, u16, u16)> for CommandPacket {
    fn from((header, data_1, data_2): (u16, u16, u16)) -> Self {
        let command_type_byte = header & 0xff;
        match command_type_byte {
            1 => CommandPacket::Beep((data_1 & 0xff) as u8),
            2 => CommandPacket::AlarmSet(Time {
                hours: (data_1 & 0xff) as u8,
                minutes: ((data_1 & 0xff00) >> 8) as u8,
                seconds: (data_2 & 0xff) as u8,
            }),
            3 => CommandPacket::AlarmGet,
            4 => CommandPacket::Reset,
            5 => CommandPacket::FlashRead(StorageSlot::from((data_1 & 0xff) as u8)),
            6 => CommandPacket::FlashWrite(
                StorageSlot::from((data_1 & 0xff) as u8),
                ((data_1 & 0xff00) >> 8) as u8,
            ),
            7 => CommandPacket::FlashEraseAll,
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
    fn alarm_set_command() {
        assert_eq!(
            CommandPacket::from((2, 0x2112, 17)),
            CommandPacket::AlarmSet(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            })
        );

        assert_eq!(
            CommandPacket::from((2, 0x1221, 1)),
            CommandPacket::AlarmSet(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            })
        );

        assert_eq!(
            CommandPacket::AlarmSet(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            })
            .to_bytes(),
            [2, 0, 18, 33, 17, 0]
        );
        assert_eq!(
            CommandPacket::AlarmSet(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            })
            .to_bytes(),
            [2, 0, 33, 18, 1, 0]
        );
    }

    #[test]
    fn alarm_get_command() {
        assert_eq!(CommandPacket::from((3, 0, 0)), CommandPacket::AlarmGet);
        assert_eq!(CommandPacket::from((3, 11, 22)), CommandPacket::AlarmGet);

        assert_eq!(CommandPacket::AlarmGet.to_bytes(), [3, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn reset_command() {
        assert_eq!(CommandPacket::from((4, 0, 0)), CommandPacket::Reset);

        assert_eq!(CommandPacket::Reset.to_bytes(), [4, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn flash_read_command() {
        assert_eq!(
            CommandPacket::from((5, 0x1f, 0)),
            CommandPacket::FlashRead(StorageSlot::One)
        );

        assert_eq!(
            CommandPacket::FlashRead(StorageSlot::One).to_bytes(),
            [5, 0, 0x1f, 0, 0, 0]
        );
    }

    #[test]
    fn flash_write_command() {
        assert_eq!(
            CommandPacket::from((6, 0x051f, 0)),
            CommandPacket::FlashWrite(StorageSlot::One, 5)
        );

        assert_eq!(
            CommandPacket::FlashWrite(StorageSlot::One, 5).to_bytes(),
            [6, 0, 0x1f, 5, 0, 0]
        );
    }

    #[test]
    fn flash_erase_all_command() {
        assert_eq!(CommandPacket::from((7, 0, 0)), CommandPacket::FlashEraseAll);
        assert_eq!(CommandPacket::FlashEraseAll.to_bytes(), [7, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn unknown_command() {
        assert_eq!(CommandPacket::from((0, 0, 0)), CommandPacket::Unknown);
        assert_eq!(CommandPacket::from((8, 0, 0)), CommandPacket::Unknown);
        assert_eq!(CommandPacket::from((10, 11, 22)), CommandPacket::Unknown);

        assert_eq!(
            CommandPacket::Unknown.to_bytes(),
            [0; COMMAND_BYTE_SEQUENCE_LENGTH]
        );
    }
}
