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

pub type ButtonsPollResult = (ButtonPressType, ButtonPressType, u32);

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ButtonsPoll {
    Ready(ButtonsPollResult),
    Pending(u32),
}

#[derive(Copy, Clone, Default)]
pub struct ButtonsState {
    pub poll_result: Option<ButtonsPollResult>,
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

pub struct Buttons<'a, T: ButtonsHardware> {
    hw: T,
    state: &'a mut ButtonsState,
}

impl<'a, T: ButtonsHardware> Buttons<'a, T> {
    pub fn new(hw: T, state: &'a mut ButtonsState) -> Self {
        Buttons { hw, state }
    }

    /// Setups Buttons hardware.
    pub fn setup(&self) {
        self.hw.setup()
    }

    /// Tears down Buttons hardware.
    pub fn teardown(&self) {
        self.hw.teardown()
    }

    pub fn poll(&mut self) -> ButtonsPoll {
        let button_one_pressed = self.hw.is_button_pressed(ButtonType::One);
        let button_ten_pressed = self.hw.is_button_pressed(ButtonType::Ten);
        let get_button_state =
            |previous_state: ButtonPressType, is_pressed: bool, pending_time: u32| {
                if is_pressed && pending_time <= 500 {
                    ButtonPressType::Short
                } else if is_pressed && pending_time >= 1250 && !previous_state.is_none() {
                    ButtonPressType::Long
                } else {
                    previous_state
                }
            };

        let (button_one_prev_state, button_ten_prev_state, pending_time) = self
            .state
            .poll_result
            .unwrap_or_else(|| (ButtonPressType::None, ButtonPressType::None, 0));
        let button_one_state =
            get_button_state(button_one_prev_state, button_one_pressed, pending_time);
        let button_ten_state =
            get_button_state(button_ten_prev_state, button_ten_pressed, pending_time);

        if (!button_one_pressed && !button_ten_pressed) || pending_time >= 1250 {
            self.state.poll_result = None;
            ButtonsPoll::Ready((button_one_state, button_ten_state, pending_time))
        } else {
            self.state.poll_result = Some((button_one_state, button_ten_state, pending_time + 250));
            ButtonsPoll::Pending(250)
        }
    }

    /// Detects whether buttons are in the middle of the poll.
    pub fn is_polling(&self) -> bool {
        self.state.poll_result.is_some()
    }

    /// Detects whether any of the control buttons was triggered.
    pub fn triggered(&self) -> bool {
        self.hw.is_button_triggered(ButtonType::One) || self.hw.is_button_triggered(ButtonType::Ten)
    }

    /// Reactivates all control buttons so that they are ready to receive events again.
    pub fn reactivate(&self) {
        self.hw.reactivate_button(ButtonType::One);
        self.hw.reactivate_button(ButtonType::Ten);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::MockData;
    use core::cell::RefCell;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    enum Call {
        Setup,
        Teardown,
        Reactivate(ButtonType),
    }

    pub(crate) struct AssociatedData<F: Fn(ButtonType) -> bool, FA: Fn(ButtonType) -> bool> {
        pub is_button_pressed: F,
        pub is_button_triggered: FA,
    }

    struct ButtonsHardwareMock<'a, 'b: 'a, F: Fn(ButtonType) -> bool, FA: Fn(ButtonType) -> bool> {
        data: RefCell<&'a mut MockData<'b, Call, AssociatedData<F, FA>>>,
    }

    impl<'a, 'b: 'a, F: Fn(ButtonType) -> bool, FA: Fn(ButtonType) -> bool> ButtonsHardware
        for ButtonsHardwareMock<'a, 'b, F, FA>
    {
        fn setup(&self) {
            self.data.borrow_mut().calls.log_call(Call::Setup);
        }

        fn teardown(&self) {
            self.data.borrow_mut().calls.log_call(Call::Teardown);
        }

        fn is_button_pressed(&self, button_type: ButtonType) -> bool {
            (self.data.borrow().data.is_button_pressed)(button_type)
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

    fn create_buttons<'a, 'b: 'a, F: Fn(ButtonType) -> bool, FA: Fn(ButtonType) -> bool>(
        mock_data: &'a mut MockData<'b, Call, AssociatedData<F, FA>>,
        state: &'a mut ButtonsState,
    ) -> Buttons<'a, ButtonsHardwareMock<'a, 'b, F, FA>> {
        Buttons::new(
            ButtonsHardwareMock {
                data: RefCell::new(mock_data),
            },
            state,
        )
    }

