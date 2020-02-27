use super::Register;
use bit_field::BitField;

/// Setup of Automatic Retransmission Register.
pub struct SetupRetrRegister([u8; 1]);
impl SetupRetrRegister {
    /// Get Auto Re-transmit Delay ‘0000’ – Wait 250+86uS, ‘0001’ – Wait 500+86uS etc.
    pub fn ard(&self) -> u8 {
        self.0[0].get_bits(4..=7).into()
    }

    /// Set Auto Re-transmit Delay ‘0000’ – Wait 250+86uS, ‘0001’ – Wait 500+86uS etc.
    pub fn set_ard(&mut self, value: u8) -> &mut Self {
        if value <= 0xF {
            self.0[0].set_bits(4..=7, value.into());
        }

        self
    }

    /// Get Auto Retransmit Count ‘0000’ – Re-Transmit disabled, ‘0001’ – Up to 1 Re-Transmit on
    /// fail of AA, etc. Up to 15 Re-Transmit on fail of AA.
    pub fn arc(&self) -> u8 {
        self.0[0].get_bits(0..=3).into()
    }

    /// Set Auto Retransmit Count ‘0000’ – Re-Transmit disabled, ‘0001’ – Up to 1 Re-Transmit on
    /// fail of AA, etc. Up to 15 Re-Transmit on fail of AA.
    pub fn set_arc(&mut self, value: u8) -> &mut Self {
        if value <= 0xF {
            self.0[0].set_bits(0..=3, value.into());
        }

        self
    }
}

impl Default for SetupRetrRegister {
    fn default() -> Self {
        SetupRetrRegister { 0: [0x03] }
    }
}

impl Register for SetupRetrRegister {
    type TRaw = [u8; 1];

    fn address() -> u8 {
        0x04
    }

    fn raw(&self) -> Self::TRaw {
        self.0
    }

    fn from_raw(buffer: Self::TRaw) -> Self {
        Self { 0: buffer }
    }
}
