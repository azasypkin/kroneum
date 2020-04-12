use array::Array;
use core::convert::TryFrom;
use usb::usb_error::USBError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyboardCommand {
    Key(u8, u8),
}

impl TryFrom<Array<u8>> for KeyboardCommand {
    type Error = USBError;

    fn try_from(mut value: Array<u8>) -> Result<Self, Self::Error> {
        if let (Some(0x1), 2) = (value.shift(), value.len()) {
            Ok(KeyboardCommand::Key(value[0], value[1]))
        } else {
            Err(USBError::InvalidCommand)
        }
    }
}

impl TryFrom<&[u8]> for KeyboardCommand {
    type Error = USBError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Array::from(slice))
    }
}

impl From<KeyboardCommand> for Array<u8> {
    fn from(packet: KeyboardCommand) -> Self {
        match packet {
            KeyboardCommand::Key(key_code, delay) => [1, key_code, delay].as_ref().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_command() {
        assert_eq!(
            KeyboardCommand::try_from([1, 1, 1].as_ref()),
            Ok(KeyboardCommand::Key(1, 1))
        );

        assert_eq!(
            KeyboardCommand::try_from([1, 3, 4].as_ref()),
            Ok(KeyboardCommand::Key(3, 4))
        );

        assert_eq!(Array::from(KeyboardCommand::Key(5, 6)).as_ref(), [1, 5, 6]);
    }

    #[test]
    fn invalid_command() {
        assert_eq!(
            KeyboardCommand::try_from([0].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([0, 1].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([0, 1, 2].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([1].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([1, 2].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([2].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([2, 3].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([2, 3, 4].as_ref()),
            Err(USBError::InvalidCommand)
        );
    }
}
