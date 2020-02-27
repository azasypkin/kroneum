use array::Array;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RadioCommand {
    Transmit(Array<u8>),
    Receive,
    Status,
    Unknown,
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
            RadioCommand::Unknown => [0].as_ref().into(),
        }
    }
}

impl Into<RadioCommand> for Array<u8> {
    fn into(self) -> RadioCommand {
        let command_type_byte = self[0];
        match command_type_byte {
            0x1 => {
                let mut data_to_transmit = Array::new();
                self.as_ref()[1..]
                    .iter()
                    .for_each(|byte| data_to_transmit.push(*byte));
                RadioCommand::Transmit(data_to_transmit)
            }
            0x2 => RadioCommand::Receive,
            0x3 => RadioCommand::Status,
            _ => RadioCommand::Unknown,
        }
    }
}

impl From<&[u8]> for RadioCommand {
    fn from(slice: &[u8]) -> Self {
        Array::from(slice).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmit_command() {
        let array = Array::from([4, 2, 3, 10].as_ref());
        assert_eq!(
            RadioCommand::from([1, 4, 2, 3, 10].as_ref()),
            RadioCommand::Transmit(array)
        );

        assert_eq!(
            Array::from(RadioCommand::Transmit(array)).as_ref(),
            [1, 4, 2, 3, 10]
        );
    }

    #[test]
    fn receive_command() {
        assert_eq!(RadioCommand::from([2].as_ref()), RadioCommand::Receive);
        assert_eq!(
            RadioCommand::from([2, 11, 22].as_ref()),
            RadioCommand::Receive
        );

        assert_eq!(Array::from(RadioCommand::Receive).as_ref(), [2]);
    }

    #[test]
    fn status_command() {
        assert_eq!(RadioCommand::from([3].as_ref()), RadioCommand::Status);
        assert_eq!(
            RadioCommand::from([3, 11, 22].as_ref()),
            RadioCommand::Status
        );

        assert_eq!(Array::from(RadioCommand::Status).as_ref(), [3]);
    }

    #[test]
    fn unknown_command() {
        assert_eq!(RadioCommand::from([0].as_ref()), RadioCommand::Unknown);
        assert_eq!(RadioCommand::from([4].as_ref()), RadioCommand::Unknown);
        assert_eq!(
            RadioCommand::from([5, 6, 7].as_ref()),
            RadioCommand::Unknown
        );

        assert_eq!(Array::from(RadioCommand::Unknown).as_ref(), [0]);
    }
}
