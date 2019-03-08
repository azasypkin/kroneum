/// Defines known button types.
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
    pub fn is_none(&self) -> bool {
        match *self {
            ButtonPressType::None => true,
            _ => false,
        }
    }
}

/// Describes the Buttons hardware management interface.
pub trait ButtonsHardware {
    /// Checks whether Button with specified type is pressed.
    fn is_button_pressed(&self, button_type: ButtonType) -> bool;

    /// Blocks the thread for the specified number of milliseconds.
    fn delay(&mut self, delay_ms: u32);
}

pub struct Buttons<T: ButtonsHardware> {
    hw: T,
}

impl<T: ButtonsHardware> Buttons<T> {
    pub fn create(hw: T) -> Self {
        Buttons { hw }
    }

    pub fn interrupt(&mut self) -> (ButtonPressType, ButtonPressType) {
        let mut button_one_state = if self.hw.is_button_pressed(ButtonType::One) {
            ButtonPressType::Short
        } else {
            ButtonPressType::None
        };

        let mut button_ten_state = if self.hw.is_button_pressed(ButtonType::Ten) {
            ButtonPressType::Short
        } else {
            ButtonPressType::None
        };

        if button_one_state.is_none() && button_ten_state.is_none() {
            return (button_one_state, button_ten_state);
        }

        for i in 1u8..8u8 {
            self.hw.delay(250);

            if !self.hw.is_button_pressed(ButtonType::One)
                && !self.hw.is_button_pressed(ButtonType::Ten)
            {
                break;
            }

            let (new_state, works_for_none) = match i {
                0...4 => (ButtonPressType::Short, true),
                5...8 => (ButtonPressType::Long, false),
                _ => break,
            };

            if self.hw.is_button_pressed(ButtonType::One)
                && (!button_one_state.is_none() || works_for_none)
            {
                button_one_state = new_state;
            }

            if self.hw.is_button_pressed(ButtonType::Ten)
                && (!button_ten_state.is_none() || works_for_none)
            {
                button_ten_state = new_state;
            }
        }

        (button_one_state, button_ten_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::RefCell;

    struct MockData<F: Fn(ButtonType, u32) -> bool> {
        pub is_button_pressed: F,
        pub current_delay: u32,
    }

    struct ButtonsHardwareMock<'a, F: Fn(ButtonType, u32) -> bool> {
        data: RefCell<&'a mut MockData<F>>,
    }

    impl<'a, F: Fn(ButtonType, u32) -> bool> ButtonsHardware for ButtonsHardwareMock<'a, F> {
        fn is_button_pressed(&self, button_type: ButtonType) -> bool {
            let current_delay = self.data.borrow().current_delay;
            (self.data.borrow().is_button_pressed)(button_type, current_delay)
        }

        fn delay(&mut self, delay_ms: u32) {
            self.data.borrow_mut().current_delay += delay_ms;
        }
    }

    fn get_buttons<F: Fn(ButtonType, u32) -> bool>(
        mock_data: &mut MockData<F>,
    ) -> Buttons<ButtonsHardwareMock<F>> {
        Buttons {
            hw: ButtonsHardwareMock {
                data: RefCell::new(mock_data),
            },
        }
    }

    #[test]
    fn both_none() {
        let mut mock_data = MockData {
            is_button_pressed: |_bt: ButtonType, _current_delay: u32| false,
            current_delay: 0,
        };

        assert_eq!(
            get_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::None, ButtonPressType::None)
        );
    }

    #[test]
    fn both_short() {
        let mut mock_data = MockData {
            is_button_pressed: |_bt: ButtonType, current_delay: u32| {
                if current_delay >= 250 {
                    false
                } else {
                    true
                }
            },
            current_delay: 0,
        };

        assert_eq!(
            get_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Short, ButtonPressType::Short)
        );
    }

    #[test]
    fn one_none_ten_short() {
        let mut mock_data = MockData {
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
            current_delay: 0,
        };

        assert_eq!(
            get_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::None, ButtonPressType::Short)
        );
    }

    #[test]
    fn one_short_ten_none() {
        let mut mock_data = MockData {
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
            current_delay: 0,
        };

        assert_eq!(
            get_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Short, ButtonPressType::None)
        );
    }

    #[test]
    fn both_long() {
        let mut mock_data = MockData {
            is_button_pressed: |_bt: ButtonType, current_delay: u32| {
                if current_delay >= 1500 {
                    false
                } else {
                    true
                }
            },
            current_delay: 0,
        };

        assert_eq!(
            get_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Long, ButtonPressType::Long)
        );
    }

    #[test]
    fn both_long_when_infinitely_pressed() {
        let mut mock_data = MockData {
            is_button_pressed: |_bt: ButtonType, _current_delay: u32| true,
            current_delay: 0,
        };

        assert_eq!(
            get_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Long, ButtonPressType::Long)
        );
    }

    #[test]
    fn one_none_ten_long() {
        let mut mock_data = MockData {
            is_button_pressed: |bt: ButtonType, _current_delay: u32| match bt {
                ButtonType::One => false,
                ButtonType::Ten => true,
            },
            current_delay: 0,
        };

        assert_eq!(
            get_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::None, ButtonPressType::Long)
        );
    }

    #[test]
    fn one_short_ten_long() {
        let mut mock_data = MockData {
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
            current_delay: 0,
        };

        assert_eq!(
            get_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Short, ButtonPressType::Long)
        );
    }

    #[test]
    fn one_long_ten_none() {
        let mut mock_data = MockData {
            is_button_pressed: |bt: ButtonType, _current_delay: u32| match bt {
                ButtonType::One => true,
                ButtonType::Ten => false,
            },
            current_delay: 0,
        };

        assert_eq!(
            get_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Long, ButtonPressType::None)
        );
    }

    #[test]
    fn one_long_ten_short() {
        let mut mock_data = MockData {
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
            current_delay: 0,
        };

        assert_eq!(
            get_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Long, ButtonPressType::Short)
        );
    }
}
