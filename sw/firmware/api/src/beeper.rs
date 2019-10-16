use systick::{SysTick, SysTickHardware};

/// Note durations based on `200 b/m` (beats per minute), see https://msu.edu/course/asc/232/song_project/dectalk_pages/note_to_%20ms.html.
const BEATS_PER_MINUTE_BASE: u32 = 50;
const NOTE_1_8_DURATION: u32 = BEATS_PER_MINUTE_BASE;
const NOTE_1_4_DURATION: u32 = BEATS_PER_MINUTE_BASE * 2;
const NOTE_3_8_DURATION: u32 = BEATS_PER_MINUTE_BASE * 3;
const NOTE_1_2_DURATION: u32 = BEATS_PER_MINUTE_BASE * 4;

const ROOT: f32 = 1.059_463_1;

#[allow(dead_code)]
enum Note {
    C(u8),
    CSharp(u8),
    D(u8),
    DSharp(u8),
    E(u8),
    F(u8),
    FSharp(u8),
    G(u8),
    GSharp(u8),
    A(u8),
    ASharp(u8),
    B(u8),
    Silence,
}

impl Note {
    /// Note frequencies, see http://pages.mtu.edu/~suits/notefreqs.html.
    /// https://pages.mtu.edu/~suits/NoteFreqCalcs.html
    fn calculate_frequency(n: u8, order: i8) -> u32 {
        let power = (n as i8 - 4) * 12 - order;
        let root_power = libm::powf(ROOT, power.into());
        libm::roundf(440_f32 * root_power) as u32
    }

    pub fn frequency(&self) -> u32 {
        match self {
            Note::C(n) => Note::calculate_frequency(*n, 9),
            Note::CSharp(n) => Note::calculate_frequency(*n, 8),
            Note::D(n) => Note::calculate_frequency(*n, 7),
            Note::DSharp(n) => Note::calculate_frequency(*n, 6),
            Note::E(n) => Note::calculate_frequency(*n, 5),
            Note::F(n) => Note::calculate_frequency(*n, 4),
            Note::FSharp(n) => Note::calculate_frequency(*n, 3),
            Note::G(n) => Note::calculate_frequency(*n, 2),
            Note::GSharp(n) => Note::calculate_frequency(*n, 1),
            Note::A(n) => Note::calculate_frequency(*n, 0),
            Note::ASharp(n) => Note::calculate_frequency(*n, -1),
            Note::B(n) => Note::calculate_frequency(*n, -2),
            Note::Silence => 0,
        }
    }
}

/// Defines a predefined melody to play.
#[derive(Debug, Copy, Clone)]
pub enum Melody {
    Alarm,
    Beep,
    Reset,
    Setup,
}

/// Melody that is being played when alarm triggers.
const ALARM_MELODY: [(Note, u32); 24] = [
    (Note::B(7), NOTE_1_4_DURATION),
    (Note::GSharp(7), NOTE_1_4_DURATION),
    (Note::DSharp(7), NOTE_1_2_DURATION),
    (Note::GSharp(7), NOTE_1_4_DURATION),
    (Note::DSharp(7), NOTE_1_4_DURATION),
    (Note::FSharp(7), NOTE_1_2_DURATION),
    (Note::DSharp(7), NOTE_1_4_DURATION),
    (Note::FSharp(7), NOTE_1_4_DURATION),
    (Note::DSharp(7), NOTE_1_2_DURATION),
    (Note::Silence, NOTE_1_2_DURATION),
    (Note::DSharp(7), NOTE_1_2_DURATION),
    (Note::FSharp(7), NOTE_1_4_DURATION),
    (Note::DSharp(7), NOTE_1_4_DURATION),
    (Note::F(7), NOTE_1_4_DURATION),
    (Note::DSharp(7), NOTE_1_4_DURATION),
    (Note::F(7), NOTE_1_4_DURATION),
    (Note::DSharp(7), NOTE_1_4_DURATION),
    (Note::D(7), NOTE_1_4_DURATION),
    (Note::F(7), NOTE_1_4_DURATION),
    (Note::CSharp(7), NOTE_1_4_DURATION),
    (Note::F(7), NOTE_1_4_DURATION),
    (Note::FSharp(7), NOTE_1_2_DURATION),
    (Note::DSharp(7), NOTE_1_2_DURATION),
    (Note::Silence, NOTE_1_2_DURATION),
];

