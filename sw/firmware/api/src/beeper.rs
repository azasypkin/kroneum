pub mod melody;
pub mod note;
pub mod tone;

use self::{note::Note, tone::Tone};
use array::Array;
use systick::{SysTick, SysTickHardware};

/// Describes the Beeper hardware management interface.
pub trait PWMBeeperHardware {
    /// Toggles on/off device PWM output.
    fn toggle_pwm(&self, enable: bool);

    /// Forces PWM to pulse of the specified frequency.
    fn pulse(&self, note_frequency: u32);
}

#[derive(Copy, Clone, Default)]
pub struct TonesToPlay {
    tones: Array<Tone>,
    current_index: usize,
    repeat: usize,
}

#[derive(Copy, Clone, Default)]
pub struct BeeperState {
    pub tones_to_play: Option<TonesToPlay>,
}

pub struct PWMBeeper<'a, T: PWMBeeperHardware, S: SysTickHardware> {
    hw: T,
    systick: &'a mut SysTick<S>,
    state: &'a mut BeeperState,
}

impl<'a, T: PWMBeeperHardware, S: SysTickHardware> PWMBeeper<'a, T, S> {
    pub fn new(hw: T, systick: &'a mut SysTick<S>, state: &'a mut BeeperState) -> Self {
        PWMBeeper { hw, systick, state }
    }

    /// Starts playing specified tones in a sequence.
    pub fn play<TN: Into<Array<Tone>> + Sized>(&mut self, tones: TN) {
        self.play_and_repeat(tones, 1);
    }

    /// Starts playing specified `tones` in a sequence repeating it `repeat` number of times with a
    /// `100ms` delay between repetitions.
    pub fn play_and_repeat<TN: Into<Array<Tone>> + Sized>(&mut self, tones: TN, repeat: usize) {
        // Stop current melody if there's any.
        if self.is_playing() {
            self.stop();
        }

        self.hw.toggle_pwm(true);

        self.state.tones_to_play = Some(TonesToPlay {
            tones: tones.into(),
            current_index: 0,
            repeat,
        });

        self.resume();
    }

    /// Stops current melody.
    pub fn stop(&mut self) {
        self.state.tones_to_play = None;
        self.hw.toggle_pwm(false);
    }

    /// Asks Beeper to resume from the next tone.
    pub fn resume(&mut self) {
        if let Some(TonesToPlay {
            tones,
            current_index,
            repeat,
        }) = self.state.tones_to_play
        {
            if current_index < tones.len() {
                self.play_tone(tones[current_index]);

                self.state.tones_to_play.replace(TonesToPlay {
                    tones,
                    current_index: current_index + 1,
                    repeat,
                });
            } else if repeat > 1 {
                // Make a 100ms pause between repetitions.
                self.play_tone(Tone::new(Note::Silence as u8, 100));

                self.state.tones_to_play.replace(TonesToPlay {
                    tones,
                    current_index: 0,
                    repeat: repeat - 1,
                });
            } else {
                self.stop();
            }
        }
    }

    /// Checks whether there is melody is playing.
    pub fn is_playing(&self) -> bool {
        self.state.tones_to_play.is_some()
    }

    fn play_tone(&mut self, tone: Tone) {
        if tone.duration == 0 {
            return;
        }

        self.hw.pulse(tone.frequency());
        self.systick.start(tone.duration as u32);
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
        assert_eq!(Tone::new(Note::Silence as u8, 0).frequency(), 0);
        assert_eq!(Tone::new(Note::C0 as u8, 0).frequency(), 16);
        assert_eq!(Tone::new(Note::E3 as u8, 0).frequency(), 165);
        assert_eq!(Tone::new(Note::A4 as u8, 0).frequency(), 440);
        assert_eq!(Tone::new(Note::C5 as u8, 0).frequency(), 523);
        assert_eq!(Tone::new(Note::DSharp7 as u8, 0).frequency(), 2489);
        assert_eq!(Tone::new(Note::B7 as u8, 0).frequency(), 3951)
    }
}
