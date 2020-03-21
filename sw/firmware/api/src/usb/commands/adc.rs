use adc::ADCChannel;
use array::Array;
use core::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ADCCommand {
    Read(ADCChannel),
    Unknown,
}

impl From<ADCCommand> for Array<u8> {
    fn from(packet: ADCCommand) -> Self {
        match packet {
            ADCCommand::Read(channel) => [1, channel.into()].as_ref().into(),
            ADCCommand::Unknown => [0].as_ref().into(),
        }
    }
}

impl Into<ADCCommand> for Array<u8> {
    fn into(mut self) -> ADCCommand {
        match (self.shift(), self.len()) {
            (Some(0x1), 1) => {
                if let Ok(channel) = ADCChannel::try_from(self[0]) {
                    ADCCommand::Read(channel)
                } else {
                    ADCCommand::Unknown
                }
            }
            _ => ADCCommand::Unknown,
        }
    }
}

impl From<&[u8]> for ADCCommand {
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
            ADCCommand::from([1, 1].as_ref()),
            ADCCommand::Read(ADCChannel::Channel1)
        );

        assert_eq!(
            ADCCommand::from([1, 3].as_ref()),
            ADCCommand::Read(ADCChannel::Channel3)
        );

        assert_eq!(
            Array::from(ADCCommand::Read(ADCChannel::Channel5)).as_ref(),
            [1, 5]
        );
    }

    #[test]
    fn unknown_command() {
        assert_eq!(ADCCommand::from([0].as_ref()), ADCCommand::Unknown);
        assert_eq!(ADCCommand::from([2].as_ref()), ADCCommand::Unknown);
        assert_eq!(ADCCommand::from([3, 4, 5].as_ref()), ADCCommand::Unknown);

        // Read command with unknown channels.
        assert_eq!(ADCCommand::from([1, 0].as_ref()), ADCCommand::Unknown);
        assert_eq!(ADCCommand::from([1, 2].as_ref()), ADCCommand::Unknown);
        assert_eq!(ADCCommand::from([1, 8].as_ref()), ADCCommand::Unknown);

        assert_eq!(Array::from(ADCCommand::Unknown).as_ref(), [0]);
    }
}
