use crate::flash::storage_slot::StorageSlot;
use crate::time::Time;
use core::ops::Index;
pub use usb::descriptors::MAX_PACKET_SIZE;

/// Byte representation of the `CommandPacket`.
pub struct CommandBytes {
    buffer: [u8; MAX_PACKET_SIZE],
    len: usize,
}

impl Index<usize> for CommandBytes {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer[index]
    }
}

impl CommandBytes {
    /// Creates `CommandBytes` structure with empty buffer of `MAX_PACKET_SIZE` size and the length
    /// of the actual data.
    pub fn new() -> Self {
        CommandBytes {
            buffer: [0; MAX_PACKET_SIZE],
            len: 0,
        }
    }

    /// Pushes `u8` value into `CommandBytes`. Note that if internal buffer is full, no more data
    /// will be written effectively making it read-only.
    pub fn push(&mut self, byte: u8) {
        if self.len < MAX_PACKET_SIZE {
            self.buffer[self.len] = byte;
            self.len += 1;
        }
    }

    /// Returns the length of the actual data stored in the structure.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl AsRef<[u8]> for CommandBytes {
    fn as_ref(&self) -> &[u8] {
        &self.buffer[..self.len]
    }
}

impl From<&[u8]> for CommandBytes {
    fn from(slice: &[u8]) -> Self {
        let mut command_bytes = CommandBytes::new();
        command_bytes.buffer;
        slice
            .iter()
            .enumerate()
            .for_each(|(_, n)| command_bytes.push(*n));
        command_bytes
    }
}

impl From<CommandPacket> for CommandBytes {
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
            CommandPacket::Unknown => [0].as_ref().into(),
        }
    }
}

impl Into<CommandPacket> for CommandBytes {
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
            _ => CommandPacket::Unknown,
        }
    }
}

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

impl From<&[u8]> for CommandPacket {
    fn from(slice: &[u8]) -> Self {
        CommandBytes::from(slice).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beep_command() {
        assert_eq!(CommandPacket::from([1, 1].as_ref()), CommandPacket::Beep(1));
        assert_eq!(
            CommandPacket::from([1, 15].as_ref()),
            CommandPacket::Beep(15)
        );

        assert_eq!(CommandBytes::from(CommandPacket::Beep(1)).as_ref(), [1, 1]);
        assert_eq!(
            CommandBytes::from(CommandPacket::Beep(15)).as_ref(),
            [1, 15]
        );
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
            CommandBytes::from(CommandPacket::AlarmSet(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            }))
            .as_ref(),
            [2, 18, 33, 17]
        );

        assert_eq!(
            CommandBytes::from(CommandPacket::AlarmSet(Time {
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

        assert_eq!(CommandBytes::from(CommandPacket::AlarmGet).as_ref(), [3]);
    }

    #[test]
    fn reset_command() {
        assert_eq!(CommandPacket::from([4].as_ref()), CommandPacket::Reset);

        assert_eq!(CommandBytes::from(CommandPacket::Reset).as_ref(), [4]);
    }

    #[test]
    fn flash_read_command() {
        assert_eq!(
            CommandPacket::from([5, 0x1f].as_ref()),
            CommandPacket::FlashRead(StorageSlot::One)
        );

        assert_eq!(
            CommandBytes::from(CommandPacket::FlashRead(StorageSlot::One)).as_ref(),
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
            CommandBytes::from(CommandPacket::FlashWrite(StorageSlot::One, 5)).as_ref(),
            [6, 0x1f, 5]
        );
    }

    #[test]
    fn flash_erase_all_command() {
        assert_eq!(
            CommandPacket::from([7].as_ref()),
            CommandPacket::FlashEraseAll
        );

        assert_eq!(
            CommandBytes::from(CommandPacket::FlashEraseAll).as_ref(),
            [7]
        );
    }

    #[test]
    fn unknown_command() {
        assert_eq!(CommandPacket::from([0].as_ref()), CommandPacket::Unknown);
        assert_eq!(CommandPacket::from([8].as_ref()), CommandPacket::Unknown);
        assert_eq!(
            CommandPacket::from([10, 11, 12].as_ref()),
            CommandPacket::Unknown
        );

        assert_eq!(CommandBytes::from(CommandPacket::Unknown).as_ref(), [0]);
    }
}
