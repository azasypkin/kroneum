use array::Array;
use systick::{SysTick, SysTickHardware};

/// Note durations based on `200 b/m` (beats per minute), see https://msu.edu/course/asc/232/song_project/dectalk_pages/note_to_%20ms.html.
pub const NOTE_1_8_DURATION: u8 = 50;
pub const NOTE_1_4_DURATION: u8 = NOTE_1_8_DURATION * 2;
pub const NOTE_1_2_DURATION: u8 = NOTE_1_4_DURATION * 2;

const ROOT: f32 = 1.059_463_1;

#[derive(Debug, Copy, Clone)]
pub enum Note {
    C0 = 0x10,
    CSharp0 = 0x20,
    D0 = 0x30,
    DSharp0 = 0x40,
    E0 = 0x50,
    F0 = 0x60,
    FSharp0 = 0x70,
    G0 = 0x80,
    GSharp0 = 0x90,
    A0 = 0xA0,
    ASharp0 = 0xB0,
    B0 = 0xC0,

    C1 = 0x11,
    CSharp1 = 0x21,
    D1 = 0x31,
    DSharp1 = 0x41,
    E1 = 0x51,
    F1 = 0x61,
    FSharp1 = 0x71,
    G1 = 0x81,
    GSharp1 = 0x91,
    A1 = 0xA1,
    ASharp1 = 0xB1,
    B1 = 0xC1,

    C2 = 0x12,
    CSharp2 = 0x22,
    D2 = 0x32,
    DSharp2 = 0x42,
    E2 = 0x52,
    F2 = 0x62,
    FSharp2 = 0x72,
    G2 = 0x82,
    GSharp2 = 0x92,
    A2 = 0xA2,
    ASharp2 = 0xB2,
    B2 = 0xC2,

    C3 = 0x13,
    CSharp3 = 0x23,
    D3 = 0x33,
    DSharp3 = 0x43,
    E3 = 0x53,
    F3 = 0x63,
    FSharp3 = 0x73,
    G3 = 0x83,
    GSharp3 = 0x93,
    A3 = 0xA3,
    ASharp3 = 0xB3,
    B3 = 0xC3,

    C4 = 0x14,
    CSharp4 = 0x24,
    D4 = 0x34,
    DSharp4 = 0x44,
    E4 = 0x54,
    F4 = 0x64,
    FSharp4 = 0x74,
    G4 = 0x84,
    GSharp4 = 0x94,
    A4 = 0xA4,
    ASharp4 = 0xB4,
    B4 = 0xC4,

    C5 = 0x15,
    CSharp5 = 0x25,
    D5 = 0x35,
    DSharp5 = 0x45,
    E5 = 0x55,
    F5 = 0x65,
    FSharp5 = 0x75,
    G5 = 0x85,
    GSharp5 = 0x95,
    A5 = 0xA5,
    ASharp5 = 0xB5,
    B5 = 0xC5,

    C6 = 0x16,
    CSharp6 = 0x26,
    D6 = 0x36,
    DSharp6 = 0x46,
    E6 = 0x56,
    F6 = 0x66,
    FSharp6 = 0x76,
    G6 = 0x86,
    GSharp6 = 0x96,
    A6 = 0xA6,
    ASharp6 = 0xB6,
    B6 = 0xC6,

    C7 = 0x17,
    CSharp7 = 0x27,
    D7 = 0x37,
    DSharp7 = 0x47,
    E7 = 0x57,
    F7 = 0x67,
    FSharp7 = 0x77,
    G7 = 0x87,
    GSharp7 = 0x97,
    A7 = 0xA7,
    ASharp7 = 0xB7,
    B7 = 0xC7,

    C8 = 0x18,
    CSharp8 = 0x28,
    D8 = 0x38,
    DSharp8 = 0x48,
    E8 = 0x58,
    F8 = 0x68,
    FSharp8 = 0x78,
    G8 = 0x88,
    GSharp8 = 0x98,
    A8 = 0xA8,
    ASharp8 = 0xB8,
    B8 = 0xC8,

