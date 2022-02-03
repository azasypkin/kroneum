use super::note::Note;

const ROOT: f32 = 1.059_463_1;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Tone {
    pub note: u8,
    pub duration: u8,
}

impl Tone {
    pub const fn new(note: u8, duration: u8) -> Self {
        Tone { note, duration }
    }

    /// Note frequencies, see http://pages.mtu.edu/~suits/notefreqs.html.
    /// https://pages.mtu.edu/~suits/NoteFreqCalcs.html
    pub fn frequency(self) -> u32 {
        // Check whether tone's note means silence.
        if self.note == Note::Silence as u8 {
            return 0;
        }

        let power = ((self.note & 0x0f) as i8 - 4) * 12 - (10 - ((self.note & 0xf0) >> 4) as i8);
        let root_power = libm::powf(ROOT, power.into());
        libm::roundf(440_f32 * root_power) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn properly_calculates_frequency() {
        assert_eq!(Tone::new(Note::Silence as u8, 0).frequency(), 0);
        assert_eq!(Tone::new(Note::C0 as u8, 0).frequency(), 16);
        assert_eq!(Tone::new(Note::E3 as u8, 0).frequency(), 165);
        assert_eq!(Tone::new(Note::A4 as u8, 0).frequency(), 440);
        assert_eq!(Tone::new(Note::C5 as u8, 0).frequency(), 523);
        assert_eq!(Tone::new(Note::DSharp7 as u8, 0).frequency(), 2489);
        assert_eq!(Tone::new(Note::B7 as u8, 0).frequency(), 3951)
    }

    #[test]
    fn properly_calculates_frequency_2() {
        assert_eq!(Tone::new(Note::A5 as u8, 0).frequency(), 880);
        assert_eq!(Tone::new(Note::ASharp5 as u8, 0).frequency(), 932);
        assert_eq!(Tone::new(Note::B5 as u8, 0).frequency(), 988);
        assert_eq!(Tone::new(Note::C6 as u8, 0).frequency(), 1047);
        assert_eq!(Tone::new(Note::CSharp6 as u8, 0).frequency(), 1109);
        assert_eq!(Tone::new(Note::D6 as u8, 0).frequency(), 1175);
        assert_eq!(Tone::new(Note::DSharp6 as u8, 0).frequency(), 1245);
        assert_eq!(Tone::new(Note::E6 as u8, 0).frequency(), 1319);
        assert_eq!(Tone::new(Note::F6 as u8, 0).frequency(), 1397);
        assert_eq!(Tone::new(Note::FSharp6 as u8, 0).frequency(), 1480);
        assert_eq!(Tone::new(Note::G6 as u8, 0).frequency(), 1568);
        assert_eq!(Tone::new(Note::GSharp6 as u8, 0).frequency(), 1661);
        assert_eq!(Tone::new(Note::A6 as u8, 0).frequency(), 1760);
        assert_eq!(Tone::new(Note::G5 as u8, 0).frequency(), 1760);
    }

    #[test]
    fn properly_keeps_note_and_duration() {
        let tone = Tone::new(Note::Silence as u8, 0);
        assert_eq!(tone.duration, 0);
        assert_eq!(tone.note, Note::Silence as u8);

        let tone = Tone::new(Note::C0 as u8, 100);
        assert_eq!(tone.duration, 100);
        assert_eq!(tone.note, Note::C0 as u8);

        let tone = Tone::new(Note::B7 as u8, 250);
        assert_eq!(tone.duration, 250);
        assert_eq!(tone.note, Note::B7 as u8);
    }
}