    #[test]
    fn setup() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType| false,
            is_button_triggered: |_: ButtonType| false,
        });
        let mut state = ButtonsState::default();

        create_buttons(&mut mock_data, &mut state).setup();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Setup)])
    }

    #[test]
    fn teardown() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType| false,
            is_button_triggered: |_: ButtonType| false,
        });
        let mut state = ButtonsState::default();

        create_buttons(&mut mock_data, &mut state).teardown();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Teardown)])
    }

    #[test]
    fn reactivates_both() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType| false,
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        create_buttons(&mut mock_data, &mut state).reactivate();

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
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType| false,
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::None, ButtonPressType::None, 0))
        );
    }

    #[test]
    fn triggered_false_if_both_not_triggered() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType| true,
            is_button_triggered: |_: ButtonType| false,
        });
        let mut state = ButtonsState::default();

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).triggered(),
            false
        );
    }

    #[test]
    fn triggered_true_if_both_triggered() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType| true,
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        assert_eq!(create_buttons(&mut mock_data, &mut state).triggered(), true);
    }

    #[test]
    fn triggered_true_if_ten_is_triggered_only() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType| true,
            is_button_triggered: |bt: ButtonType| match bt {
                ButtonType::One => false,
                ButtonType::Ten => true,
            },
        });
        let mut state = ButtonsState::default();

        assert_eq!(create_buttons(&mut mock_data, &mut state).triggered(), true);
    }

    #[test]
    fn triggered_true_if_one_is_triggered_only() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType| true,
            is_button_triggered: |bt: ButtonType| match bt {
                ButtonType::One => true,
                ButtonType::Ten => false,
            },
        });
        let mut state = ButtonsState::default();

        assert_eq!(create_buttons(&mut mock_data, &mut state).triggered(), true);
    }

    #[test]
    fn both_short() {
        let pending_time = RefCell::new(0);
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_bt: ButtonType| {
                if *pending_time.borrow() >= 250 {
                    false
                } else {
                    true
                }
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Pending(250)
        );

        *pending_time.borrow_mut() = 250;

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::Short, ButtonPressType::Short, 250))
        );
    }

    #[test]
    fn both_short_even_if_one_pressed_later() {
        let pending_time = RefCell::new(0);
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType| {
                if *pending_time.borrow() >= 500 {
                    false
                } else if *pending_time.borrow() < 250 {
                    match bt {
                        ButtonType::One => false,
                        ButtonType::Ten => true,
                    }
                } else {
                    true
                }
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        for time in (0..500).step_by(250) {
            *pending_time.borrow_mut() = time;
            assert_eq!(
                create_buttons(&mut mock_data, &mut state).poll(),
                ButtonsPoll::Pending(250)
            );
        }

        *pending_time.borrow_mut() += 250;

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::Short, ButtonPressType::Short, 500))
        );
    }

    #[test]
    fn both_short_even_if_ten_pressed_later() {
        let pending_time = RefCell::new(0);
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType| {
                if *pending_time.borrow() >= 500 {
                    false
                } else if *pending_time.borrow() < 250 {
                    match bt {
                        ButtonType::One => true,
                        ButtonType::Ten => false,
                    }
                } else {
                    true
                }
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        for time in (0..500).step_by(250) {
            *pending_time.borrow_mut() = time;
            assert_eq!(
                create_buttons(&mut mock_data, &mut state).poll(),
                ButtonsPoll::Pending(250)
            );
        }

        *pending_time.borrow_mut() += 250;

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::Short, ButtonPressType::Short, 500))
        );
    }

    #[test]
    fn one_none_ten_short() {
        let pending_time = RefCell::new(0);
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType| match bt {
                ButtonType::One => false,
                ButtonType::Ten => {
                    if *pending_time.borrow() >= 250 {
                        false
                    } else {
                        true
                    }
                }
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Pending(250)
        );

        *pending_time.borrow_mut() = 250;

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::None, ButtonPressType::Short, 250))
        );
    }

    #[test]
    fn one_short_ten_none() {
        let pending_time = RefCell::new(0);
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType| match bt {
                ButtonType::One => {
                    if *pending_time.borrow() >= 250 {
                        false
                    } else {
                        true
                    }
                }
                ButtonType::Ten => false,
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Pending(250)
        );

        *pending_time.borrow_mut() = 250;

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::Short, ButtonPressType::None, 250))
        );
    }

    #[test]
    fn both_long() {
        let pending_time = RefCell::new(0);
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_bt: ButtonType| {
                if *pending_time.borrow() > 1250 {
                    false
                } else {
                    true
                }
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        for time in (0..1250).step_by(250) {
            *pending_time.borrow_mut() = time;
            assert_eq!(
                create_buttons(&mut mock_data, &mut state).poll(),
                ButtonsPoll::Pending(250)
            );
        }

        *pending_time.borrow_mut() += 250;

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::Long, ButtonPressType::Long, 1250))
        );
    }

    #[test]
    fn both_long_when_infinitely_pressed() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_bt: ButtonType| true,
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        for _ in (0..1250).step_by(250) {
            assert_eq!(
                create_buttons(&mut mock_data, &mut state).poll(),
                ButtonsPoll::Pending(250)
            );
        }

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::Long, ButtonPressType::Long, 1250))
        );
    }

    #[test]
    fn both_long_even_if_one_pressed_later() {
        let pending_time = RefCell::new(0);
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType| {
                if *pending_time.borrow() > 1250 {
                    false
                } else if *pending_time.borrow() < 500 {
                    match bt {
                        ButtonType::One => false,
                        ButtonType::Ten => true,
                    }
                } else {
                    true
                }
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        for time in (0..1250).step_by(250) {
            *pending_time.borrow_mut() = time;
            assert_eq!(
                create_buttons(&mut mock_data, &mut state).poll(),
                ButtonsPoll::Pending(250)
            );
        }

        *pending_time.borrow_mut() += 250;

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::Long, ButtonPressType::Long, 1250))
        );
    }

    #[test]
    fn both_long_even_if_ten_pressed_later() {
        let pending_time = RefCell::new(0);
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType| {
                if *pending_time.borrow() > 1250 {
                    false
                } else if *pending_time.borrow() < 500 {
                    match bt {
                        ButtonType::One => true,
                        ButtonType::Ten => false,
                    }
                } else {
                    true
                }
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        for time in (0..1250).step_by(250) {
            *pending_time.borrow_mut() = time;
            assert_eq!(
                create_buttons(&mut mock_data, &mut state).poll(),
                ButtonsPoll::Pending(250)
            );
        }

        *pending_time.borrow_mut() += 250;

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::Long, ButtonPressType::Long, 1250))
        );
    }

    #[test]
    fn one_none_ten_long() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType| match bt {
                ButtonType::One => false,
                ButtonType::Ten => true,
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        for _ in (0..1250).step_by(250) {
            assert_eq!(
                create_buttons(&mut mock_data, &mut state).poll(),
                ButtonsPoll::Pending(250)
            );
        }

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::None, ButtonPressType::Long, 1250))
        );
    }

    #[test]
    fn one_short_ten_long() {
        let pending_time = RefCell::new(0);
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType| match bt {
                ButtonType::One => {
                    if *pending_time.borrow() >= 250 {
                        false
                    } else {
                        true
                    }
                }
                ButtonType::Ten => true,
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        for time in (0..1250).step_by(250) {
            *pending_time.borrow_mut() = time;
            assert_eq!(
                create_buttons(&mut mock_data, &mut state).poll(),
                ButtonsPoll::Pending(250)
            );
        }

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::Short, ButtonPressType::Long, 1250))
        );
    }

    #[test]
    fn one_long_ten_none() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType| match bt {
                ButtonType::One => true,
                ButtonType::Ten => false,
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        for _ in (0..1250).step_by(250) {
            assert_eq!(
                create_buttons(&mut mock_data, &mut state).poll(),
                ButtonsPoll::Pending(250)
            );
        }

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::Long, ButtonPressType::None, 1250))
        );
    }

    #[test]
    fn one_long_ten_short() {
        let pending_time = RefCell::new(0);
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType| match bt {
                ButtonType::One => true,
                ButtonType::Ten => {
                    if *pending_time.borrow() >= 250 {
                        false
                    } else {
                        true
                    }
                }
            },
            is_button_triggered: |_: ButtonType| true,
        });
        let mut state = ButtonsState::default();

        for time in (0..1250).step_by(250) {
            *pending_time.borrow_mut() = time;
            assert_eq!(
                create_buttons(&mut mock_data, &mut state).poll(),
                ButtonsPoll::Pending(250)
            );
        }

        *pending_time.borrow_mut() += 250;

        assert_eq!(
            create_buttons(&mut mock_data, &mut state).poll(),
            ButtonsPoll::Ready((ButtonPressType::Long, ButtonPressType::Short, 1250))
        );
    }
}
