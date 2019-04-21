use systick::{SysTick, SysTickHardware};

/// Defines known button types.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum ButtonType {
    One,
    Ten,
}

/// Defines type of the press (short, long, very long).
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ButtonPressType {
    /// Button is not pressed.
    None,
    /// Button is keep pressed for less then a second.
    Short,
    /// Button is pressed for more than a second, but less than 2 seconds.
    Long,
}

impl ButtonPressType {
    pub fn is_none(self) -> bool {
        match self {
            ButtonPressType::None => true,
            _ => false,
        }
    }
}

/// Describes the Buttons hardware management interface.
pub trait ButtonsHardware {
    /// Initializes hardware if needed.
    fn setup(&self);

    /// Releases hardware if needed.
    fn teardown(&self);

    /// Checks whether Button with specified type is pressed.
    fn is_button_pressed(&self, button_type: ButtonType) -> bool;

    /// Checks whether button with the specified type was triggered (e.g. by interrupt). Being
    /// triggered doesn't mean to be pressed, button could have been triggered, but isn't pressed
    /// anymore when this method is called.
    fn is_button_triggered(&self, button_type: ButtonType) -> bool;

    /// Clears "triggered" flag for a button and makes it listen to a new trigger event.
    fn reactivate_button(&self, button_type: ButtonType);
}

pub struct Buttons<'a, T: ButtonsHardware, S: SysTickHardware> {
    hw: T,
    systick: &'a mut SysTick<S>,
}

impl<'a, T: ButtonsHardware, S: SysTickHardware> Buttons<'a, T, S> {
    pub fn new(hw: T, systick: &'a mut SysTick<S>) -> Self {
        Buttons { hw, systick }
    }

    /// Setups Buttons hardware.
    pub fn setup(&self) {
        self.hw.setup()
    }

    /// Tears down Buttons hardware.
    pub fn teardown(&self) {
        self.hw.teardown()
    }

