use array::Array;
use core::convert::TryFrom;
use time::Time;
use usb::usb_error::USBError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AlarmCommand {
    Get,
    Set(Time),
}

impl From<AlarmCommand> for Array<u8> {
    fn from(packet: AlarmCommand) -> Self {
        match packet {
            AlarmCommand::Get => [1].as_ref().into(),
            AlarmCommand::Set(time) => [2, time.hours, time.minutes, time.seconds].as_ref().into(),
        }
    }
}

impl TryFrom<Array<u8>> for AlarmCommand {
    type Error = USBError;

    fn try_from(mut value: Array<u8>) -> Result<Self, Self::Error> {
        match (value.shift(), value.len()) {
            (Some(0x1), 0) => Ok(AlarmCommand::Get),
            (Some(0x2), 3) => Ok(AlarmCommand::Set(Time {
                hours: value[0],
                minutes: value[1],
                seconds: value[2],
            })),
            _ => Err(USBError::InvalidCommand),
        }
    }
}

impl TryFrom<&[u8]> for AlarmCommand {
    type Error = USBError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Array::from(slice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_command() {
        assert_eq!(AlarmCommand::try_from([1].as_ref()), Ok(AlarmCommand::Get));

        assert_eq!(Array::from(AlarmCommand::Get).as_ref(), [1]);
    }

    #[test]
    fn set_command() {
        assert_eq!(
            AlarmCommand::try_from([2, 18, 33, 17].as_ref()),
            Ok(AlarmCommand::Set(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            }))
        );
        assert_eq!(
            AlarmCommand::try_from([2, 33, 18, 1].as_ref()),
            Ok(AlarmCommand::Set(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            }))
        );

        assert_eq!(
            Array::from(AlarmCommand::Set(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            }))
            .as_ref(),
            [2, 18, 33, 17]
        );

        assert_eq!(
            Array::from(AlarmCommand::Set(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            }))
            .as_ref(),
            [2, 33, 18, 1]
        );
    }

    #[test]
    fn invalid_command() {
        assert_eq!(
            AlarmCommand::try_from([0].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            AlarmCommand::try_from([3].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            AlarmCommand::try_from([4, 5, 6].as_ref()),
            Err(USBError::InvalidCommand)
        );
    }
}
