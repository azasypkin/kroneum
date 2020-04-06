use array::Array;
use beeper::tone::Tone;
use core::convert::TryFrom;
use usb::command_error::CommandError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BeeperCommand {
    Beep(u8),
    Melody(Array<Tone>),
}

impl From<BeeperCommand> for Array<u8> {
    fn from(packet: BeeperCommand) -> Self {
        match packet {
            BeeperCommand::Beep(n_beeps) => (&[1, n_beeps]).into(),
            BeeperCommand::Melody(tones) => {
                let mut array = Array::from(&[2]);
                tones.as_ref().iter().for_each(|tone| {
                    array.push(tone.note);
                    array.push(tone.duration);
                });
                array.as_ref().into()
            }
        }
    }
}

impl TryFrom<Array<u8>> for BeeperCommand {
    type Error = CommandError;

    fn try_from(mut value: Array<u8>) -> Result<Self, Self::Error> {
        match (value.shift(), value.len()) {
            (Some(0x1), 1) => Ok(BeeperCommand::Beep(value[0])),
            // Every tone consists of frequency and duration, so number of bytes should be even.
            (Some(0x2), n_tones) if n_tones > 1 && n_tones & 1 == 0 => {
                let mut array = Array::new();
                value
                    .as_ref()
                    .chunks(2)
                    .for_each(|pair| array.push(Tone::new(pair[0], pair[1])));
                Ok(BeeperCommand::Melody(array))
            }
            _ => Err(CommandError::InvalidCommand),
        }
    }
}

impl TryFrom<&[u8]> for BeeperCommand {
    type Error = CommandError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Array::from(slice))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beeper::note::Note;

    #[test]
    fn beep_command() {
        assert_eq!(
            BeeperCommand::try_from([1, 5].as_ref()),
            Ok(BeeperCommand::Beep(5))
        );

        assert_eq!(Array::from(BeeperCommand::Beep(5)).as_ref(), [1, 5]);
    }

    #[test]
    fn melody_command() {
        let mut tones: Array<Tone> = Array::new();
        tones.push(Tone::new(Note::A5 as u8, 100));
        tones.push(Tone::new(Note::B5 as u8, 50));

        assert_eq!(
            BeeperCommand::try_from([2, 0xA5, 100, 0xC5, 50].as_ref()),
            Ok(BeeperCommand::Melody(tones))
        );

        assert_eq!(
            Array::from(BeeperCommand::Melody(tones)).as_ref(),
            [2, 0xA5, 100, 0xC5, 50]
        );
    }

    #[test]
    fn unknown_command() {
        assert_eq!(
            BeeperCommand::try_from([0].as_ref()),
            Err(CommandError::InvalidCommand)
        );
        assert_eq!(
            BeeperCommand::try_from([3].as_ref()),
            Err(CommandError::InvalidCommand)
        );
        assert_eq!(
            BeeperCommand::try_from([4, 5, 6].as_ref()),
            Err(CommandError::InvalidCommand)
        );
    }
}
