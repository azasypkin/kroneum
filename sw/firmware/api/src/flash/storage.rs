use super::{
    storage_page::{StoragePage, StoragePageFullError},
    storage_page_status::StoragePageStatus,
    storage_slot::StorageSlot,
};

/// Max number of pages used by storage.
const PAGES_COUNT: usize = 2;

/// Describes multi-page storage that simulates EEPROM on top of flash. This is very naive and simple
/// implementation that allows device to store up to 5 different 8-bit values in so called slots.
/// That's pretty much enough for the configuration options device may need.
#[doc = r"Flash EEPROM emulation storage"]
#[derive(Debug)]
pub struct Storage {
    pub pages: [StoragePage; PAGES_COUNT],
}

impl Storage {
    /// Reads value located in the specified virtual memory slot.
    pub fn read(&self, slot: StorageSlot) -> Option<u8> {
        self.active_page().read(slot.into())
    }

    /// Writes value into specified virtual memory slot. If write fails the error includes references
    /// to the current page and the one that should be used next to allow consumer to prepare next
    /// page if needed and switch to rolling over all existing values from the current page.
    pub fn write(&self, slot: StorageSlot, value: u8) -> Result<(), StoragePageFullError> {
        let active_page = self.active_page();
        if let Err(_) = active_page.write(slot.into(), value) {
            Err(StoragePageFullError {
                active_page,
                next_page: self.next_page(),
            })
        } else {
            Ok(())
        }
    }

    /// Marks active page as full and switches to the next page rolling over latest version of all
    /// existing values to the new page.
    pub fn rollover(&self) -> Result<(), ()> {
        let active_page = self.active_page();
        let next_page = self.next_page();

        active_page.set_status(StoragePageStatus::Full);
        next_page.set_status(StoragePageStatus::Active);

        active_page.flush_to(next_page)
    }

    /// Returns currently active page. If it doesn't find an active page it marks first one as an
    /// active page.
    fn active_page(&self) -> &StoragePage {
        let active_page_index = self
            .pages
            .iter()
            .position(|page| page.status() == StoragePageStatus::Active);
        if let Some(page_index) = active_page_index {
            &self.pages[page_index]
        } else {
            self.pages[0].set_status(StoragePageStatus::Active);
            &self.pages[0]
        }
    }

