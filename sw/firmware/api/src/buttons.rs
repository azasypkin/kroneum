/// Defines known button types.
pub enum ButtonType {
    One,
    Ten,
}

/// Defines type of the press (short, long, very long).
#[derive(Copy, Clone, PartialEq)]
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
