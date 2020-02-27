use super::Register;
use bit_field::BitField;

#[derive(Copy, Clone)]
pub enum ConfigRegisterPrim {
    Transmitter,
    Receiver,
}

impl Into<bool> for ConfigRegisterPrim {
    fn into(self) -> bool {
        match self {
            ConfigRegisterPrim::Transmitter => false,
            ConfigRegisterPrim::Receiver => true,
        }
    }
}

impl From<bool> for ConfigRegisterPrim {
    fn from(value: bool) -> Self {
        match value {
            false => ConfigRegisterPrim::Transmitter,
            true => ConfigRegisterPrim::Receiver,
        }
    }
}

pub enum ConfigRegisterCRCO {
    TwoBytes,
    OneByte,
}

impl Into<bool> for ConfigRegisterCRCO {
    fn into(self) -> bool {
        match self {
            ConfigRegisterCRCO::TwoBytes => true,
            ConfigRegisterCRCO::OneByte => false,
        }
    }
}

impl From<bool> for ConfigRegisterCRCO {
    fn from(value: bool) -> Self {
        match value {
            true => ConfigRegisterCRCO::TwoBytes,
            false => ConfigRegisterCRCO::OneByte,
        }
    }
}

/// Configuration Register.
#[derive(Debug, Copy, Clone)]
pub struct ConfigRegister([u8; 1]);
impl ConfigRegister {
    /// Set RX/TX control (1 for Receiver and 0 for Transmitter mode).
    pub fn set_prim_rx(&mut self, value: ConfigRegisterPrim) -> &mut Self {
        self.0[0].set_bit(0, value.into());
        self
    }

    /// Set Power Up status.
    pub fn set_pwr_up(&mut self, value: bool) -> &mut Self {
        self.0[0].set_bit(1, value);
        self
    }

    /// Set CRC encoding scheme.
    pub fn set_crco(&mut self, value: ConfigRegisterCRCO) -> &mut Self {
        self.0[0].set_bit(2, value.into());
        self
    }

    /// Set CRC Enabled status.
    pub fn set_en_crc(&mut self, value: bool) -> &mut Self {
        self.0[0].set_bit(3, value);
        self
    }
}

impl Default for ConfigRegister {
    fn default() -> Self {
        ConfigRegister { 0: [0x08] }
    }
}

impl Register for ConfigRegister {
    type TRaw = [u8; 1];

    fn address() -> u8 {
        0x00
    }

    fn raw(&self) -> Self::TRaw {
        self.0
    }

    fn from_raw(buffer: Self::TRaw) -> Self {
        Self { 0: buffer }
    }
}
