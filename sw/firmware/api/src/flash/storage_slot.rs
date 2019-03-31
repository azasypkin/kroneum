/// Describes memory slot where we can write to or read from u8 data value.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum StorageSlot {
    One,
    Two,
    Three,
    Four,
    Five,
    Unknown,
}

impl From<u8> for StorageSlot {
    fn from(slot_value: u8) -> Self {
        match slot_value {
            0x1f => StorageSlot::One,
            0x2f => StorageSlot::Two,
            0x3f => StorageSlot::Three,
            0x4f => StorageSlot::Four,
            0x5f => StorageSlot::Five,
            _ => StorageSlot::Unknown,
        }
    }
}

impl Into<u8> for StorageSlot {
    fn into(self) -> u8 {
        match self {
            StorageSlot::One => 0x1f,
            StorageSlot::Two => 0x2f,
            StorageSlot::Three => 0x3f,
            StorageSlot::Four => 0x4f,
            StorageSlot::Five => 0x5f,
            StorageSlot::Unknown => 0xff,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_created_from_u8() {
        assert_eq!(StorageSlot::from(0x1f), StorageSlot::One);
        assert_eq!(StorageSlot::from(0x2f), StorageSlot::Two);
        assert_eq!(StorageSlot::from(0x3f), StorageSlot::Three);
        assert_eq!(StorageSlot::from(0x4f), StorageSlot::Four);
        assert_eq!(StorageSlot::from(0x5f), StorageSlot::Five);
        assert_eq!(StorageSlot::from(0x6f), StorageSlot::Unknown);
        assert_eq!(StorageSlot::from(0xff), StorageSlot::Unknown);
    }

    #[test]
    fn correctly_converted_to_u8() {
        assert_eq!(Into::<u8>::into(StorageSlot::One), 0x1f);
        assert_eq!(Into::<u8>::into(StorageSlot::Two), 0x2f);
        assert_eq!(Into::<u8>::into(StorageSlot::Three), 0x3f);
        assert_eq!(Into::<u8>::into(StorageSlot::Four), 0x4f);
        assert_eq!(Into::<u8>::into(StorageSlot::Five), 0x5f);
        assert_eq!(Into::<u8>::into(StorageSlot::Unknown), 0xff);
    }
}
