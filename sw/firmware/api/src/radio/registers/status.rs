use super::{super::pipes::Pipe, Register};
use bit_field::BitField;

/// FIFO Status Register.
#[derive(Debug, Copy, Clone)]
pub struct StatusRegister([u8; 1]);
impl StatusRegister {
    /// Check if TX FIFO is full.
    pub fn tx_full(&self) -> bool {
        self.0[0].get_bit(0)
    }

    /// Data pipe number for the payload available for reading from RX_FIFO.
    pub fn rx_p_no(&self) -> Option<Pipe> {
        match self.0[0].get_bits(1..=3) {
            0x0 => Some(Pipe::Pipe0),
            0x1 => Some(Pipe::Pipe1),
            0x2 => Some(Pipe::Pipe2),
            0x3 => Some(Pipe::Pipe3),
            0x4 => Some(Pipe::Pipe4),
            0x5 => Some(Pipe::Pipe5),
            _ => None,
        }
    }

    /// Check if maximum number of TX retransmits interrupt is triggered.
    pub fn max_rt(&self) -> bool {
        self.0[0].get_bit(4)
    }

    /// Clear maximum number of TX retransmits interrupt flag.
    pub fn clear_max_rt(&mut self) -> &mut Self {
        self.0[0].set_bit(4, true);
        self
    }

    /// Check if data sent TX FIFO interrupt is triggered.
    pub fn tx_ds(&self) -> bool {
        self.0[0].get_bit(5)
    }

    /// Clear maximum sent TX FIFO interrupt flag.
    pub fn clear_tx_ds(&mut self) -> &mut Self {
        self.0[0].set_bit(5, true);
        self
    }

    /// Check if data ready RX FIFO interrupt is triggered.
    pub fn rx_dr(&self) -> bool {
        self.0[0].get_bit(6)
    }

    /// Clear maximum ready RX FIFO interrupt flag.
    pub fn clear_rx_dr(&mut self) -> &mut Self {
        self.0[0].set_bit(6, true);
        self
    }
}

impl Register for StatusRegister {
    type TRaw = [u8; 1];

    fn address() -> u8 {
        0x07
    }

    fn raw(&self) -> Self::TRaw {
        self.0
    }

    fn from_raw(buffer: Self::TRaw) -> Self {
        Self { 0: buffer }
    }
}
