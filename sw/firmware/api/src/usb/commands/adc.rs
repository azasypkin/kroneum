use adc::ADCChannel;
use array::Array;
use core::convert::TryFrom;
use usb::command_error::CommandError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ADCCommand {
    Read(ADCChannel),
}

impl TryFrom<Array<u8>> for ADCCommand {
    type Error = CommandError;

    fn try_from(mut value: Array<u8>) -> Result<Self, Self::Error> {
        if let (Some(0x1), 1) = (value.shift(), value.len()) {
            if let Ok(channel) = ADCChannel::try_from(value[0]) {
                return Ok(ADCCommand::Read(channel));
            }
        }

        Err(CommandError::InvalidCommand)
    }
}

impl TryFrom<&[u8]> for ADCCommand {
    type Error = CommandError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Array::from(slice))
    }
}

impl From<ADCCommand> for Array<u8> {
    fn from(packet: ADCCommand) -> Self {
        match packet {
            ADCCommand::Read(channel) => [1, channel.into()].as_ref().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_command() {
        assert_eq!(
            ADCCommand::try_from([1, 1].as_ref()),
            Ok(ADCCommand::Read(ADCChannel::Channel1))
        );

        assert_eq!(
            ADCCommand::try_from([1, 3].as_ref()),
            Ok(ADCCommand::Read(ADCChannel::Channel3))
        );

        assert_eq!(
            Array::from(ADCCommand::Read(ADCChannel::Channel5)).as_ref(),
            [1, 5]
        );
    }

    #[test]
    fn unknown_command() {
        assert_eq!(
            ADCCommand::try_from([0].as_ref()),
            Err(CommandError::InvalidCommand)
        );
        assert_eq!(
            ADCCommand::try_from([2].as_ref()),
            Err(CommandError::InvalidCommand)
        );
        assert_eq!(
            ADCCommand::try_from([3, 4, 5].as_ref()),
            Err(CommandError::InvalidCommand)
        );

        // Read command with unknown channels.
        assert_eq!(
            ADCCommand::try_from([1, 0].as_ref()),
            Err(CommandError::InvalidCommand)
        );
        assert_eq!(
            ADCCommand::try_from([1, 2].as_ref()),
            Err(CommandError::InvalidCommand)
        );
        assert_eq!(
            ADCCommand::try_from([1, 8].as_ref()),
            Err(CommandError::InvalidCommand)
        );
    }
}