/// Melody to be used as beep (e.g. when setting alarm).
const BEEP_MELODY: [(Note, u32); 1] = [(Note::G(5), NOTE_1_4_DURATION)];

/// Melody that is played when alarm is reset.
const RESET_MELODY: [(Note, u32); 6] = [
    (Note::F(5), NOTE_1_4_DURATION),
    (Note::F(5), NOTE_1_8_DURATION),
    (Note::G(5), NOTE_3_8_DURATION),
    (Note::F(5), NOTE_1_4_DURATION),
    (Note::F(5), NOTE_1_8_DURATION),
    (Note::G(5), NOTE_3_8_DURATION),
];

/// Melody that is played when user enters setup mode.
const SETUP_MELODY: [(Note, u32); 2] = [
    (Note::DSharp(5), NOTE_1_4_DURATION),
    (Note::DSharp(5), NOTE_1_4_DURATION),
];

/// Describes the Beeper hardware management interface.
pub trait PWMBeeperHardware {
    /// Toggles on/off device PWM output.
    fn toggle_pwm(&self, enable: bool);

    /// Forces PWM to pulse of the specified frequency.
    fn pulse(&self, note_frequency: u32);
}

pub struct PWMBeeper<'a, T: PWMBeeperHardware, S: SysTickHardware> {
    hw: T,
    systick: &'a mut SysTick<S>,
}

impl<'a, T: PWMBeeperHardware, S: SysTickHardware> PWMBeeper<'a, T, S> {
    pub fn new(hw: T, systick: &'a mut SysTick<S>) -> Self {
        PWMBeeper { hw, systick }
    }

