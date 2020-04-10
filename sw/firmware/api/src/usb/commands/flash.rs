use array::Array;
use core::convert::TryFrom;
use flash::storage_slot::StorageSlot;
use usb::usb_error::USBError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlashCommand {
    Read(StorageSlot),
    Write(StorageSlot, u8),
    EraseAll,
}

impl From<FlashCommand> for Array<u8> {
    fn from(packet: FlashCommand) -> Self {
        match packet {
            FlashCommand::Read(storage_slot) => [1, storage_slot.into()].as_ref().into(),
            FlashCommand::Write(storage_slot, value) => {
                [2, storage_slot.into(), value].as_ref().into()
            }
            FlashCommand::EraseAll => [3].as_ref().into(),
        }
    }
}

impl TryFrom<Array<u8>> for FlashCommand {
    type Error = USBError;

    fn try_from(mut value: Array<u8>) -> Result<Self, Self::Error> {
        match (value.shift(), value.len()) {
            (Some(0x1), 1) => Ok(FlashCommand::Read(value[0].into())),
            (Some(0x2), 2) => Ok(FlashCommand::Write(value[0].into(), value[1])),
            (Some(0x3), 0) => Ok(FlashCommand::EraseAll),
            _ => Err(USBError::InvalidCommand),
        }
    }
}

impl TryFrom<&[u8]> for FlashCommand {
    type Error = USBError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Array::from(slice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_command() {
        assert_eq!(
            FlashCommand::try_from([1, 0x2f].as_ref()),
            Ok(FlashCommand::Read(StorageSlot::Two))
        );

        assert_eq!(
            Array::from(FlashCommand::Read(StorageSlot::Five)).as_ref(),
            [1, 0x5f]
        );
    }

    #[test]
    fn write_command() {
        assert_eq!(
            FlashCommand::try_from([2, 0x1f, 8].as_ref()),
            Ok(FlashCommand::Write(StorageSlot::One, 8))
        );
        assert_eq!(
            FlashCommand::try_from([2, 0x3f, 22].as_ref()),
            Ok(FlashCommand::Write(StorageSlot::Three, 22))
        );

        assert_eq!(
            Array::from(FlashCommand::Write(StorageSlot::One, 5)).as_ref(),
            [2, 0x1f, 5]
        );
    }

    #[test]
    fn erase_all_command() {
        assert_eq!(
            FlashCommand::try_from([3].as_ref()),
            Ok(FlashCommand::EraseAll)
        );

        assert_eq!(Array::from(FlashCommand::EraseAll).as_ref(), [3]);
    }

    #[test]
    fn invalid_command() {
        assert_eq!(
            FlashCommand::try_from([0].as_ref()),
            Err(USBError::InvalidCommand),
        );
        assert_eq!(
            FlashCommand::try_from([4].as_ref()),
            Err(USBError::InvalidCommand),
        );
        assert_eq!(
            FlashCommand::try_from([5, 6, 7].as_ref()),
            Err(USBError::InvalidCommand),
        );
    }
}
