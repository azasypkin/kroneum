use array::Array;
use core::convert::TryFrom;
use usb::command_error::CommandError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SystemCommand {
    Reset,
    Echo(Array<u8>),
}

impl From<SystemCommand> for Array<u8> {
    fn from(packet: SystemCommand) -> Self {
        match packet {
            SystemCommand::Reset => [1].as_ref().into(),
            SystemCommand::Echo(mut echo_data) => {
                echo_data.unshift(2);
                echo_data
            }
        }
    }
}

impl TryFrom<Array<u8>> for SystemCommand {
    type Error = CommandError;

    fn try_from(mut value: Array<u8>) -> Result<Self, Self::Error> {
        match (value.shift(), value.len()) {
            (Some(0x1), 0) => Ok(SystemCommand::Reset),
            (Some(0x2), n_echo_bytes) if n_echo_bytes > 0 => Ok(SystemCommand::Echo(value)),
            _ => Err(CommandError::InvalidCommand),
        }
    }
}

impl TryFrom<&[u8]> for SystemCommand {
    type Error = CommandError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Array::from(slice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_command() {
        assert_eq!(
            SystemCommand::try_from([1].as_ref()),
            Ok(SystemCommand::Reset)
        );

        assert_eq!(Array::from(SystemCommand::Reset).as_ref(), [1]);
    }

    #[test]
    fn echo_command() {
        let array = Array::from(&[1, 2, 3, 10]);
        assert_eq!(
            SystemCommand::try_from([2, 1, 2, 3, 10].as_ref()),
            Ok(SystemCommand::Echo(array))
        );

        assert_eq!(
            Array::from(SystemCommand::Echo(array)).as_ref(),
            [2, 1, 2, 3, 10]
        );
    }

    #[test]
    fn unknown_command() {
        assert_eq!(
            SystemCommand::try_from([0].as_ref()),
            Err(CommandError::InvalidCommand)
        );
        assert_eq!(
            SystemCommand::try_from([3].as_ref()),
            Err(CommandError::InvalidCommand)
        );
        assert_eq!(
            SystemCommand::try_from([4, 5, 6].as_ref()),
            Err(CommandError::InvalidCommand)
        );
    }
}
