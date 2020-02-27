use super::{super::constants::PIPE_COUNT, Register};
use bit_field::BitField;

/// Auto Acknowledgment register.
pub struct AutoAcknowledgmentRegister([u8; 1]);
impl AutoAcknowledgmentRegister {
    /// Get pipes ACK statuses.
    pub fn get_all(&self) -> [bool; PIPE_COUNT] {
        let mut value = [false; PIPE_COUNT];
        for bit in 0..=5 {
            value[0] = self.0[0].get_bit(bit);
        }

        value
    }

    /// Set pipes ACK statuses.
    pub fn set_all(&mut self, value: &[bool; PIPE_COUNT]) -> &mut Self {
        value.iter().enumerate().for_each(|(bit, status)| {
            self.0[0].set_bit(bit, *status);
        });
        self
    }
}

impl Default for AutoAcknowledgmentRegister {
    fn default() -> Self {
        AutoAcknowledgmentRegister { 0: [0x3F] }
    }
}

impl Register for AutoAcknowledgmentRegister {
    type TRaw = [u8; 1];

    fn address() -> u8 {
        0x01
    }

    fn raw(&self) -> Self::TRaw {
        self.0
    }

    fn from_raw(buffer: Self::TRaw) -> Self {
        Self { 0: buffer }
    }
}