    Silence = 0x00,
}

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
        let power = ((self.note & 0x0f) as i8 - 4) * 12 - (10 - ((self.note & 0xf0) >> 4) as i8);
        let root_power = libm::powf(ROOT, power.into());
        libm::roundf(440_f32 * root_power) as u32
    }
}

/// Defines a predefined melody to play.
#[derive(Debug, Copy, Clone)]
pub enum Melody {
    Alarm,
    Beep,
    Reset,
    Setup,
    Custom(Array<Tone>),
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

        let tones: &[Tone] = match &melody {
            Melody::Alarm => &ALARM_MELODY,
            Melody::Beep => &BEEP_MELODY,
            Melody::Reset => &RESET_MELODY,
            Melody::Setup => &SETUP_MELODY,
            Melody::Custom(tones) => tones.as_ref(),
        };

        tones.iter().for_each(|tone| {
            if tone.duration > 0 {
                self.hw.pulse(tone.frequency());
                self.systick.delay_ms(tone.duration as u32);
            }
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

    #[derive(Copy, Clone, Debug, PartialEq)]
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
            [Some((
                SystickCall::Delay(BEEP_MELODY[0].duration as u32),
                2
            ))],
            systick_mock.calls.ordered_logs(),
        );
        assert_eq!(
            [
                Some((Call::EnablePWM, 0)),
                Some((Call::Pulse(BEEP_MELODY[0].frequency()), 1)),
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
                Some((SystickCall::Delay(BEEP_MELODY[0].duration as u32), 2)),
                Some((SystickCall::Delay(100), 4)),
                Some((SystickCall::Delay(BEEP_MELODY[0].duration as u32), 7)),
                Some((SystickCall::Delay(100), 9)),
                Some((SystickCall::Delay(BEEP_MELODY[0].duration as u32), 12))
            ],
            systick_mock.calls.ordered_logs()
        );

        assert_eq!(
            [
                Some((Call::EnablePWM, 0)),
                Some((Call::Pulse(BEEP_MELODY[0].frequency()), 1)),
                // Delay (2)
                Some((Call::DisablePWM, 3)),
                // Delay 100ms (4)
                Some((Call::EnablePWM, 5)),
                Some((Call::Pulse(BEEP_MELODY[0].frequency()), 6)),
                // Delay (7)
                Some((Call::DisablePWM, 8)),
                // Delay 100ms (9)
                Some((Call::EnablePWM, 10)),
                Some((Call::Pulse(BEEP_MELODY[0].frequency()), 11)),
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
                Some((SystickCall::Delay(SETUP_MELODY[0].duration as u32), 2)),
                Some((SystickCall::Delay(SETUP_MELODY[1].duration as u32), 4)),
            ],
            systick_mock.calls.ordered_logs(),
        );

        assert_eq!(
            [
                Some((Call::EnablePWM, 0)),
                Some((Call::Pulse(SETUP_MELODY[0].frequency()), 1)),
                // Delay (2)
                Some((Call::Pulse(SETUP_MELODY[1].frequency()), 3)),
                // Delay (4)
                Some((Call::DisablePWM, 5))
            ],
            beeper_mock.calls.ordered_logs(),
        );
    }

    #[test]
    fn properly_calculates_notes_frequency() {
        assert_eq!(Tone::new(Note::C0 as u8, 0).frequency(), 16);
        assert_eq!(Tone::new(Note::E3 as u8, 0).frequency(), 165);
        assert_eq!(Tone::new(Note::A4 as u8, 0).frequency(), 440);
        assert_eq!(Tone::new(Note::C5 as u8, 0).frequency(), 523);
        assert_eq!(Tone::new(Note::DSharp7 as u8, 0).frequency(), 2489);
        assert_eq!(Tone::new(Note::B7 as u8, 0).frequency(), 3951)
    }
}
