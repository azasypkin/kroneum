pub mod melody;
pub mod note;
pub mod tone;

use self::{note::Note, tone::Tone};
use array::Array;
use systick::{SysTick, SysTickHardware};

/// Describes the Beeper hardware management interface.
pub trait PWMBeeperHardware {
    /// Enables device PWM output.
    fn enable_pwm(&self);

    /// Disables device PWM output.
    fn disable_pwm(&self);

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
    hw: &'a T,
    systick: &'a mut SysTick<S>,
    state: &'a mut BeeperState,
}

impl<'a, T: PWMBeeperHardware, S: SysTickHardware> PWMBeeper<'a, T, S> {
    pub fn new(hw: &'a T, systick: &'a mut SysTick<S>, state: &'a mut BeeperState) -> Self {
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

        self.hw.enable_pwm();

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
        self.hw.disable_pwm();
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
    use crate::systick::tests::{create_systick, AssociatedData, Call as SystickCall};
    use crate::tests::{MockData, Order};
    use core::cell::RefCell;

    #[derive(Copy, Clone, Debug, PartialEq)]
    enum Call {
        EnablePWM,
        DisablePWM,
        Pulse(u32),
    }

    struct PWMBeeperHardwareMock<'a> {
        pub data: RefCell<MockData<'a, Call>>,
    }

    impl<'a> PWMBeeperHardware for PWMBeeperHardwareMock<'a> {
        fn enable_pwm(&self) {
            self.data.borrow_mut().calls.log_call(Call::EnablePWM);
        }

        fn disable_pwm(&self) {
            self.data.borrow_mut().calls.log_call(Call::DisablePWM);
        }

        fn pulse(&self, note_frequency: u32) {
            self.data
                .borrow_mut()
                .calls
                .log_call(Call::Pulse(note_frequency));
        }
    }

    fn create_tones() -> Array<Tone> {
        Array::from(&[
            Tone::new(Note::C0 as u8, 100),
            Tone::new(Note::B7 as u8, 250),
        ])
    }

    #[test]
    fn play_start_pwm_sound() {
        let order = Order::default();
        let mut systick_mock =
            MockData::with_data_and_call_order(AssociatedData::default(), &order);

        let mut systick = create_systick(&mut systick_mock);
        let mut state = BeeperState::default();
        let tones = create_tones();

        let beeper_hw_mock = PWMBeeperHardwareMock {
            data: RefCell::new(MockData::<Call, ()>::with_call_order(&order)),
        };

        let mut beeper = PWMBeeper::new(&beeper_hw_mock, &mut systick, &mut state);

        assert_eq!(beeper.is_playing(), false);
        beeper.play(tones);
        assert_eq!(beeper.is_playing(), true);

        assert_eq!(
            [
                Some((Call::EnablePWM, 0)),
                Some((Call::Pulse(tones[0].frequency()), 1))
            ],
            beeper.hw.data.borrow().calls.ordered_logs()
        );
        assert_eq!(
            [
                Some((SystickCall::Delay(tones[0].duration as u32), 2)),
                Some((SystickCall::EnableInterrupt, 3)),
                Some((SystickCall::EnableCounter, 4))
            ],
            systick_mock.calls.ordered_logs(),
        );
    }

    #[test]
    fn resume_continues_pwm_sound() {
        let order = Order::default();
        let mut systick_mock =
            MockData::with_data_and_call_order(AssociatedData::default(), &order);

        let mut systick = create_systick(&mut systick_mock);
        let mut state = BeeperState::default();
        let tones = create_tones();

        let beeper_hw_mock = PWMBeeperHardwareMock {
            data: RefCell::new(MockData::<Call, ()>::with_call_order(&order)),
        };

        let mut beeper = PWMBeeper::new(&beeper_hw_mock, &mut systick, &mut state);

        beeper.play(tones);

        beeper.resume();
        assert_eq!(beeper.is_playing(), true);

        assert_eq!(
            [
                Some((Call::EnablePWM, 0)),
                Some((Call::Pulse(tones[0].frequency()), 1)),
                Some((Call::Pulse(tones[1].frequency()), 5))
            ],
            beeper.hw.data.borrow().calls.ordered_logs()
        );
        assert_eq!(
            [
                Some((SystickCall::Delay(tones[0].duration as u32), 2)),
                Some((SystickCall::EnableInterrupt, 3)),
                Some((SystickCall::EnableCounter, 4)),
                Some((SystickCall::Delay(tones[1].duration as u32), 6)),
                Some((SystickCall::EnableInterrupt, 7)),
                Some((SystickCall::EnableCounter, 8))
            ],
            systick_mock.calls.ordered_logs(),
        );
    }

