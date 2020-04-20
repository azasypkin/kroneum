use array::Array;
use core::convert::TryFrom;

/// Represents common info about System.
#[derive(Debug, PartialEq)]
pub struct SystemInfo {
    /// A 12-byte unique device ID.
    pub id: [u8; 12],

    /// The Flash memory size of the device in Kilobytes.
    pub flash_size_kb: u16,
}

impl Into<Array<u8>> for SystemInfo {
    fn into(self) -> Array<u8> {
        let mut array = Array::from(&self.id);
        array.push((self.flash_size_kb & 0x00ff) as u8);
        array.push(((self.flash_size_kb & 0xff00) >> 8) as u8);
        array
    }
}

impl TryFrom<Array<u8>> for SystemInfo {
    type Error = ();

    fn try_from(value: Array<u8>) -> Result<Self, Self::Error> {
        if value.len() != core::mem::size_of::<SystemInfo>() {
            return Err(());
        }

        let mut id = [0u8; 12];
        value.as_ref()[..12]
            .iter()
            .enumerate()
            .for_each(|(index, value)| {
                id[index] = *value;
            });

        Ok(SystemInfo {
            id,
            flash_size_kb: value[12] as u16 | (value[13] as u16) << 8,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn properly_serialized() {
        assert_eq!(
            Into::<Array<u8>>::into(SystemInfo {
                id: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                flash_size_kb: 0xabcd
            })
            .as_ref(),
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0xcd, 0xab]
        );
    }

    #[test]
    fn properly_deserialized() {
        assert_eq!(
            SystemInfo::try_from(Array::from(&[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0xcd, 0xab
            ])),
            Ok(SystemInfo {
                id: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
                flash_size_kb: 0xabcd
            })
        );
    }

    #[test]
    fn invalid_serialized_data() {
        assert_eq!(
            SystemInfo::try_from(Array::from(&[
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0xcd, 0xab
            ])),
            Err(())
        );

        assert_eq!(
            SystemInfo::try_from(Array::from(&[
                2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0xcd, 0xab
            ])),
            Err(())
        );
    }
}
