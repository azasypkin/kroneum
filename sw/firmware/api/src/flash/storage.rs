use core::ops::Range;

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

const KNOWN_SLOTS: [StorageSlot; 5] = [
    StorageSlot::One,
    StorageSlot::Two,
    StorageSlot::Three,
    StorageSlot::Four,
    StorageSlot::Five,
];

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

#[derive(PartialOrd, PartialEq)]
enum StoragePageStatus {
    Active,
    Full,
    Erased,
}

#[derive(Debug)]
pub struct PageFullError<'a> {
    pub current_page: &'a StoragePage,
    pub next_page: &'a StoragePage,
}

#[doc = r"Flash EEPROM emulation page"]
#[derive(Debug)]
pub struct StoragePage {
    pub address: usize,
    pub size: usize,
}

impl StoragePage {
    fn status(&self) -> StoragePageStatus {
        match self.u16(self.address) {
            0x0fff => StoragePageStatus::Active,
            0x00ff => StoragePageStatus::Full,
            _ => StoragePageStatus::Erased,
        }
    }

    fn set_status(&self, status: StoragePageStatus) {
        self.set_u16(
            self.address,
            match status {
                StoragePageStatus::Active => 0x0fff,
                StoragePageStatus::Full => 0x00ff,
                StoragePageStatus::Erased => 0xffff,
            },
        )
    }

    fn size_hint(&self) -> u16 {
        self.u16(self.address + 2)
    }

    fn set_size_hint(&self, size_hint: u16) {
        self.set_u16(self.address + 2, size_hint);
    }

    fn read(&self, virtual_address: u8) -> Option<u8> {
        assert_ne!(virtual_address, 0xff);
        for offset in self.search_range().rev().step_by(2) {
            let value = self.u16(self.address + offset);
            if value != 0xffff && (value >> 8) as u8 == virtual_address {
                return Some((value & 0xff) as u8);
            }
        }

        None
    }

