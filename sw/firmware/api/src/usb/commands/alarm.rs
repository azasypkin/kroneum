use array::Array;
use time::Time;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AlarmCommand {
    Get,
    Set(Time),
    Unknown,
}

impl From<AlarmCommand> for Array<u8> {
    fn from(packet: AlarmCommand) -> Self {
        match packet {
            AlarmCommand::Get => [1].as_ref().into(),
            AlarmCommand::Set(time) => [2, time.hours, time.minutes, time.seconds].as_ref().into(),
            AlarmCommand::Unknown => [0].as_ref().into(),
        }
    }
}

impl Into<AlarmCommand> for Array<u8> {
    fn into(mut self) -> AlarmCommand {
        match (self.shift(), self.len()) {
            (Some(0x1), 0) => AlarmCommand::Get,
            (Some(0x2), 3) => AlarmCommand::Set(Time {
                hours: self[0],
                minutes: self[1],
                seconds: self[2],
            }),
            _ => AlarmCommand::Unknown,
        }
    }
}

impl From<&[u8]> for AlarmCommand {
    fn from(slice: &[u8]) -> Self {
        Array::from(slice).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_command() {
        assert_eq!(AlarmCommand::from([1].as_ref()), AlarmCommand::Get);

        assert_eq!(Array::from(AlarmCommand::Get).as_ref(), [1]);
    }

    #[test]
    fn set_command() {
        assert_eq!(
            AlarmCommand::from([2, 18, 33, 17].as_ref()),
            AlarmCommand::Set(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            })
        );
        assert_eq!(
            AlarmCommand::from([2, 33, 18, 1].as_ref()),
            AlarmCommand::Set(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            })
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
    fn unknown_command() {
        assert_eq!(AlarmCommand::from([0].as_ref()), AlarmCommand::Unknown);
        assert_eq!(AlarmCommand::from([3].as_ref()), AlarmCommand::Unknown);
        assert_eq!(
            AlarmCommand::from([4, 5, 6].as_ref()),
            AlarmCommand::Unknown
        );

        assert_eq!(Array::from(AlarmCommand::Unknown).as_ref(), [0]);
    }
}