    /// Plays predefined melody.
    pub fn play(&mut self, melody: Melody) {
        self.hw.toggle_pwm(true);

        let notes: &[(Note, u32)] = match melody {
            Melody::Alarm => &ALARM_MELODY,
            Melody::Beep => &BEEP_MELODY,
            Melody::Reset => &RESET_MELODY,
            Melody::Setup => &SETUP_MELODY,
        };

        notes.iter().for_each(|(note, delay)| {
            self.hw.pulse(note.frequency());
            self.systick.delay_ms(*delay);
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
                self.systick.delay_ms(100);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systick::tests::{
        create_systick, AssociatedData, Call as SystickCall, SystickHardwareMock,
    };
    use crate::tests::{MockData, Order};
    use core::cell::RefCell;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    enum Call {
        EnablePWM,
        DisablePWM,
        Pulse(u32),
    }

    struct PWMBeeperHardwareMock<'a, 'b: 'a> {
        data: RefCell<&'a mut MockData<'b, Call>>,
    }

    impl<'a, 'b: 'a> PWMBeeperHardware for PWMBeeperHardwareMock<'a, 'b> {
        fn toggle_pwm(&self, enable: bool) {
            self.data.borrow_mut().calls.log_call(if enable {
                Call::EnablePWM
            } else {
                Call::DisablePWM
            });
        }

        fn pulse(&self, note_frequency: u32) {
            self.data
                .borrow_mut()
                .calls
                .log_call(Call::Pulse(note_frequency));
        }
    }

    fn create_beeper<'a, 'b: 'a>(
        beeper_mock: &'a mut MockData<'b, Call>,
        systick: &'a mut SysTick<SystickHardwareMock<'a, 'b>>,
    ) -> PWMBeeper<'a, PWMBeeperHardwareMock<'a, 'b>, SystickHardwareMock<'a, 'b>> {
        PWMBeeper::new(
            PWMBeeperHardwareMock {
                data: RefCell::new(beeper_mock),
            },
            systick,
        )
    }

    #[test]
    fn handles_beep() {
        let order = Order::default();
        let mut beeper_mock = MockData::<Call, ()>::with_call_order(&order);
        let mut systick_mock =
            MockData::with_data_and_call_order(AssociatedData::default(), &order);

        create_beeper(&mut beeper_mock, &mut create_systick(&mut systick_mock)).beep();

        assert_eq!(
            [Some((SystickCall::Delay(BEEP_MELODY[0].1), 2))],
            systick_mock.calls.ordered_logs(),
        );
        assert_eq!(
            [
                Some((Call::EnablePWM, 0)),
                Some((Call::Pulse(BEEP_MELODY[0].0.frequency()), 1)),
                // Delay (order 2)
                Some((Call::DisablePWM, 3))
            ],
            beeper_mock.calls.ordered_logs()
        );
    }

    #[test]
    fn handles_beep_n() {
        let order = Order::default();
        let mut beeper_mock = MockData::<Call, ()>::with_call_order(&order);
        let mut systick_mock =
            MockData::with_data_and_call_order(AssociatedData::default(), &order);

        create_beeper(&mut beeper_mock, &mut create_systick(&mut systick_mock)).beep_n(3);

        assert_eq!(
            [
                Some((SystickCall::Delay(BEEP_MELODY[0].1), 2)),
                Some((SystickCall::Delay(100), 4)),
                Some((SystickCall::Delay(BEEP_MELODY[0].1), 7)),
                Some((SystickCall::Delay(100), 9)),
                Some((SystickCall::Delay(BEEP_MELODY[0].1), 12))
            ],
            systick_mock.calls.ordered_logs()
        );

        assert_eq!(
            [
                Some((Call::EnablePWM, 0)),
                Some((Call::Pulse(BEEP_MELODY[0].0.frequency()), 1)),
                // Delay (2)
                Some((Call::DisablePWM, 3)),
                // Delay 100ms (4)
                Some((Call::EnablePWM, 5)),
                Some((Call::Pulse(BEEP_MELODY[0].0.frequency()), 6)),
                // Delay (7)
                Some((Call::DisablePWM, 8)),
                // Delay 100ms (9)
                Some((Call::EnablePWM, 10)),
                Some((Call::Pulse(BEEP_MELODY[0].0.frequency()), 11)),
                // Delay (12)
                Some((Call::DisablePWM, 13))
            ],
            beeper_mock.calls.ordered_logs()
        );
    }

    #[test]
    fn handles_play() {
        let order = Order::default();
        let mut beeper_mock = MockData::<Call, ()>::with_call_order(&order);
        let mut systick_mock =
            MockData::with_data_and_call_order(AssociatedData::default(), &order);

        create_beeper(&mut beeper_mock, &mut create_systick(&mut systick_mock)).play(Melody::Setup);

        assert_eq!(
            [
                Some((SystickCall::Delay(SETUP_MELODY[0].1), 2)),
                Some((SystickCall::Delay(SETUP_MELODY[1].1), 4)),
            ],
            systick_mock.calls.ordered_logs(),
        );

        assert_eq!(
            [
                Some((Call::EnablePWM, 0)),
                Some((Call::Pulse(SETUP_MELODY[0].0.frequency()), 1)),
                // Delay (2)
                Some((Call::Pulse(SETUP_MELODY[1].0.frequency()), 3)),
                // Delay (4)
                Some((Call::DisablePWM, 5))
            ],
            beeper_mock.calls.ordered_logs(),
        );
    }

    #[test]
    fn properly_calculates_notes_frequency() {
        assert_eq!(Note::C(0).frequency(), 16);
        assert_eq!(Note::E(3).frequency(), 165);
        assert_eq!(Note::A(4).frequency(), 440);
        assert_eq!(Note::C(5).frequency(), 523);
        assert_eq!(Note::DSharp(7).frequency(), 2489);
        assert_eq!(Note::B(7).frequency(), 3951);
    }
}
