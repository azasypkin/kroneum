use super::{
    storage_page::{StoragePage, StoragePageFullError},
    storage_page_status::StoragePageStatus,
    storage_slot::StorageSlot,
};

/// Describes multi-page storage that simulates EEPROM on top of flash.
#[doc = r"Flash EEPROM emulation storage"]
#[derive(Debug)]
pub struct Storage {
    pub pages: [StoragePage; 2],
}

impl Storage {
    pub fn read(&self, slot: StorageSlot) -> Option<u8> {
        let (_, active_page) = self.active_page();
        active_page.read(slot.into())
    }

    pub fn write(&self, slot: StorageSlot, value: u8) -> Result<(), StoragePageFullError> {
        let (current_page_index, active_page) = self.active_page();
        if active_page.write(slot.into(), value).is_err() {
            Err(StoragePageFullError {
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

    pub fn rollover(&self) -> Result<(), StoragePageFullError> {
        let (active_page_index, active_page) = self.active_page();
        let next_page = &self.pages[if active_page_index + 1 == self.pages.len() {
            0
        } else {
            active_page_index + 1
        }];

        active_page.set_status(StoragePageStatus::Full);
        next_page.set_status(StoragePageStatus::Active);

        for slot in [
            StorageSlot::One,
            StorageSlot::Two,
            StorageSlot::Three,
            StorageSlot::Four,
            StorageSlot::Five,
        ]
        .iter()
        {
            if let Some(value) = active_page.read((*slot).into()) {
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
