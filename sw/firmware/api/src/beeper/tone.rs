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