    fn write(&self, virtual_address: u8, value: u8) -> Result<(), ()> {
        assert_ne!(virtual_address, 0xff);
        let search_range = self.search_range();
        let start_of_range = search_range.start;
        let end_of_range = search_range.end;

        let offset = self
            .search_range()
            .rev()
            .step_by(2)
            .find(|offset| self.u16(self.address + offset) != 0xffff)
            .map(|offset| offset + 2)
            .unwrap_or_else(|| {
                if self.u16(self.address + start_of_range) == 0xffff {
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
            self.address + offset,
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
        let current_hint = self.size_hint();
        let current_bound = current_hint.leading_zeros();

        let required_bound = (upper_bound_offset / 64) as u32;
        if required_bound > current_bound {
            self.set_size_hint(current_hint >> (required_bound - current_bound));
        }
    }
}

pub struct Storage {
    pub pages: [StoragePage; 2],
}

impl Storage {
    pub fn read(&self, slot: StorageSlot) -> Option<u8> {
        let (_, active_page) = self.active_page();
        active_page.read(slot as u8)
    }

    pub fn write(&self, slot: StorageSlot, value: u8) -> Result<(), PageFullError> {
        let (current_page_index, active_page) = self.active_page();
        if active_page.write(slot as u8, value).is_err() {
            Err(PageFullError {
                current_page: active_page,
                next_page: &self.pages[if current_page_index + 1 == self.pages.len() {
                    0
                } else {
                    current_page_index + 1
                }],
            })
        } else {
            Ok(())
        }
    }

    pub fn rollover(&self) -> Result<(), PageFullError> {
        let (active_page_index, active_page) = self.active_page();
        let next_page = &self.pages[if active_page_index + 1 == self.pages.len() {
            0
        } else {
            active_page_index + 1
        }];

        active_page.set_status(StoragePageStatus::Full);
        next_page.set_status(StoragePageStatus::Active);

        for slot in KNOWN_SLOTS.iter() {
            if let Some(value) = active_page.read(*slot as u8) {
                self.write(*slot, value)?
            }
        }

        Ok(())
    }

    fn active_page(&self) -> (usize, &StoragePage) {
        let active_page_index = self
            .pages
            .iter()
            .position(|page| page.status() == StoragePageStatus::Active);
        if let Some(page_index) = active_page_index {
            (page_index, &self.pages[page_index])
        } else {
            self.pages[0].set_status(StoragePageStatus::Active);
            (0, &self.pages[0])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Size of the page in bytes (u8).
    const PAGE_SIZE: usize = 1024;

    #[test]
    fn correctly_initializes() {
        let page_1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page_2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let storage = Storage {
            pages: [
                StoragePage {
                    address: &page_1 as *const _ as usize,
                    size: PAGE_SIZE,
                },
                StoragePage {
                    address: &page_2 as *const _ as usize,
                    size: PAGE_SIZE,
                },
            ],
        };

        let page_1_slice = &page_1[..6];
        assert_eq!(
            page_1_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );

        let page_2_slice = &page_2[..6];
        assert_eq!(
            page_2_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );

        assert_eq!(storage.read(StorageSlot::One), None);
    }

    #[test]
    fn correctly_writes_and_reads() {
        let page_1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page_2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let storage = Storage {
            pages: [
                StoragePage {
                    address: &page_1 as *const _ as usize,
                    size: PAGE_SIZE,
                },
                StoragePage {
                    address: &page_2 as *const _ as usize,
                    size: PAGE_SIZE,
                },
            ],
        };

        let page_1_slice = &page_1[..6];
        let page_2_slice = &page_2[..6];

        assert_eq!(storage.read(StorageSlot::One), None);
        assert_eq!(storage.write(StorageSlot::One, 2).is_ok(), true);
        assert_eq!(storage.read(StorageSlot::One), Some(2));
        assert_eq!(
            page_1_slice,
            [0x0fff, 0xffff, 0x1f02, 0xffff, 0xffff, 0xffff]
        );
        assert_eq!(
            page_2_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );

        assert_eq!(storage.write(StorageSlot::One, 3).is_ok(), true);
        assert_eq!(storage.read(StorageSlot::One), Some(3));
        assert_eq!(
            page_1_slice,
            [0x0fff, 0xffff, 0x1f02, 0x1f03, 0xffff, 0xffff]
        );
        assert_eq!(
            page_2_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );

        assert_eq!(storage.write(StorageSlot::Two, 4).is_ok(), true);
        assert_eq!(storage.read(StorageSlot::One), Some(3));
        assert_eq!(storage.read(StorageSlot::Two), Some(4));
        assert_eq!(
            page_1_slice,
            [0x0fff, 0xffff, 0x1f02, 0x1f03, 0x2f04, 0xffff]
        );
        assert_eq!(
            page_2_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );
    }

    #[test]
    fn fails_when_page_is_full() {
        let page_1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page_2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let storage = Storage {
            pages: [
                StoragePage {
                    address: &page_1 as *const _ as usize,
                    size: PAGE_SIZE,
                },
                StoragePage {
                    address: &page_2 as *const _ as usize,
                    size: PAGE_SIZE,
                },
            ],
        };

        // Fill all memory slots, but the latest one.
        assert_eq!(storage.write(StorageSlot::One, 1).is_ok(), true);
        assert_eq!(storage.write(StorageSlot::Two, 2).is_ok(), true);
        for _ in 0..506 {
            assert_eq!(storage.write(StorageSlot::Two, 3).is_ok(), true);
        }
        assert_eq!(storage.write(StorageSlot::Three, 4).is_ok(), true);

        assert_eq!(storage.read(StorageSlot::One), Some(1));
        assert_eq!(storage.read(StorageSlot::Two), Some(3));
        assert_eq!(storage.read(StorageSlot::Three), Some(4));

        let page_1_slice = &page_1[508..];

        assert_eq!(page_1_slice, [0x2f03, 0x2f03, 0x3f04, 0xffff]);

        // Fill last slot
        assert_eq!(storage.write(StorageSlot::Five, 15).is_ok(), true);
        assert_eq!(page_1_slice, [0x2f03, 0x2f03, 0x3f04, 0x5f0f]);

        // Now we can't write anymore
        let write_result = storage.write(StorageSlot::One, 1);
        assert_eq!(write_result.is_err(), true);

        // Move to next page.
        let page_2_slice = &page_2[..8];
        assert_eq!(
            page_2_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );

        assert_eq!(storage.rollover().is_ok(), true);

        assert_eq!(page_1_slice, [0x2f03, 0x2f03, 0x3f04, 0x5f0f]);
        assert_eq!(
            page_2_slice,
            [0x0fff, 0xffff, 0x1f01, 0x2f03, 0x3f04, 0x5f0f, 0xffff, 0xffff]
        );

        assert_eq!(storage.write(StorageSlot::Two, 10).is_ok(), true);
        assert_eq!(
            page_2_slice,
            [0x0fff, 0xffff, 0x1f01, 0x2f03, 0x3f04, 0x5f0f, 0x2f0a, 0xffff]
        );
    }
}
