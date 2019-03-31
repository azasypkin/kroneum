/// Defines status of the storage page. Status is a part of page header and stored as two very first
/// bytes of the page.
#[derive(Debug, PartialOrd, PartialEq)]
pub(crate) enum StoragePageStatus {
    /// Page is active and can accept new values.
    Active,
    /// Page is full and should be erased before it can be used.
    Full,
    /// Page is erased and ready to accept new values.
    Erased,
}

impl From<u16> for StoragePageStatus {
    fn from(value: u16) -> Self {
        match value {
            0x0fff => StoragePageStatus::Active,
            0x00ff => StoragePageStatus::Full,
            _ => StoragePageStatus::Erased,
        }
    }
}

impl Into<u16> for StoragePageStatus {
    fn into(self) -> u16 {
        match self {
            StoragePageStatus::Active => 0x0fff,
            StoragePageStatus::Full => 0x00ff,
            StoragePageStatus::Erased => 0xffff,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_created_from_u16() {
        assert_eq!(StoragePageStatus::from(0x0fff), StoragePageStatus::Active);
        assert_eq!(StoragePageStatus::from(0x00ff), StoragePageStatus::Full);
        assert_eq!(StoragePageStatus::from(0xffff), StoragePageStatus::Erased);
    }

    #[test]
    fn correctly_converted_to_u16() {
        assert_eq!(Into::<u16>::into(StoragePageStatus::Active), 0x0fff);
        assert_eq!(Into::<u16>::into(StoragePageStatus::Full), 0x00ff);
        assert_eq!(Into::<u16>::into(StoragePageStatus::Erased), 0xffff);
    }
}
