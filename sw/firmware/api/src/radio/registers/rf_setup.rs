use super::Register;
use bit_field::BitField;

#[repr(u8)]
pub enum RFSetupRegisterRFPower {
    /// 0 dBm.
    Highest = 0x3,

    /// -6 dBm.
    High = 0x2,

    /// -12 dBm.
    Low = 0x1,

    /// -18 dBm.
    Lowest = 0x0,
}

impl From<u8> for RFSetupRegisterRFPower {
    fn from(value: u8) -> Self {
        match value {
            0x0 => RFSetupRegisterRFPower::Lowest,
            0x1 => RFSetupRegisterRFPower::Low,
            0x2 => RFSetupRegisterRFPower::High,
            _ => RFSetupRegisterRFPower::Highest,
        }
    }
}

pub enum RFSetupRegisterRFDR {
    DataRate250Kbps,
    DataRate2Mbps,
    DataRate1Mbps,
}

impl From<u8> for RFSetupRegisterRFDR {
    fn from(value: u8) -> Self {
        match value {
            0x0 => RFSetupRegisterRFDR::DataRate1Mbps,
            0x1 => RFSetupRegisterRFDR::DataRate2Mbps,
            _ => RFSetupRegisterRFDR::DataRate250Kbps,
        }
    }
}

/// RF Setup Register.
pub struct RFSetupRegister([u8; 1]);
impl RFSetupRegister {
    /// Get RF output power in TX mode.
    pub fn rf_pwr(&self) -> RFSetupRegisterRFPower {
        self.0[0].get_bits(1..=2).into()
    }

    /// Set RF output power in TX mode.
    pub fn set_rf_pwr(&mut self, value: RFSetupRegisterRFPower) -> &mut Self {
        self.0[0].set_bits(1..=2, value as u8);
        self
    }

    /// Get RF data rate.
    pub fn rf_dr(&self) -> RFSetupRegisterRFDR {
        match (self.0[0].get_bit(5), self.0[0].get_bit(3)) {
            (false, false) => RFSetupRegisterRFDR::DataRate1Mbps,
            (false, true) => RFSetupRegisterRFDR::DataRate2Mbps,
            _ => RFSetupRegisterRFDR::DataRate250Kbps,
        }
    }

    /// Set RF data rate.
    pub fn set_rf_dr(&mut self, value: RFSetupRegisterRFDR) -> &mut Self {
        let (low_bit, high_bit) = match value {
            RFSetupRegisterRFDR::DataRate1Mbps => (false, false),
            RFSetupRegisterRFDR::DataRate2Mbps => (false, true),
            RFSetupRegisterRFDR::DataRate250Kbps => (true, false),
        };

        self.0[0].set_bit(5, low_bit);
        self.0[0].set_bit(3, high_bit);

        self
    }
}

impl Default for RFSetupRegister {
    fn default() -> Self {
        RFSetupRegister { 0: [0x0E] }
    }
}

impl Register for RFSetupRegister {
    type TRaw = [u8; 1];

    fn address() -> u8 {
        0x06
    }

    fn raw(&self) -> Self::TRaw {
        self.0
    }

    fn from_raw(buffer: Self::TRaw) -> Self {
        Self { 0: buffer }
    }
}
