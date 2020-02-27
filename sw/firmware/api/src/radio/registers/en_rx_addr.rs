use super::{super::constants::PIPE_COUNT, Register};
use bit_field::BitField;

/// Register with RX pipes statuses.
pub struct RxPipesStatusesRegister([u8; 1]);
impl RxPipesStatusesRegister {
    /// Get pipes enabled/disabled statuses.
    pub fn get_all(&self) -> [bool; PIPE_COUNT] {
        let mut value = [false; PIPE_COUNT];
        for bit in 0..=5 {
            value[0] = self.0[0].get_bit(bit);
        }

        value
    }

    /// Set pipes enabled/disabled statuses.
    pub fn set_all(&mut self, value: &[bool; PIPE_COUNT]) -> &mut Self {
        value.iter().enumerate().for_each(|(bit, status)| {
            self.0[0].set_bit(bit, *status);
        });
        self
    }
}

impl Default for RxPipesStatusesRegister {
    fn default() -> Self {
        RxPipesStatusesRegister { 0: [0x03] }
    }
}

impl Register for RxPipesStatusesRegister {
    type TRaw = [u8; 1];

    fn address() -> u8 {
        0x02
    }

    fn raw(&self) -> Self::TRaw {
        self.0
    }

    fn from_raw(buffer: Self::TRaw) -> Self {
        Self { 0: buffer }
    }
}
