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

    /// Blocks the thread for the specified number of milliseconds.
    fn delay(&mut self, delay_ms: u32);
}

pub struct Buttons<T: ButtonsHardware> {
    hw: T,
}

impl<T: ButtonsHardware> Buttons<T> {
    pub fn new(hw: T) -> Self {
        Buttons { hw }
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
    use crate::tests::MockData;
    use core::cell::RefCell;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    enum Call {
        Setup,
        Teardown,
    }

    pub(crate) struct AssociatedData<F: Fn(ButtonType, u32) -> bool> {
        pub is_button_pressed: F,
        pub current_delay: u32,
    }

    struct ButtonsHardwareMock<'a, 'b: 'a, F: Fn(ButtonType, u32) -> bool> {
        data: RefCell<&'a mut MockData<'b, Call, AssociatedData<F>>>,
    }

    impl<'a, 'b: 'a, F: Fn(ButtonType, u32) -> bool> ButtonsHardware
        for ButtonsHardwareMock<'a, 'b, F>
    {
        fn setup(&self) {
            self.data.borrow_mut().calls.log_call(Call::Setup);
        }

        fn teardown(&self) {
            self.data.borrow_mut().calls.log_call(Call::Teardown);
        }

        fn is_button_pressed(&self, button_type: ButtonType) -> bool {
            let current_delay = self.data.borrow().data.current_delay;
            (self.data.borrow().data.is_button_pressed)(button_type, current_delay)
        }

        fn delay(&mut self, delay_ms: u32) {
            self.data.borrow_mut().data.current_delay += delay_ms;
        }
    }

    fn create_buttons<'a, 'b: 'a, F: Fn(ButtonType, u32) -> bool>(
        mock_data: &'a mut MockData<'b, Call, AssociatedData<F>>,
    ) -> Buttons<ButtonsHardwareMock<'a, 'b, F>> {
        Buttons::new(ButtonsHardwareMock {
            data: RefCell::new(mock_data),
        })
    }

    #[test]
    fn setup() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| false,
            current_delay: 0,
        });

        create_buttons(&mut mock_data).setup();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Setup)])
    }

    #[test]
    fn teardown() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| false,
            current_delay: 0,
        });

        create_buttons(&mut mock_data).teardown();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Teardown)])
    }

    #[test]
    fn both_none() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_: ButtonType, _: u32| false,
            current_delay: 0,
        });

        assert_eq!(
            create_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::None, ButtonPressType::None)
        );
    }

    #[test]
    fn both_short() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_bt: ButtonType, current_delay: u32| {
                if current_delay >= 250 {
                    false
                } else {
                    true
                }
            },
            current_delay: 0,
        });

        assert_eq!(
            create_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Short, ButtonPressType::Short)
        );
    }

    #[test]
    fn one_none_ten_short() {
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
            current_delay: 0,
        });

        assert_eq!(
            create_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::None, ButtonPressType::Short)
        );
    }

    #[test]
    fn one_short_ten_none() {
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
            current_delay: 0,
        });

        assert_eq!(
            create_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Short, ButtonPressType::None)
        );
    }

    #[test]
    fn both_long() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_bt: ButtonType, current_delay: u32| {
                if current_delay >= 1500 {
                    false
                } else {
                    true
                }
            },
            current_delay: 0,
        });

        assert_eq!(
            create_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Long, ButtonPressType::Long)
        );
    }

    #[test]
    fn both_long_when_infinitely_pressed() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |_bt: ButtonType, _current_delay: u32| true,
            current_delay: 0,
        });

        assert_eq!(
            create_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Long, ButtonPressType::Long)
        );
    }

    #[test]
    fn one_none_ten_long() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType, _current_delay: u32| match bt {
                ButtonType::One => false,
                ButtonType::Ten => true,
            },
            current_delay: 0,
        });

        assert_eq!(
            create_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::None, ButtonPressType::Long)
        );
    }

    #[test]
    fn one_short_ten_long() {
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
            current_delay: 0,
        });

        assert_eq!(
            create_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Short, ButtonPressType::Long)
        );
    }

    #[test]
    fn one_long_ten_none() {
        let mut mock_data = MockData::new(AssociatedData {
            is_button_pressed: |bt: ButtonType, _current_delay: u32| match bt {
                ButtonType::One => true,
                ButtonType::Ten => false,
            },
            current_delay: 0,
        });

        assert_eq!(
            create_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Long, ButtonPressType::None)
        );
    }

    #[test]
    fn one_long_ten_short() {
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
            current_delay: 0,
        });

        assert_eq!(
            create_buttons(&mut mock_data).interrupt(),
            (ButtonPressType::Long, ButtonPressType::Short)
        );
    }
}
