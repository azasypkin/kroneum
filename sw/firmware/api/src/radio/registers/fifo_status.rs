use super::Register;
use bit_field::BitField;

/// FIFO Status Register.
#[derive(Debug, Copy, Clone)]
pub struct FIFOStatusRegister([u8; 1]);
impl FIFOStatusRegister {
    /// Check if TX FIFO is full.
    pub fn tx_full(&self) -> bool {
        self.0[0].get_bit(5)
    }

    /// Check if TX FIFO is empty.
    pub fn tx_empty(&self) -> bool {
        self.0[0].get_bit(4)
    }

    /// Check if TX FIFO is full.
    pub fn rx_full(&self) -> bool {
        self.0[0].get_bit(1)
    }

    /// Check if TX FIFO is empty.
    pub fn rx_empty(&self) -> bool {
        self.0[0].get_bit(0)
    }
}

impl Register for FIFOStatusRegister {
    type TRaw = [u8; 1];

    fn address() -> u8 {
        0x17
    }

    fn raw(&self) -> Self::TRaw {
        self.0
    }

    fn from_raw(buffer: Self::TRaw) -> Self {
        Self { 0: buffer }
    }
}
