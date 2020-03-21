use array::Array;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SystemCommand {
    Reset,
    Echo(Array<u8>),
    Unknown,
}

impl From<SystemCommand> for Array<u8> {
    fn from(packet: SystemCommand) -> Self {
        match packet {
            SystemCommand::Reset => [1].as_ref().into(),
            SystemCommand::Echo(mut echo_data) => {
                echo_data.unshift(2);
                echo_data
            }
            SystemCommand::Unknown => [0].as_ref().into(),
        }
    }
}

impl Into<SystemCommand> for Array<u8> {
    fn into(mut self) -> SystemCommand {
        match (self.shift(), self.len()) {
            (Some(0x1), 0) => SystemCommand::Reset,
            (Some(0x2), n_echo_bytes) if n_echo_bytes > 0 => SystemCommand::Echo(self),
            _ => SystemCommand::Unknown,
        }
    }
}

impl From<&[u8]> for SystemCommand {
    fn from(slice: &[u8]) -> Self {
        Array::from(slice).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_command() {
        assert_eq!(SystemCommand::from([1].as_ref()), SystemCommand::Reset);

        assert_eq!(Array::from(SystemCommand::Reset).as_ref(), [1]);
    }

    #[test]
    fn echo_command() {
        let array = Array::from([1, 2, 3, 10].as_ref());
        assert_eq!(
            SystemCommand::from([2, 1, 2, 3, 10].as_ref()),
            SystemCommand::Echo(array)
        );

        assert_eq!(
            Array::from(SystemCommand::Echo(array)).as_ref(),
            [2, 1, 2, 3, 10]
        );
    }

    #[test]
    fn unknown_command() {
        assert_eq!(SystemCommand::from([0].as_ref()), SystemCommand::Unknown);
        assert_eq!(SystemCommand::from([3].as_ref()), SystemCommand::Unknown);
        assert_eq!(
            SystemCommand::from([4, 5, 6].as_ref()),
            SystemCommand::Unknown
        );

        assert_eq!(Array::from(SystemCommand::Unknown).as_ref(), [0]);
    }
}
