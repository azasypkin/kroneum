use super::note::{Note, NOTE_1_2_DURATION, NOTE_1_4_DURATION};
use array::Array;
use beeper::tone::Tone;

/// Defines a predefined melody to play.
#[derive(Debug, Copy, Clone)]
pub enum Melody {
    Alarm,
    Beep,
    Reset,
    Setup,
    Custom(Array<Tone>),
}

impl Into<Array<Tone>> for Melody {
    fn into(self) -> Array<Tone> {
        match self {
            Melody::Alarm => Array::from(&ALARM_MELODY),
            Melody::Beep => Array::from(&BEEP_MELODY),
            Melody::Reset => Array::from(&RESET_MELODY),
            Melody::Setup => Array::from(&SETUP_MELODY),
            Melody::Custom(tones) => tones,
        }
    }
}

/// Melody that is being played when alarm triggers.
/// Can be generated at https://onlinesequencer.net/
const ALARM_MELODY: [Tone; 24] = [
    Tone::new(Note::B7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::GSharp7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::DSharp7 as u8, NOTE_1_2_DURATION),
    Tone::new(Note::GSharp7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::DSharp7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::FSharp7 as u8, NOTE_1_2_DURATION),
    Tone::new(Note::DSharp7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::FSharp7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::DSharp7 as u8, NOTE_1_2_DURATION),
    Tone::new(Note::Silence as u8, NOTE_1_2_DURATION),
    Tone::new(Note::DSharp7 as u8, NOTE_1_2_DURATION),
    Tone::new(Note::FSharp7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::DSharp7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::F7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::DSharp7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::F7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::DSharp7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::D7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::F7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::CSharp7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::F7 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::FSharp7 as u8, NOTE_1_2_DURATION),
    Tone::new(Note::DSharp7 as u8, NOTE_1_2_DURATION),
    Tone::new(Note::Silence as u8, NOTE_1_2_DURATION),
];

/// Melody to be used as beep (e.g. when setting alarm).
const BEEP_MELODY: [Tone; 1] = [Tone::new(Note::G5 as u8, NOTE_1_4_DURATION)];

/// Melody that is played when alarm is reset.
const RESET_MELODY: [Tone; 13] = [
    Tone::new(Note::A5 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::ASharp5 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::B5 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::C6 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::CSharp6 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::D6 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::DSharp6 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::E6 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::F6 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::FSharp6 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::G6 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::GSharp6 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::A6 as u8, NOTE_1_4_DURATION),
];

/// Melody that is played when user enters setup mode.
const SETUP_MELODY: [Tone; 2] = [
    Tone::new(Note::DSharp5 as u8, NOTE_1_4_DURATION),
    Tone::new(Note::DSharp5 as u8, NOTE_1_4_DURATION),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn properly_converts_into_tones_array() {
        let melody_tones: Array<Tone> = Melody::Beep.into();
        assert_eq!(melody_tones, Array::from(&BEEP_MELODY));

        let melody_tones: Array<Tone> = Melody::Alarm.into();
        assert_eq!(melody_tones, Array::from(&ALARM_MELODY));

        let melody_tones: Array<Tone> = Melody::Reset.into();
        assert_eq!(melody_tones, Array::from(&RESET_MELODY));

        let melody_tones: Array<Tone> = Melody::Setup.into();
        assert_eq!(melody_tones, Array::from(&SETUP_MELODY));

        let custom_tones = [
            Tone::new(Note::A0 as u8, NOTE_1_4_DURATION),
            Tone::new(Note::DSharp1 as u8, NOTE_1_2_DURATION),
        ];
        let melody_tones: Array<Tone> = Melody::Custom(Array::from(&custom_tones)).into();
        assert_eq!(melody_tones, Array::from(&custom_tones));
    }
}