    /// Returns the page reference that will be used once the current page is full.
    fn next_page(&self) -> &StoragePage {
        let active_page = self.active_page();
        let active_page_index = self
            .pages
            .iter()
            .position(|page| page == active_page)
            .unwrap_or_default();

        &self.pages[if active_page_index + 1 == self.pages.len() {
            0
        } else {
            active_page_index + 1
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Size of the page in bytes (u8).
    const PAGE_SIZE: usize = 1024;

    #[test]
    fn correctly_initializes() {
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let storage = Storage {
            pages: [
                StoragePage {
                    address: &page1 as *const _ as usize,
                    size: PAGE_SIZE,
                },
                StoragePage {
                    address: &page2 as *const _ as usize,
                    size: PAGE_SIZE,
                },
            ],
        };

        let page1_slice = &page1[..6];
        assert_eq!(
            page1_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );

        let page2_slice = &page2[..6];
        assert_eq!(
            page2_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );

        assert_eq!(storage.read(StorageSlot::One), None);
    }

    #[test]
    fn correctly_writes_and_reads() {
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let storage = Storage {
            pages: [
                StoragePage {
                    address: &page1 as *const _ as usize,
                    size: PAGE_SIZE,
                },
                StoragePage {
                    address: &page2 as *const _ as usize,
                    size: PAGE_SIZE,
                },
            ],
        };

        let page1_slice = &page1[..6];
        let page2_slice = &page2[..6];

        assert_eq!(storage.read(StorageSlot::One), None);
        assert_eq!(storage.write(StorageSlot::One, 2).is_ok(), true);
        assert_eq!(storage.read(StorageSlot::One), Some(2));
        assert_eq!(
            page1_slice,
            [0x0fff, 0xffff, 0x1f02, 0xffff, 0xffff, 0xffff]
        );
        assert_eq!(
            page2_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );

        assert_eq!(storage.write(StorageSlot::One, 3).is_ok(), true);
        assert_eq!(storage.read(StorageSlot::One), Some(3));
        assert_eq!(
            page1_slice,
            [0x0fff, 0xffff, 0x1f02, 0x1f03, 0xffff, 0xffff]
        );
        assert_eq!(
            page2_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );

        assert_eq!(storage.write(StorageSlot::Two, 4).is_ok(), true);
        assert_eq!(storage.read(StorageSlot::One), Some(3));
        assert_eq!(storage.read(StorageSlot::Two), Some(4));
        assert_eq!(
            page1_slice,
            [0x0fff, 0xffff, 0x1f02, 0x1f03, 0x2f04, 0xffff]
        );
        assert_eq!(
            page2_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );
    }

    #[test]
    fn fails_when_page_is_full() {
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let storage = Storage {
            pages: [
                StoragePage {
                    address: &page1 as *const _ as usize,
                    size: PAGE_SIZE,
                },
                StoragePage {
                    address: &page2 as *const _ as usize,
                    size: PAGE_SIZE,
                },
            ],
        };

        // Fill all memory slots.
        for _ in 0..510 {
            assert_eq!(storage.write(StorageSlot::One, 1).is_ok(), true);
        }

        // Now we can't write anymore
        assert_eq!(storage.write(StorageSlot::One, 1).is_err(), true);
        assert_eq!(&page1[510..], [0x1f01, 0x1f01])
    }

    #[test]
    fn successfully_rolls_over_to_next_page() {
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let storage = Storage {
            pages: [
                StoragePage {
                    address: &page1 as *const _ as usize,
                    size: PAGE_SIZE,
                },
                StoragePage {
                    address: &page2 as *const _ as usize,
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

        // Fill remaining slots.
        assert_eq!(storage.write(StorageSlot::Three, 4).is_ok(), true);
        assert_eq!(storage.write(StorageSlot::Five, 15).is_ok(), true);

        // Now we can't write anymore
        assert_eq!(storage.write(StorageSlot::One, 1).is_err(), true);

        let page1_slice = &page1[508..];
        assert_eq!(page1_slice, [0x2f03, 0x2f03, 0x3f04, 0x5f0f]);

        // Move to next page.
        let page2_slice = &page2[..8];
        assert_eq!(
            page2_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );

        assert_eq!(storage.rollover().is_ok(), true);

        assert_eq!(page1_slice, [0x2f03, 0x2f03, 0x3f04, 0x5f0f]);
        assert_eq!(
            page2_slice,
            [0x0fff, 0xffff, 0x5f0f, 0x3f04, 0x2f03, 0x1f01, 0xffff, 0xffff]
        );

        assert_eq!(storage.write(StorageSlot::Two, 10).is_ok(), true);
        assert_eq!(
            page2_slice,
            [0x0fff, 0xffff, 0x5f0f, 0x3f04, 0x2f03, 0x1f01, 0x2f0a, 0xffff]
        );
    }

    #[test]
    fn successfully_rolls_over_to_first_page() {
        let mut page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let storage = Storage {
            pages: [
                StoragePage {
                    address: &page1 as *const _ as usize,
                    size: PAGE_SIZE,
                },
                StoragePage {
                    address: &page2 as *const _ as usize,
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

        // Fill remaining slots.
        assert_eq!(storage.write(StorageSlot::Three, 4).is_ok(), true);
        assert_eq!(storage.write(StorageSlot::Five, 15).is_ok(), true);

        // Now we can't write anymore
        assert_eq!(storage.write(StorageSlot::One, 1).is_err(), true);

        assert_eq!(storage.rollover().is_ok(), true);

        // Fill next page.
        // Fill all memory slots, but the latest one.
        assert_eq!(storage.write(StorageSlot::One, 10).is_ok(), true);
        assert_eq!(storage.write(StorageSlot::Two, 20).is_ok(), true);

        for _ in 0..502 {
            assert_eq!(storage.write(StorageSlot::Two, 30).is_ok(), true);
        }

        // Fill remaining slots.
        assert_eq!(storage.write(StorageSlot::Three, 40).is_ok(), true);
        assert_eq!(storage.write(StorageSlot::Five, 55).is_ok(), true);

        // Now we can't write anymore
        assert_eq!(storage.write(StorageSlot::One, 1).is_err(), true);

        // Erase first page
        for i in 0..(PAGE_SIZE / 2) {
            page1[i] = 0xffff;
        }

        assert_eq!(storage.rollover().is_ok(), true);

        assert_eq!(storage.write(StorageSlot::One, 1).is_ok(), true);

        assert_eq!(storage.read(StorageSlot::One), Some(1));
        assert_eq!(storage.read(StorageSlot::Two), Some(30));
        assert_eq!(storage.read(StorageSlot::Three), Some(40));
        assert_eq!(storage.read(StorageSlot::Five), Some(55));
        assert_eq!(
            &page1[..7],
            [0x0fff, 0xffff, 0x5f37, 0x3f28, 0x2f1e, 0x1f0a, 0x1f01]
        );
    }
}
