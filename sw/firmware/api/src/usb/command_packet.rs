use array::Array;
use beeper::tone::Tone;
use flash::storage_slot::StorageSlot;
use time::Time;

impl From<CommandPacket> for Array<u8> {
    fn from(packet: CommandPacket) -> Self {
        match packet {
            CommandPacket::Beep(n_beeps) => [1, n_beeps].as_ref().into(),
            CommandPacket::AlarmSet(time) => {
                [2, time.hours, time.minutes, time.seconds].as_ref().into()
            }
            CommandPacket::AlarmGet => [3].as_ref().into(),
            CommandPacket::Reset => [4].as_ref().into(),
            CommandPacket::FlashRead(slot) => [5, slot.into()].as_ref().into(),
            CommandPacket::FlashWrite(slot, value) => [6, slot.into(), value].as_ref().into(),
            CommandPacket::FlashEraseAll => [7].as_ref().into(),
            CommandPacket::Melody(tones) => {
                let mut array = Array::new();
                array.push(8);
                tones.as_ref().iter().for_each(|tone| {
                    array.push(tone.note);
                    array.push(tone.duration);
                });
                array.as_ref().into()
            }
            CommandPacket::Echo(echo_array) => {
                let mut array = Array::new();
                array.push(9);
                echo_array.as_ref().iter().for_each(|echo_value| {
                    array.push(*echo_value);
                });
                array.as_ref().into()
            }
            CommandPacket::Unknown => [0].as_ref().into(),
        }
    }
}

impl Into<CommandPacket> for Array<u8> {
    fn into(self) -> CommandPacket {
        let command_type_byte = self[0];
        match command_type_byte {
            1 => CommandPacket::Beep(self[1]),
            2 => CommandPacket::AlarmSet(Time {
                hours: self[1],
                minutes: self[2],
                seconds: self[3],
            }),
            3 => CommandPacket::AlarmGet,
            4 => CommandPacket::Reset,
            5 => CommandPacket::FlashRead(StorageSlot::from(self[1])),
            6 => CommandPacket::FlashWrite(StorageSlot::from(self[1]), self[2]),
            7 => CommandPacket::FlashEraseAll,
            8 => {
                let mut array: Array<Tone> = Array::new();
                for index in (1..self.len()).step_by(2) {
                    array.push(Tone::new(self[index], self[index + 1]));
                }
                CommandPacket::Melody(array)
            }
            9 => {
                let mut echo_array = Array::new();
                self.as_ref()[1..]
                    .iter()
                    .for_each(|echo_value| echo_array.push(*echo_value));
                CommandPacket::Echo(echo_array)
            }
            _ => CommandPacket::Unknown,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum CommandPacket {
    Beep(u8),
    AlarmGet,
    AlarmSet(Time),
    FlashRead(StorageSlot),
    FlashWrite(StorageSlot, u8),
    FlashEraseAll,
    Reset,
    Melody(Array<Tone>),
    Echo(Array<u8>),
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
    use beeper::Note;

    #[test]
    fn beep_command() {
        assert_eq!(CommandPacket::from([1, 1].as_ref()), CommandPacket::Beep(1));
        assert_eq!(
            CommandPacket::from([1, 15].as_ref()),
            CommandPacket::Beep(15)
        );

        assert_eq!(Array::from(CommandPacket::Beep(1)).as_ref(), [1, 1]);
        assert_eq!(Array::from(CommandPacket::Beep(15)).as_ref(), [1, 15]);
    }

    #[test]
    fn alarm_set_command() {
        assert_eq!(
            CommandPacket::from([2, 18, 33, 17].as_ref()),
            CommandPacket::AlarmSet(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            })
        );

        assert_eq!(
            CommandPacket::from([2, 33, 18, 1].as_ref()),
            CommandPacket::AlarmSet(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            })
        );

        assert_eq!(
            Array::from(CommandPacket::AlarmSet(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            }))
            .as_ref(),
            [2, 18, 33, 17]
        );

        assert_eq!(
            Array::from(CommandPacket::AlarmSet(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            }))
            .as_ref(),
            [2, 33, 18, 1]
        );
    }

    #[test]
    fn alarm_get_command() {
        assert_eq!(CommandPacket::from([3].as_ref()), CommandPacket::AlarmGet);
        assert_eq!(
            CommandPacket::from([3, 11, 22].as_ref()),
            CommandPacket::AlarmGet
        );

        assert_eq!(Array::from(CommandPacket::AlarmGet).as_ref(), [3]);
    }

    #[test]
    fn reset_command() {
        assert_eq!(CommandPacket::from([4].as_ref()), CommandPacket::Reset);

        assert_eq!(Array::from(CommandPacket::Reset).as_ref(), [4]);
    }

    #[test]
    fn flash_read_command() {
        assert_eq!(
            CommandPacket::from([5, 0x1f].as_ref()),
            CommandPacket::FlashRead(StorageSlot::One)
        );

        assert_eq!(
            Array::from(CommandPacket::FlashRead(StorageSlot::One)).as_ref(),
            [5, 0x1f]
        );
    }

    #[test]
    fn flash_write_command() {
        assert_eq!(
            CommandPacket::from([6, 0x1f, 5].as_ref()),
            CommandPacket::FlashWrite(StorageSlot::One, 5)
        );

        assert_eq!(
            Array::from(CommandPacket::FlashWrite(StorageSlot::One, 5)).as_ref(),
            [6, 0x1f, 5]
        );
    }

    #[test]
    fn flash_erase_all_command() {
        assert_eq!(
            CommandPacket::from([7].as_ref()),
            CommandPacket::FlashEraseAll
        );

        assert_eq!(Array::from(CommandPacket::FlashEraseAll).as_ref(), [7]);
    }

    #[test]
    fn melody_command() {
        let mut array: Array<Tone> = Array::new();
        array.push(Tone::new(Note::A5 as u8, 100));
        array.push(Tone::new(Note::B5 as u8, 50));
        assert_eq!(
            CommandPacket::from([8, 0xA5, 100, 0xC5, 50].as_ref()),
            CommandPacket::Melody(array)
        );

        assert_eq!(
            Array::from(CommandPacket::Melody(array)).as_ref(),
            [8, 0xA5, 100, 0xC5, 50]
        );
    }

    #[test]
    fn echo_command() {
        let array = Array::from([1, 2, 3, 10].as_ref());
        assert_eq!(
            CommandPacket::from([9, 1, 2, 3, 10].as_ref()),
            CommandPacket::Echo(array)
        );

        assert_eq!(
            Array::from(CommandPacket::Echo(array)).as_ref(),
            [9, 1, 2, 3, 10]
        );
    }

    #[test]
    fn unknown_command() {
        assert_eq!(CommandPacket::from([0].as_ref()), CommandPacket::Unknown);
        assert_eq!(CommandPacket::from([10].as_ref()), CommandPacket::Unknown);
        assert_eq!(
            CommandPacket::from([11, 12, 13].as_ref()),
            CommandPacket::Unknown
        );

        assert_eq!(Array::from(CommandPacket::Unknown).as_ref(), [0]);
    }
}
