/// Note durations based on `200 b/m` (beats per minute), see https://msu.edu/course/asc/232/song_project/dectalk_pages/note_to_%20ms.html.
const EIGHTH_NOTE: u32 = 150;
const QUARTER_DOT_NOTE: u32 = 450;
const QUARTER_NOTE: u32 = 300;

/// Note frequencies, see http://pages.mtu.edu/~suits/notefreqs.html.
const NOTE_FREQUENCIES: [u32; 12] = [523, 554, 587, 622, 659, 698, 740, 784, 831, 880, 932, 988];

pub enum Melody {
    Alarm,
    Beep,
    Reset,
    Setup,
}

const ALARM_MELODY: [(u32, u32); 15] = [
    (NOTE_FREQUENCIES[7], QUARTER_NOTE),     // G
    (NOTE_FREQUENCIES[7], QUARTER_NOTE),     // G
    (NOTE_FREQUENCIES[8], QUARTER_NOTE),     // A
    (NOTE_FREQUENCIES[10], QUARTER_NOTE),    // B
    (NOTE_FREQUENCIES[10], QUARTER_NOTE),    // B
    (NOTE_FREQUENCIES[8], QUARTER_NOTE),     // A
    (NOTE_FREQUENCIES[7], QUARTER_NOTE),     // G
    (NOTE_FREQUENCIES[5], QUARTER_NOTE),     // F
    (NOTE_FREQUENCIES[3], QUARTER_NOTE),     // D#
    (NOTE_FREQUENCIES[3], QUARTER_NOTE),     // E
    (NOTE_FREQUENCIES[5], QUARTER_NOTE),     // F
    (NOTE_FREQUENCIES[7], QUARTER_NOTE),     // G
    (NOTE_FREQUENCIES[7], QUARTER_DOT_NOTE), // G.
    (NOTE_FREQUENCIES[5], EIGHTH_NOTE),      // F
    (NOTE_FREQUENCIES[5], QUARTER_DOT_NOTE),
];

const BEEP_MELODY: [(u32, u32); 1] = [(NOTE_FREQUENCIES[7], EIGHTH_NOTE)];

const RESET_MELODY: [(u32, u32); 6] = [
    (NOTE_FREQUENCIES[5], QUARTER_NOTE),     // F
    (NOTE_FREQUENCIES[5], EIGHTH_NOTE),      // F
    (NOTE_FREQUENCIES[7], QUARTER_DOT_NOTE), // G.
    (NOTE_FREQUENCIES[5], QUARTER_NOTE),     // F
    (NOTE_FREQUENCIES[5], EIGHTH_NOTE),      // F
    (NOTE_FREQUENCIES[7], QUARTER_DOT_NOTE), // G.
];

const SETUP_MELODY: [(u32, u32); 2] = [
    (NOTE_FREQUENCIES[3], QUARTER_NOTE), // D#
    (NOTE_FREQUENCIES[3], QUARTER_NOTE), // E
];

/// Describes the Beeper hardware management interface.
pub trait PWMBeeperHardware {
    /// Toggles on/off device PWM output.
    fn toggle_pwm(&self, enable: bool);

    /// Forces PWM to pulse of the specified frequency.
    fn pulse(&mut self, note_frequency: u32);

    /// Blocks the thread for the specified number of milliseconds.
    fn delay(&mut self, delay_ms: u32);
}

pub struct PWMBeeper<T: PWMBeeperHardware> {
    hw: T,
}

impl<T: PWMBeeperHardware> PWMBeeper<T> {
    pub fn create(hw: T) -> Self {
        PWMBeeper { hw }
    }

    /// Plays predefined melody.
    pub fn play(&mut self, melody: Melody) {
        self.hw.toggle_pwm(true);

        let notes: &[(u32, u32)] = match melody {
            Melody::Alarm => &ALARM_MELODY,
            Melody::Beep => &BEEP_MELODY,
            Melody::Reset => &RESET_MELODY,
            Melody::Setup => &SETUP_MELODY,
        };

        notes.iter().for_each(|(frequency, delay)| {
            self.play_note(frequency.clone(), delay.clone());
        });

        self.hw.toggle_pwm(false);
    }

    /// Produces one audible beep.
    pub fn beep(&mut self) {
        self.play(Melody::Beep);
    }

    /// Produces `n` audible beeps with `100ms` delay.
    ///
    pub fn beep_n(&mut self, n: u8) {
        for i in 1..n + 1 {
            self.play(Melody::Beep);

            if i < n {
                self.hw.delay(100);
            }
        }
    }

    fn play_note(&mut self, note_frequency: u32, delay: u32) {
        self.hw.pulse(note_frequency);
        self.hw.delay(delay);
    }
}
