/// Note durations based on `200 b/m` (beats per minute), see https://msu.edu/course/asc/232/song_project/dectalk_pages/note_to_%20ms.html.
const EIGHTH_NOTE: u32 = 150;
const QUARTER_DOT_NOTE: u32 = 450;
const QUARTER_NOTE: u32 = 300;

/// Note frequencies, see http://pages.mtu.edu/~suits/notefreqs.html.
const NOTE_FREQUENCIES: [u32; 12] = [523, 554, 587, 622, 659, 698, 740, 784, 831, 880, 932, 988];

/// Defines a predefined melody to play.
#[derive(Debug, Copy, Clone)]
pub enum Melody {
    Alarm,
    Beep,
    Reset,
    Setup,
}

/// Melody that is being played when alarm triggers.
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

/// Melody to be used as beep (e.g. when setting alarm).
const BEEP_MELODY: [(u32, u32); 1] = [(NOTE_FREQUENCIES[7], EIGHTH_NOTE)];

/// Melody that is played when alarm is reset.
const RESET_MELODY: [(u32, u32); 6] = [
    (NOTE_FREQUENCIES[5], QUARTER_NOTE),     // F
    (NOTE_FREQUENCIES[5], EIGHTH_NOTE),      // F
    (NOTE_FREQUENCIES[7], QUARTER_DOT_NOTE), // G.
    (NOTE_FREQUENCIES[5], QUARTER_NOTE),     // F
    (NOTE_FREQUENCIES[5], EIGHTH_NOTE),      // F
    (NOTE_FREQUENCIES[7], QUARTER_DOT_NOTE), // G.
];

/// Melody that is played when user enters setup mode.
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
    pub fn beep_n(&mut self, n: u8) {
        for i in 1..=n {
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

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::RefCell;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    enum Call {
        EnablePWM,
        DisablePWM,
        Delay(u32),
        Pulse(u32),
        Unknown,
    }

    struct MockData {
        pub calls: [Call; 15],
        pub pointer: usize,
    }

    impl MockData {
        pub fn new() -> Self {
            MockData {
                calls: [Call::Unknown; 15],
                pointer: 0,
            }
        }
    }

    struct PWMBeeperHardwareMock<'a> {
        data: RefCell<&'a mut MockData>,
    }

    impl<'a> PWMBeeperHardwareMock<'a> {
        pub fn register_call(&self, call: Call) {
            let mut data = self.data.borrow_mut();
            let pointer = data.pointer;
            data.calls[pointer] = call;
            data.pointer += 1;
        }
    }

    impl<'a> PWMBeeperHardware for PWMBeeperHardwareMock<'a> {
        fn toggle_pwm(&self, enable: bool) {
            if enable {
                self.register_call(Call::EnablePWM);
            } else {
                self.register_call(Call::DisablePWM);
            }
        }

        fn pulse(&mut self, note_frequency: u32) {
            self.register_call(Call::Pulse(note_frequency));
        }

        fn delay(&mut self, delay_ms: u32) {
            self.register_call(Call::Delay(delay_ms));
        }
    }

    fn get_beeper(mock_data: &mut MockData) -> PWMBeeper<PWMBeeperHardwareMock> {
        PWMBeeper {
            hw: PWMBeeperHardwareMock {
                data: RefCell::new(mock_data),
            },
        }
    }

    #[test]
    fn handles_beep() {
        let mut mock_data = MockData::new();

        get_beeper(&mut mock_data).beep();
        assert_eq!(mock_data.pointer, 4);
        assert_eq!(
            mock_data.calls[..mock_data.pointer],
            [
                Call::EnablePWM,
                Call::Pulse(BEEP_MELODY[0].0),
                Call::Delay(BEEP_MELODY[0].1),
                Call::DisablePWM
            ]
        );
    }

    #[test]
    fn handles_beep_n() {
        let mut mock_data = MockData::new();

        get_beeper(&mut mock_data).beep_n(3);
        assert_eq!(mock_data.pointer, 14);
        assert_eq!(
            mock_data.calls[..mock_data.pointer],
            [
                Call::EnablePWM,
                Call::Pulse(BEEP_MELODY[0].0),
                Call::Delay(BEEP_MELODY[0].1),
                Call::DisablePWM,
                Call::Delay(100),
                Call::EnablePWM,
                Call::Pulse(BEEP_MELODY[0].0),
                Call::Delay(BEEP_MELODY[0].1),
                Call::DisablePWM,
                Call::Delay(100),
                Call::EnablePWM,
                Call::Pulse(BEEP_MELODY[0].0),
                Call::Delay(BEEP_MELODY[0].1),
                Call::DisablePWM
            ]
        );
    }

    #[test]
    fn handles_play() {
        let mut mock_data = MockData::new();

        get_beeper(&mut mock_data).play(Melody::Setup);
        assert_eq!(mock_data.pointer, 6);
        assert_eq!(
            mock_data.calls[..mock_data.pointer],
            [
                Call::EnablePWM,
                Call::Pulse(SETUP_MELODY[0].0),
                Call::Delay(SETUP_MELODY[0].1),
                Call::Pulse(SETUP_MELODY[1].0),
                Call::Delay(SETUP_MELODY[1].1),
                Call::DisablePWM
            ]
        );
    }
}
