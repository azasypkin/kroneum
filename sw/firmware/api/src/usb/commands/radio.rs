use array::Array;
use core::convert::TryFrom;
use usb::command_error::CommandError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RadioCommand {
    Transmit(Array<u8>),
    Receive,
    Status,
}

impl From<RadioCommand> for Array<u8> {
    fn from(packet: RadioCommand) -> Self {
        match packet {
            RadioCommand::Transmit(data_to_transmit) => {
                let mut array = Array::new();
                [1].iter()
                    .chain(data_to_transmit.as_ref().iter())
                    .for_each(|byte| array.push(*byte));
                array.as_ref().into()
            }
            RadioCommand::Receive => [2].as_ref().into(),
            RadioCommand::Status => [3].as_ref().into(),
        }
    }
}

impl TryFrom<Array<u8>> for RadioCommand {
    type Error = CommandError;

    fn try_from(mut value: Array<u8>) -> Result<Self, Self::Error> {
        match (value.shift(), value.len()) {
            (Some(0x1), num) if num > 0 => Ok(RadioCommand::Transmit(value)),
            (Some(0x2), 0) => Ok(RadioCommand::Receive),
            (Some(0x3), 0) => Ok(RadioCommand::Status),
            _ => Err(CommandError::InvalidCommand),
        }
    }
}

impl TryFrom<&[u8]> for RadioCommand {
    type Error = CommandError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Array::from(slice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmit_command() {
        let array = Array::from(&[4, 2, 3, 10]);
        assert_eq!(
            RadioCommand::try_from([1, 4, 2, 3, 10].as_ref()),
            Ok(RadioCommand::Transmit(array))
        );

        assert_eq!(
            Array::from(RadioCommand::Transmit(array)).as_ref(),
            [1, 4, 2, 3, 10]
        );
    }

    #[test]
    fn receive_command() {
        assert_eq!(
            RadioCommand::try_from([2].as_ref()),
            Ok(RadioCommand::Receive)
        );
        assert_eq!(
            RadioCommand::try_from([2].as_ref()),
            Ok(RadioCommand::Receive)
        );

        assert_eq!(Array::from(RadioCommand::Receive).as_ref(), [2]);
    }

    #[test]
    fn status_command() {
        assert_eq!(
            RadioCommand::try_from([3].as_ref()),
            Ok(RadioCommand::Status)
        );
        assert_eq!(
            RadioCommand::try_from([3].as_ref()),
            Ok(RadioCommand::Status)
        );

        assert_eq!(Array::from(RadioCommand::Status).as_ref(), [3]);
    }

    #[test]
    fn unknown_command() {
        assert_eq!(
            RadioCommand::try_from([0].as_ref()),
            Err(CommandError::InvalidCommand)
        );
        assert_eq!(
            RadioCommand::try_from([4].as_ref()),
            Err(CommandError::InvalidCommand)
        );
        assert_eq!(
            RadioCommand::try_from([5, 6, 7].as_ref()),
            Err(CommandError::InvalidCommand)
        );
    }
}
