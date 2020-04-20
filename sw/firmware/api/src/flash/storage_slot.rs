use core::convert::TryFrom;

/// Describes memory slot where we can write to or read from u8 data value.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum StorageSlot {
    Configuration,
    /// Nested value is the index of custom slot: 1..=4.
    Custom(u8),
}

impl TryFrom<u8> for StorageSlot {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xaf => Ok(StorageSlot::Configuration),
            slot if matches!(slot, 0x1f | 0x2f | 0x3f | 0x4f) => Ok(StorageSlot::Custom(slot >> 4)),
            _ => Err(()),
        }
    }
}

impl Into<u8> for StorageSlot {
    fn into(self) -> u8 {
        match self {
            StorageSlot::Configuration => 0xaf,
            StorageSlot::Custom(slot) => slot << 4 | 0xf,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_created_from_u8() {
        assert_eq!(StorageSlot::try_from(0xaf), Ok(StorageSlot::Configuration));
        assert_eq!(StorageSlot::try_from(0x1f), Ok(StorageSlot::Custom(1)));
        assert_eq!(StorageSlot::try_from(0x2f), Ok(StorageSlot::Custom(2)));
        assert_eq!(StorageSlot::try_from(0x3f), Ok(StorageSlot::Custom(3)));
        assert_eq!(StorageSlot::try_from(0x4f), Ok(StorageSlot::Custom(4)));
    }

    #[test]
    fn correctly_converted_to_u8() {
        assert_eq!(Into::<u8>::into(StorageSlot::Configuration), 0xaf);
        assert_eq!(Into::<u8>::into(StorageSlot::Custom(1)), 0x1f);
        assert_eq!(Into::<u8>::into(StorageSlot::Custom(2)), 0x2f);
        assert_eq!(Into::<u8>::into(StorageSlot::Custom(3)), 0x3f);
        assert_eq!(Into::<u8>::into(StorageSlot::Custom(4)), 0x4f);
    }

    #[test]
    fn invalid_slot() {
        for slot_id in &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x6f, 0xff] {
            assert_eq!(StorageSlot::try_from(*slot_id), Err(()));
        }
    }
}
