use super::{super::MAX_REGISTER_VALUE_SIZE, Register};

pub struct TxAddrRegister([u8; MAX_REGISTER_VALUE_SIZE]);
impl Default for TxAddrRegister {
    fn default() -> Self {
        Self {
            0: [0xE7, 0xE7, 0xE7, 0xE7, 0xE7],
        }
    }
}

impl Register for TxAddrRegister {
    type TRaw = [u8; MAX_REGISTER_VALUE_SIZE];

    fn address() -> u8 {
        0x10
    }

    fn raw(&self) -> Self::TRaw {
        self.0
    }

    /// Set address field width.
    fn from_raw(buffer: Self::TRaw) -> Self {
        Self { 0: buffer }
    }
}
