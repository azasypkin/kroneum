use systick::{SysTick, SysTickHardware};

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

        let notes: &[(u32, u32)] = match melody {
            Melody::Alarm => &ALARM_MELODY,
            Melody::Beep => &BEEP_MELODY,
            Melody::Reset => &RESET_MELODY,
            Melody::Setup => &SETUP_MELODY,
        };

        notes.iter().for_each(|(frequency, delay)| {
            self.hw.pulse(*frequency);
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
                Some((Call::Pulse(BEEP_MELODY[0].0), 1)),
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
                Some((Call::Pulse(BEEP_MELODY[0].0), 1)),
                // Delay (2)
                Some((Call::DisablePWM, 3)),
                // Delay 100ms (4)
                Some((Call::EnablePWM, 5)),
                Some((Call::Pulse(BEEP_MELODY[0].0), 6)),
                // Delay (7)
                Some((Call::DisablePWM, 8)),
                // Delay 100ms (9)
                Some((Call::EnablePWM, 10)),
                Some((Call::Pulse(BEEP_MELODY[0].0), 11)),
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
                Some((Call::Pulse(SETUP_MELODY[0].0), 1)),
                // Delay (2)
                Some((Call::Pulse(SETUP_MELODY[1].0), 3)),
                // Delay (4)
                Some((Call::DisablePWM, 5))
            ],
            beeper_mock.calls.ordered_logs(),
        );
    }
}
