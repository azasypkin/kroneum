use array::Array;
use beeper::tone::Tone;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BeeperCommand {
    Beep(u8),
    Melody(Array<Tone>),
    Unknown,
}

impl From<BeeperCommand> for Array<u8> {
    fn from(packet: BeeperCommand) -> Self {
        match packet {
            BeeperCommand::Beep(n_beeps) => [1, n_beeps].as_ref().into(),
            BeeperCommand::Melody(tones) => {
                let mut array = Array::from([2].as_ref());
                tones.as_ref().iter().for_each(|tone| {
                    array.push(tone.note);
                    array.push(tone.duration);
                });
                array.as_ref().into()
            }
            BeeperCommand::Unknown => [0].as_ref().into(),
        }
    }
}

impl Into<BeeperCommand> for Array<u8> {
    fn into(mut self) -> BeeperCommand {
        match (self.shift(), self.len()) {
            (Some(0x1), 1) => BeeperCommand::Beep(self[0].into()),
            // Every tone consists of frequency and duration, so number of bytes should be even.
            (Some(0x2), n_tones) if n_tones > 1 && n_tones & 1 == 0 => {
                let mut array: Array<Tone> = Array::new();
                for index in (0..n_tones).step_by(2) {
                    array.push(Tone::new(self[index], self[index + 1]));
                }
                BeeperCommand::Melody(array)
            }
            _ => BeeperCommand::Unknown,
        }
    }
}

impl From<&[u8]> for BeeperCommand {
    fn from(slice: &[u8]) -> Self {
        Array::from(slice).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beeper::note::Note;

    #[test]
    fn beep_command() {
        assert_eq!(BeeperCommand::from([1, 5].as_ref()), BeeperCommand::Beep(5));

        assert_eq!(Array::from(BeeperCommand::Beep(5)).as_ref(), [1, 5]);
    }

    #[test]
    fn melody_command() {
        let mut tones: Array<Tone> = Array::new();
        tones.push(Tone::new(Note::A5 as u8, 100));
        tones.push(Tone::new(Note::B5 as u8, 50));

        assert_eq!(
            BeeperCommand::from([2, 0xA5, 100, 0xC5, 50].as_ref()),
            BeeperCommand::Melody(tones)
        );

        assert_eq!(
            Array::from(BeeperCommand::Melody(tones)).as_ref(),
            [2, 0xA5, 100, 0xC5, 50]
        );
    }

    #[test]
    fn unknown_command() {
        assert_eq!(BeeperCommand::from([0].as_ref()), BeeperCommand::Unknown);
        assert_eq!(BeeperCommand::from([3].as_ref()), BeeperCommand::Unknown);
        assert_eq!(
            BeeperCommand::from([4, 5, 6].as_ref()),
            BeeperCommand::Unknown
        );

        assert_eq!(Array::from(BeeperCommand::Unknown).as_ref(), [0]);
    }
}