    #[test]
    fn resume_eventually_stops_pwm_sound() {
        let order = Order::default();
        let mut systick_mock =
            MockData::with_data_and_call_order(AssociatedData::default(), &order);

        let mut systick = create_systick(&mut systick_mock);
        let mut state = BeeperState::default();
        let tones = create_tones();

        let beeper_hw_mock = PWMBeeperHardwareMock {
            data: RefCell::new(MockData::<Call, ()>::with_call_order(&order)),
        };

        let mut beeper = PWMBeeper::new(&beeper_hw_mock, &mut systick, &mut state);

        beeper.play(tones);

        beeper.resume();
        beeper.resume();
        assert_eq!(beeper.is_playing(), false);

        assert_eq!(
            [
                Some((Call::EnablePWM, 0)),
                Some((Call::Pulse(tones[0].frequency()), 1)),
                Some((Call::Pulse(tones[1].frequency()), 5)),
                Some((Call::DisablePWM, 9)),
            ],
            beeper.hw.data.borrow().calls.ordered_logs()
        );
        assert_eq!(
            [
                Some((SystickCall::Delay(tones[0].duration as u32), 2)),
                Some((SystickCall::EnableInterrupt, 3)),
                Some((SystickCall::EnableCounter, 4)),
                Some((SystickCall::Delay(tones[1].duration as u32), 6)),
                Some((SystickCall::EnableInterrupt, 7)),
                Some((SystickCall::EnableCounter, 8))
            ],
            systick_mock.calls.ordered_logs(),
        );
    }

    #[test]
    fn stop_immediately_stops_pwm_sound() {
        let order = Order::default();
        let mut systick_mock =
            MockData::with_data_and_call_order(AssociatedData::default(), &order);

        let mut systick = create_systick(&mut systick_mock);
        let mut state = BeeperState::default();
        let tones = create_tones();

        let beeper_hw_mock = PWMBeeperHardwareMock {
            data: RefCell::new(MockData::<Call, ()>::with_call_order(&order)),
        };

        let mut beeper = PWMBeeper::new(&beeper_hw_mock, &mut systick, &mut state);

        beeper.play(tones);

        beeper.stop();
        assert_eq!(beeper.is_playing(), false);

        assert_eq!(
            [
                Some((Call::EnablePWM, 0)),
                Some((Call::Pulse(tones[0].frequency()), 1)),
                Some((Call::DisablePWM, 5)),
            ],
            beeper.hw.data.borrow().calls.ordered_logs()
        );
        assert_eq!(
            [
                Some((SystickCall::Delay(tones[0].duration as u32), 2)),
                Some((SystickCall::EnableInterrupt, 3)),
                Some((SystickCall::EnableCounter, 4)),
            ],
            systick_mock.calls.ordered_logs(),
        );
    }

    #[test]
    fn play_and_repeat_repeats_pwm_sound() {
        let order = Order::default();
        let mut systick_mock =
            MockData::with_data_and_call_order(AssociatedData::default(), &order);

        let mut systick = create_systick(&mut systick_mock);
        let mut state = BeeperState::default();
        let tones = create_tones();

        let beeper_hw_mock = PWMBeeperHardwareMock {
            data: RefCell::new(MockData::<Call, ()>::with_call_order(&order)),
        };

        let mut beeper = PWMBeeper::new(&beeper_hw_mock, &mut systick, &mut state);

        beeper.play_and_repeat(tones, 2);

        // First repetition.
        beeper.resume();
        beeper.resume();
        assert_eq!(beeper.is_playing(), true);

        // Second repetition (silence note).
        beeper.resume();

        // two consequent notes.
        beeper.resume();
        assert_eq!(beeper.is_playing(), true);

        beeper.resume();
        assert_eq!(beeper.is_playing(), false);

        assert_eq!(
            [
                // First repetition.
                Some((Call::EnablePWM, 0)),
                Some((Call::Pulse(tones[0].frequency()), 1)),
                Some((Call::Pulse(tones[1].frequency()), 5)),
                // Silence
                Some((Call::Pulse(0), 9)),
                // Second repetition.
                Some((Call::Pulse(tones[0].frequency()), 13)),
                Some((Call::Pulse(tones[1].frequency()), 17)),
                Some((Call::DisablePWM, 21)),
            ],
            beeper.hw.data.borrow().calls.ordered_logs()
        );
        assert_eq!(
            [
                // First repetition.
                Some((SystickCall::Delay(tones[0].duration as u32), 2)),
                Some((SystickCall::EnableInterrupt, 3)),
                Some((SystickCall::EnableCounter, 4)),
                Some((SystickCall::Delay(tones[1].duration as u32), 6)),
                Some((SystickCall::EnableInterrupt, 7)),
                Some((SystickCall::EnableCounter, 8)),
                // Silence
                Some((SystickCall::Delay(100), 10)),
                Some((SystickCall::EnableInterrupt, 11)),
                Some((SystickCall::EnableCounter, 12)),
                // Second repetition.
                Some((SystickCall::Delay(tones[0].duration as u32), 14)),
                Some((SystickCall::EnableInterrupt, 15)),
                Some((SystickCall::EnableCounter, 16)),
                Some((SystickCall::Delay(tones[1].duration as u32), 18)),
                Some((SystickCall::EnableInterrupt, 19)),
                Some((SystickCall::EnableCounter, 20)),
            ],
            systick_mock.calls.ordered_logs(),
        );
    }
}
