use array::Array;
use flash::storage_slot::StorageSlot;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlashCommand {
    Read(StorageSlot),
    Write(StorageSlot, u8),
    EraseAll,
    Unknown,
}

impl From<FlashCommand> for Array<u8> {
    fn from(packet: FlashCommand) -> Self {
        match packet {
            FlashCommand::Read(storage_slot) => [1, storage_slot.into()].as_ref().into(),
            FlashCommand::Write(storage_slot, value) => {
                [2, storage_slot.into(), value].as_ref().into()
            }
            FlashCommand::EraseAll => [3].as_ref().into(),
            FlashCommand::Unknown => [0].as_ref().into(),
        }
    }
}

impl Into<FlashCommand> for Array<u8> {
    fn into(mut self) -> FlashCommand {
        match (self.shift(), self.len()) {
            (Some(0x1), 1) => FlashCommand::Read(self[0].into()),
            (Some(0x2), 2) => FlashCommand::Write(self[0].into(), self[1]),
            (Some(0x3), 0) => FlashCommand::EraseAll,
            _ => FlashCommand::Unknown,
        }
    }
}

impl From<&[u8]> for FlashCommand {
    fn from(slice: &[u8]) -> Self {
        Array::from(slice).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_command() {
        assert_eq!(
            FlashCommand::from([1, 0x2f].as_ref()),
            FlashCommand::Read(StorageSlot::Two)
        );

        assert_eq!(
            Array::from(FlashCommand::Read(StorageSlot::Five)).as_ref(),
            [1, 0x5f]
        );
    }

    #[test]
    fn write_command() {
        assert_eq!(
            FlashCommand::from([2, 0x1f, 8].as_ref()),
            FlashCommand::Write(StorageSlot::One, 8)
        );
        assert_eq!(
            FlashCommand::from([2, 0x3f, 22].as_ref()),
            FlashCommand::Write(StorageSlot::Three, 22)
        );

        assert_eq!(
            Array::from(FlashCommand::Write(StorageSlot::One, 5)).as_ref(),
            [2, 0x1f, 5]
        );
    }

    #[test]
    fn erase_all_command() {
        assert_eq!(FlashCommand::from([3].as_ref()), FlashCommand::EraseAll);

        assert_eq!(Array::from(FlashCommand::EraseAll).as_ref(), [3]);
    }

    #[test]
    fn unknown_command() {
        assert_eq!(FlashCommand::from([0].as_ref()), FlashCommand::Unknown);
        assert_eq!(FlashCommand::from([4].as_ref()), FlashCommand::Unknown);
        assert_eq!(
            FlashCommand::from([5, 6, 7].as_ref()),
            FlashCommand::Unknown
        );

        assert_eq!(Array::from(FlashCommand::Unknown).as_ref(), [0]);
    }
}
