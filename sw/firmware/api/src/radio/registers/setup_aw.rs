use super::Register;
use bit_field::BitField;

#[repr(u8)]
pub enum SetupAWRegisterAW {
    Illegal = 0x0,
    ThreeBytes = 0x1,
    FourBytes = 0x2,
    FiveBytes = 0x3,
}

impl SetupAWRegisterAW {
    pub fn is_illegal(&self) -> bool {
        if let SetupAWRegisterAW::Illegal = self {
            true
        } else {
            false
        }
    }
}

impl From<u8> for SetupAWRegisterAW {
    fn from(value: u8) -> Self {
        match value {
            0x1 => SetupAWRegisterAW::ThreeBytes,
            0x2 => SetupAWRegisterAW::FourBytes,
            0x3 => SetupAWRegisterAW::FiveBytes,
            _ => SetupAWRegisterAW::Illegal,
        }
    }
}

/// Setup of Address Widths Register (common for all data pipes).
pub struct SetupAWRegister([u8; 1]);
impl SetupAWRegister {
    /// Get address field width.
    pub fn aw(&self) -> SetupAWRegisterAW {
        self.0[0].get_bits(0..=1).into()
    }

    /// Set address field width.
    pub fn set_aw(&mut self, value: SetupAWRegisterAW) -> &mut Self {
        self.0[0].set_bits(0..=1, value as u8);
        self
    }
}

impl Register for SetupAWRegister {
    type TRaw = [u8; 1];

    fn address() -> u8 {
        0x03
    }

    fn raw(&self) -> Self::TRaw {
        self.0
    }

    fn from_raw(buffer: Self::TRaw) -> Self {
        Self { 0: buffer }
    }
}
