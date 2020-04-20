pub mod storage;
mod storage_page;
mod storage_page_status;
pub mod storage_slot;

use self::{storage::Storage, storage_page::StoragePage, storage_slot::StorageSlot};

/// Describes the Flash hardware management interface.
pub trait FlashHardware {
    /// Returns addresses of the flash memory pages.
    fn page_addresses(&self) -> [usize; 2];

    /// Erases page using specified address.
    fn erase_page(&self, page_address: usize);

    /// Makes peripheral to enter `write` mode.
    fn enable_write_mode(&self);

    /// Makes peripheral to exit `write` mode.
    fn disable_write_mode(&self);
}

pub struct Flash<'a, T: FlashHardware> {
    hw: &'a T,
    storage: Storage,
}

impl<'a, T: FlashHardware> Flash<'a, T> {
    pub fn new(hw: &'a T) -> Self {
        let page_addresses = hw.page_addresses();
        Flash {
            hw,
            storage: Storage {
                pages: [
                    StoragePage {
                        address: page_addresses[0],
                        size: 1024,
                    },
                    StoragePage {
                        address: page_addresses[1],
                        size: 1024,
                    },
                ],
            },
        }
    }

    /// Reads a value from a specific memory slot.
    pub fn read(&self, slot: StorageSlot) -> Option<u8> {
        self.storage.read(slot)
    }

    /// Writes a value to a specific memory slot.
    pub fn write(&self, slot: StorageSlot, value: u8) -> Result<(), ()> {
        self.hw.enable_write_mode();

        let result = self.storage.write(slot, value).or_else(|err| {
            self.hw.disable_write_mode();

            self.hw.erase_page(err.next_page.address);
            self.storage.rollover()?;

            self.hw.enable_write_mode();

            self.storage.write(slot, value).map_err(|_| {})
        });

        self.hw.disable_write_mode();

        result
    }

    /// Erases all storage pages.
    pub fn erase_all(&self) {
        for page in self.storage.pages.iter() {
            self.hw.erase_page(page.address);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::MockData;
    use core::cell::RefCell;

    // Size of the page in bytes (u8).
    const PAGE_SIZE: usize = 1024;

    #[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
    enum Call {
        EnableWriteMode,
        DisableWriteMode,
        ErasePage(usize),
    }

    struct FlashHardwareMock<'a> {
        data: RefCell<MockData<'a, Call>>,
        page_addresses: [usize; 2],
    }

    impl<'a> FlashHardware for FlashHardwareMock<'a> {
        fn page_addresses(&self) -> [usize; 2] {
            self.page_addresses
        }

        fn erase_page(&self, page_address: usize) {
            self.data
                .borrow_mut()
                .calls
                .log_call(Call::ErasePage(page_address));
        }

        fn enable_write_mode(&self) {
            self.data.borrow_mut().calls.log_call(Call::EnableWriteMode);
        }

        fn disable_write_mode(&self) {
            self.data
                .borrow_mut()
                .calls
                .log_call(Call::DisableWriteMode);
        }
    }

    #[test]
    fn read() {
        let mut page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let flash_hw_mock = FlashHardwareMock {
            data: RefCell::new(MockData::<Call, ()>::without_data()),
            page_addresses: [&page1 as *const _ as usize, &page2 as *const _ as usize],
        };
        let flash = Flash::new(&flash_hw_mock);

        assert_eq!(flash.read(StorageSlot::Configuration), None);
        assert_eq!(flash.read(StorageSlot::Custom(2)), None);

        page1[2] = 0xaf0f;

        assert_eq!(flash.read(StorageSlot::Configuration), Some(0x0f));
        assert_eq!(flash.read(StorageSlot::Custom(2)), None);

        page1[3] = 0x2f01;

        assert_eq!(flash.read(StorageSlot::Configuration), Some(0x0f));
        assert_eq!(flash.read(StorageSlot::Custom(2)), Some(0x01));
    }

    #[test]
    fn write_when_page_has_enough_space() {
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let flash_hw_mock = FlashHardwareMock {
            data: RefCell::new(MockData::<Call, ()>::without_data()),
            page_addresses: [&page1 as *const _ as usize, &page2 as *const _ as usize],
        };

        let flash = Flash::new(&flash_hw_mock);
        assert_eq!(flash.write(StorageSlot::Configuration, 10).is_ok(), true);

        assert_eq!(page1[..4], [0x0fff, 0xffff, 0xaf0a, 0xffff]);
        assert_eq!(
            flash.hw.data.borrow().calls.logs(),
            [Some(Call::EnableWriteMode), Some(Call::DisableWriteMode)]
        );
    }

    #[test]
    fn write_when_page_is_full() {
        let mut page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let flash_hw_mock = FlashHardwareMock {
            data: RefCell::new(MockData::<Call, ()>::without_data()),
            page_addresses: [&page1 as *const _ as usize, &page2 as *const _ as usize],
        };

        let flash = Flash::new(&flash_hw_mock);

        // Imitate fully populated page.
        page1[0] = 0x0fff;
        page1[1] = 0x0001;
        for i in 2..(PAGE_SIZE / 2) {
            page1[i] = 0xaf12;
        }

        assert_eq!(flash.write(StorageSlot::Custom(2), 0x17).is_ok(), true);

        assert_eq!(page2[..5], [0x0fff, 0xffff, 0xaf12, 0x2f17, 0xffff]);
        assert_eq!(
            flash.hw.data.borrow().calls.logs(),
            [
                Some(Call::EnableWriteMode),
                Some(Call::DisableWriteMode),
                Some(Call::ErasePage(&page2 as *const _ as usize)),
                Some(Call::EnableWriteMode),
                Some(Call::DisableWriteMode)
            ]
        );
    }

    #[test]
    fn erase_all() {
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let flash_hw_mock = FlashHardwareMock {
            data: RefCell::new(MockData::<Call, ()>::without_data()),
            page_addresses: [&page1 as *const _ as usize, &page2 as *const _ as usize],
        };

        let flash = Flash::new(&flash_hw_mock);

        flash.erase_all();

        assert_eq!(
            flash.hw.data.borrow().calls.logs(),
            [
                Some(Call::ErasePage(&page1 as *const _ as usize)),
                Some(Call::ErasePage(&page2 as *const _ as usize))
            ]
        );
    }
}
