use super::Register;
use bit_field::BitField;

const MAX_CHANNEL: u8 = 125;

/// RF Channel.
pub struct RFCHRegister([u8; 1]);
impl RFCHRegister {
    /// Get the frequency channel.
    pub fn rf_ch(&self) -> u8 {
        self.0[0].get_bits(0..=7)
    }

    /// Set the frequency channel.
    pub fn set_rf_ch(&mut self, value: u8) -> &mut Self {
        self.0[0].set_bits(
            0..=7,
            if value > MAX_CHANNEL {
                MAX_CHANNEL
            } else {
                value
            },
        );
        self
    }
}

impl Default for RFCHRegister {
    fn default() -> Self {
        RFCHRegister { 0: [0x02] }
    }
}

impl Register for RFCHRegister {
    type TRaw = [u8; 1];

    fn address() -> u8 {
        0x05
    }

    fn raw(&self) -> Self::TRaw {
        self.0
    }

    fn from_raw(buffer: Self::TRaw) -> Self {
        Self { 0: buffer }
    }
}
