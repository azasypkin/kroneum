use core::ops::Range;

const PAGES: [usize; 2] = [0x0800_7800, 0x0800_7C00];

/// Describes memory slot where we can write to or read from u16 data value.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum StorageSlot {
    One = 0x1F,
    Two = 0x2F,
    Three = 0x3F,
    Four = 0x4F,
    Five = 0x5F,
    Unknown = 0xFF,
}

impl From<u8> for StorageSlot {
    fn from(slot_value: u8) -> Self {
        match slot_value {
            0x1F => StorageSlot::One,
            0x2F => StorageSlot::Two,
            0x3F => StorageSlot::Three,
            0x4F => StorageSlot::Four,
            0x5F => StorageSlot::Five,
            _ => StorageSlot::Unknown,
        }
    }
}

enum _StoragePageStatus {
    Active,
    Erased,
}

#[doc = r"Flash EEPROM emulation page"]
struct StoragePage {
    base_address: usize,
    size: usize,
}

impl StoragePage {
    pub fn _status(&self) -> _StoragePageStatus {
        match self.u16(self.base_address) {
            0xABCD => _StoragePageStatus::Active,
            _ => _StoragePageStatus::Erased,
        }
    }

    pub fn size_hint(&self) -> u16 {
        self.u16(self.base_address + 2)
    }

    pub fn set_size_hint(&self, size_hint: u16) {
        self.set_u16(self.base_address + 2, size_hint);
    }

    pub fn _set_status(&self, status: _StoragePageStatus) {
        self.set_u16(
            self.base_address,
            match status {
                _StoragePageStatus::Active => 0xABCD,
                _StoragePageStatus::Erased => 0xFFFF,
            },
        )
    }

    pub fn read(&self, virtual_address: u8) -> Option<u8> {
        assert_ne!(virtual_address, 0xff);
        for offset in self.search_range().rev().step_by(2) {
            let value = self.u16(self.base_address + offset);
            if value != 0xFFFF && (value >> 8) as u8 == virtual_address {
                return Some((value & 0xff) as u8);
            }
        }

        None
    }

    pub fn write(&self, virtual_address: u8, value: u8) -> Result<(), ()> {
        assert_ne!(virtual_address, 0xff);
        let search_range = self.search_range();
        let start_of_range = search_range.start;
        let end_of_range = search_range.end;

        let offset = self
            .search_range()
            .rev()
            .step_by(2)
            .find(|offset| self.u16(self.base_address + offset) != 0xFFFF)
            .map(|offset| offset + 2)
            .unwrap_or_else(|| {
                if self.u16(self.base_address + start_of_range) == 0xFFFF {
                    start_of_range
                } else {
                    end_of_range + 1
                }
            });

        if offset >= self.size {
            return Err(());
        }

        self.extend_search_range(offset);

        Ok(self.set_u16(
            self.base_address + offset,
            ((virtual_address as u16) << 8) | value as u16,
        ))
    }

    fn u16(&self, address: usize) -> u16 {
        unsafe { core::ptr::read(address as *mut u16) }
    }

    fn set_u16(&self, address: usize, val: u16) {
        unsafe { core::ptr::write(address as *mut u16, val) }
    }

    fn search_range(&self) -> Range<usize> {
        Range {
            start: 4,
            end: 63 + (self.size_hint().leading_zeros() * 64) as usize,
        }
    }

    fn extend_search_range(&self, upper_bound_offset: usize) {
        self.set_size_hint(self.size_hint() >> (upper_bound_offset / 64))
    }
}

pub struct Storage {
    pages: [StoragePage; 2],
}

impl Storage {
    pub fn read(&self, slot: StorageSlot) -> Option<u8> {
        self.pages[0].read(slot as u8)
    }

    pub fn write(&self, slot: StorageSlot, value: u8) -> Result<(), ()> {
        self.pages[0].write(slot as u8, value)
    }
}

impl Default for Storage {
    fn default() -> Self {
        Storage {
            pages: [
                StoragePage {
                    base_address: PAGES[0],
                    size: 1024,
                },
                StoragePage {
                    base_address: PAGES[1],
                    size: 1024,
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_initializes() {
        let sandbox: [u16; 1024] = [0xffff; 1024];
        let sandbox_address: usize = &sandbox as *const _ as usize;
        let storage = Storage {
            pages: [
                StoragePage {
                    base_address: sandbox_address,
                    size: 1024,
                },
                StoragePage {
                    base_address: sandbox_address,
                    size: 1024,
                },
            ],
        };

        assert_eq!(
            sandbox[0..6],
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );
        assert_eq!(storage.read(StorageSlot::One), None);
    }

    #[test]
    fn correctly_writes_and_reads() {
        let sandbox: [u16; 1024] = [0xffff; 1024];
        let sandbox_address: usize = &sandbox as *const _ as usize;
        let storage = Storage {
            pages: [
                StoragePage {
                    base_address: sandbox_address,
                    size: 1024,
                },
                StoragePage {
                    base_address: sandbox_address,
                    size: 1024,
                },
            ],
        };

        assert_eq!(storage.read(StorageSlot::One), None);
        assert_eq!(storage.write(StorageSlot::One, 2), Ok(()));
        assert_eq!(storage.read(StorageSlot::One), Some(2));
        assert_eq!(
            sandbox[0..6],
            [0xffff, 0xffff, 0x1f02, 0xffff, 0xffff, 0xffff]
        );

        assert_eq!(storage.write(StorageSlot::One, 3), Ok(()));
        assert_eq!(storage.read(StorageSlot::One), Some(3));
        assert_eq!(
            sandbox[0..6],
            [0xffff, 0xffff, 0x1f02, 0x1f03, 0xffff, 0xffff]
        );

        assert_eq!(storage.write(StorageSlot::Two, 4), Ok(()));
        assert_eq!(storage.read(StorageSlot::One), Some(3));
        assert_eq!(storage.read(StorageSlot::Two), Some(4));
        assert_eq!(
            sandbox[0..6],
            [0xffff, 0xffff, 0x1f02, 0x1f03, 0x2f04, 0xffff]
        );
    }
}