    pub fn interrupt(&mut self) -> (ButtonPressType, ButtonPressType) {
        let button_one_triggered = self.hw.is_button_triggered(ButtonType::One);
        let mut button_one_state =
            if button_one_triggered && self.hw.is_button_pressed(ButtonType::One) {
                ButtonPressType::Short
            } else {
                ButtonPressType::None
            };

        let button_ten_triggered = self.hw.is_button_triggered(ButtonType::Ten);
        let mut button_ten_state =
            if button_ten_triggered && self.hw.is_button_pressed(ButtonType::Ten) {
                ButtonPressType::Short
            } else {
                ButtonPressType::None
            };

        if !button_one_state.is_none() || !button_ten_state.is_none() {
            for i in 1u8..8u8 {
                self.systick.delay_ms(250);

                let button_one_pressed =
                    button_one_triggered && self.hw.is_button_pressed(ButtonType::One);
                let button_ten_pressed =
                    button_ten_triggered && self.hw.is_button_pressed(ButtonType::Ten);
                if !button_one_pressed && !button_ten_pressed {
                    break;
                }

                let (new_state, works_for_none) = match i {
                    0...4 => (ButtonPressType::Short, true),
                    5...8 => (ButtonPressType::Long, false),
                    _ => break,
                };

                if button_one_pressed && (!button_one_state.is_none() || works_for_none) {
                    button_one_state = new_state;
                }

                if button_ten_pressed && (!button_ten_state.is_none() || works_for_none) {
                    button_ten_state = new_state;
                }
            }
        }

        if button_one_triggered {
            self.hw.reactivate_button(ButtonType::One);
        }

        if button_ten_triggered {
            self.hw.reactivate_button(ButtonType::Ten);
        }

        (button_one_state, button_ten_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::systick::tests::{
        create_systick, AssociatedData as SystickAssociatedData, Clock, SystickHardwareMock,
    };
    use crate::tests::MockData;
    use core::cell::RefCell;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    enum Call {
        Setup,
        Teardown,
        Reactivate(ButtonType),
    }

    pub(crate) struct AssociatedData<'a, F: Fn(ButtonType, u32) -> bool, FA: Fn(ButtonType) -> bool> {
        pub is_button_pressed: F,
        pub is_button_triggered: FA,
        pub clock: &'a Clock,
    }

    struct ButtonsHardwareMock<
        'a,
        'b: 'a,
        F: Fn(ButtonType, u32) -> bool,
        FA: Fn(ButtonType) -> bool,
    > {
        data: RefCell<&'a mut MockData<'b, Call, AssociatedData<'b, F, FA>>>,
    }

    impl<'a, 'b: 'a, F: Fn(ButtonType, u32) -> bool, FA: Fn(ButtonType) -> bool> ButtonsHardware
        for ButtonsHardwareMock<'a, 'b, F, FA>
    {
        fn setup(&self) {
            self.data.borrow_mut().calls.log_call(Call::Setup);
        }

        fn teardown(&self) {
            self.data.borrow_mut().calls.log_call(Call::Teardown);
        }

        fn is_button_pressed(&self, button_type: ButtonType) -> bool {
            let current_delay = self.data.borrow().data.clock.ticks();
            (self.data.borrow().data.is_button_pressed)(button_type, current_delay)
        }

        fn is_button_triggered(&self, button_type: ButtonType) -> bool {
            (self.data.borrow().data.is_button_triggered)(button_type)
        }

        fn reactivate_button(&self, button_type: ButtonType) {
            self.data
                .borrow_mut()
                .calls
                .log_call(Call::Reactivate(button_type));
        }
    }

    fn create_buttons<'a, 'b: 'a, F: Fn(ButtonType, u32) -> bool, FA: Fn(ButtonType) -> bool>(
        mock_data: &'a mut MockData<'b, Call, AssociatedData<'b, F, FA>>,
        systick: &'a mut SysTick<SystickHardwareMock<'a, 'b>>,
    ) -> Buttons<'a, ButtonsHardwareMock<'a, 'b, F, FA>, SystickHardwareMock<'a, 'b>> {
        Buttons::new(
            ButtonsHardwareMock {
                data: RefCell::new(mock_data),
            },
            systick,
        )
    }

    #[test]
    fn setup() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData::default());
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| false,
            is_button_triggered: |_: ButtonType| false,
            clock: &clock,
        });

        create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).setup();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Setup)])
    }

    #[test]
    fn teardown() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData::default());
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| false,
            is_button_triggered: |_: ButtonType| false,
            clock: &clock,
        });

        create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).teardown();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Teardown)])
    }

    #[test]
    fn does_not_reactivate_if_was_not_triggered() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData::default());
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| false,
            is_button_triggered: |_: ButtonType| false,
            clock: &clock,
        });

        create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt();

        assert_eq!(mock_data.calls.logs(), [])
    }

    #[test]
    fn reactivates_one_only_if_only_one_was_triggered() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData::default());
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| false,
            is_button_triggered: |bt: ButtonType| match bt {
                ButtonType::One => true,
                ButtonType::Ten => false,
            },
            clock: &clock,
        });

        create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt();

        assert_eq!(
            [Some(Call::Reactivate(ButtonType::One))],
            mock_data.calls.logs(),
        )
    }

    #[test]
    fn reactivates_ten_only_if_only_ten_was_triggered() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData::default());
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| false,
            is_button_triggered: |bt: ButtonType| match bt {
                ButtonType::One => false,
                ButtonType::Ten => true,
            },
            clock: &clock,
        });

        create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt();

        assert_eq!(
            [Some(Call::Reactivate(ButtonType::Ten))],
            mock_data.calls.logs(),
        )
    }

    #[test]
    fn reactivates_both_if_both_were_triggered() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData::default());
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| false,
            is_button_triggered: |_: ButtonType| true,
            clock: &clock,
        });

        create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt();

        assert_eq!(
            [
                Some(Call::Reactivate(ButtonType::One)),
                Some(Call::Reactivate(ButtonType::Ten))
            ],
            mock_data.calls.logs(),
        )
    }

    #[test]
    fn both_none() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData::default());
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| false,
            is_button_triggered: |_: ButtonType| true,
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::None, ButtonPressType::None)
        );
    }

    #[test]
    fn both_none_if_not_triggered() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData {
            clock: Some(&clock),
            ..Default::default()
        });

        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_bt: ButtonType, current_delay: u32| {
                if current_delay >= 250 {
                    false
                } else {
                    true
                }
            },
            is_button_triggered: |_: ButtonType| false,
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::None, ButtonPressType::None)
        );
    }

    #[test]
    fn one_none_ten_short_when_one_not_triggered() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData {
            clock: Some(&clock),
            ..Default::default()
        });

        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, current_delay: u32| {
                if current_delay >= 250 {
                    false
                } else {
                    true
                }
            },
            is_button_triggered: |bt: ButtonType| match bt {
                ButtonType::One => false,
                ButtonType::Ten => true,
            },
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::None, ButtonPressType::Short)
        );
    }

    #[test]
    fn one_short_ten_none_when_ten_not_triggered() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData {
            clock: Some(&clock),
            ..Default::default()
        });

        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, current_delay: u32| {
                if current_delay >= 250 {
                    false
                } else {
                    true
                }
            },
            is_button_triggered: |bt: ButtonType| match bt {
                ButtonType::One => true,
                ButtonType::Ten => false,
            },
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::Short, ButtonPressType::None)
        );
    }

    #[test]
    fn one_none_ten_long_when_one_not_triggered() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData {
            clock: Some(&clock),
            ..Default::default()
        });

        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| true,
            is_button_triggered: |bt: ButtonType| match bt {
                ButtonType::One => false,
                ButtonType::Ten => true,
            },
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::None, ButtonPressType::Long)
        );
    }

    #[test]
    fn one_long_ten_none_when_ten_not_triggered() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData {
            clock: Some(&clock),
            ..Default::default()
        });

        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| true,
            is_button_triggered: |bt: ButtonType| match bt {
                ButtonType::One => true,
                ButtonType::Ten => false,
            },
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::Long, ButtonPressType::None)
        );
    }

    #[test]
    fn both_short() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData {
            clock: Some(&clock),
            ..Default::default()
        });

        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_bt: ButtonType, current_delay: u32| {
                if current_delay >= 250 {
                    false
                } else {
                    true
                }
            },
            is_button_triggered: |_: ButtonType| true,
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::Short, ButtonPressType::Short)
        );
    }

    #[test]
    fn one_none_ten_short() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData {
            clock: Some(&clock),
            ..Default::default()
        });

        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType, current_delay: u32| match bt {
                ButtonType::One => false,
                ButtonType::Ten => {
                    if current_delay >= 250 {
                        false
                    } else {
                        true
                    }
                }
            },
            is_button_triggered: |_: ButtonType| true,
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::None, ButtonPressType::Short)
        );
    }

    #[test]
    fn one_short_ten_none() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData {
            clock: Some(&clock),
            ..Default::default()
        });

        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType, current_delay: u32| match bt {
                ButtonType::One => {
                    if current_delay >= 250 {
                        false
                    } else {
                        true
                    }
                }
                ButtonType::Ten => false,
            },
            is_button_triggered: |_: ButtonType| true,
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::Short, ButtonPressType::None)
        );
    }

    #[test]
    fn both_long() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData {
            clock: Some(&clock),
            ..Default::default()
        });

        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_bt: ButtonType, current_delay: u32| {
                if current_delay >= 1500 {
                    false
                } else {
                    true
                }
            },
            is_button_triggered: |_: ButtonType| true,
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::Long, ButtonPressType::Long)
        );
    }

    #[test]
    fn both_long_when_infinitely_pressed() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData::default());
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_bt: ButtonType, _current_delay: u32| true,
            is_button_triggered: |_: ButtonType| true,
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::Long, ButtonPressType::Long)
        );
    }

    #[test]
    fn one_none_ten_long() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData::default());
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType, _current_delay: u32| match bt {
                ButtonType::One => false,
                ButtonType::Ten => true,
            },
            is_button_triggered: |_: ButtonType| true,
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::None, ButtonPressType::Long)
        );
    }

    #[test]
    fn one_short_ten_long() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData {
            clock: Some(&clock),
            ..Default::default()
        });

        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType, current_delay: u32| match bt {
                ButtonType::One => {
                    if current_delay >= 250 {
                        false
                    } else {
                        true
                    }
                }
                ButtonType::Ten => true,
            },
            is_button_triggered: |_: ButtonType| true,
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::Short, ButtonPressType::Long)
        );
    }

    #[test]
    fn one_long_ten_none() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData::default());
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType, _current_delay: u32| match bt {
                ButtonType::One => true,
                ButtonType::Ten => false,
            },
            is_button_triggered: |_: ButtonType| true,
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::Long, ButtonPressType::None)
        );
    }

    #[test]
    fn one_long_ten_short() {
        let clock = Clock::default();
        let mut systick_mock = MockData::new(SystickAssociatedData {
            clock: Some(&clock),
            ..Default::default()
        });

        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType, current_delay: u32| match bt {
                ButtonType::One => true,
                ButtonType::Ten => {
                    if current_delay >= 250 {
                        false
                    } else {
                        true
                    }
                }
            },
            is_button_triggered: |_: ButtonType| true,
            clock: &clock,
        });

        assert_eq!(
            create_buttons(&mut mock_data, &mut create_systick(&mut systick_mock)).interrupt(),
            (ButtonPressType::Long, ButtonPressType::Short)
        );
    }
}
