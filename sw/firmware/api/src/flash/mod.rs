pub mod storage;
mod storage_page;
mod storage_page_status;
pub mod storage_slot;

use self::{storage::Storage, storage_page::StoragePage, storage_slot::StorageSlot};

/// Describes the Flash hardware management interface.
pub trait FlashHardware {
    /// Initializes hardware if needed.
    fn setup(&self);

    /// Releases hardware if needed.
    fn teardown(&self);

    /// Returns addresses of the flash memory pages.
    fn page_addresses(&self) -> [usize; 2];

    /// Erases page using specified address.
    fn erase_page(&self, page_address: usize);

    /// Makes peripheral to enter `write` mode.
    fn enable_write_mode(&self);

    /// Makes peripheral to exit `write` mode.
    fn disable_write_mode(&self);
}

pub struct Flash<T: FlashHardware> {
    hw: T,
    storage: Storage,
}

impl<T: FlashHardware> Flash<T> {
    pub fn new(hw: T) -> Self {
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

    /// Setups Flash hardware.
    pub fn setup(&self) {
        self.hw.setup()
    }

    /// Tears down Flash hardware.
    pub fn teardown(&self) {
        self.hw.teardown()
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
        Setup,
        Teardown,
        EnableWriteMode,
        DisableWriteMode,
        ErasePage(usize),
    }

    struct FlashHardwareMock<'a, 'b: 'a> {
        data: RefCell<&'a mut MockData<'b, Call>>,
        page_addresses: [usize; 2],
    }

    impl<'a, 'b: 'a> FlashHardware for FlashHardwareMock<'a, 'b> {
        fn setup(&self) {
            self.data.borrow_mut().calls.log_call(Call::Setup);
        }

        fn teardown(&self) {
            self.data.borrow_mut().calls.log_call(Call::Teardown);
        }

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

    fn create_flash<'a, 'b: 'a>(
        mock_data: &'a mut MockData<'b, Call>,
        page_addresses: [usize; 2],
    ) -> Flash<FlashHardwareMock<'a, 'b>> {
        Flash::new(FlashHardwareMock {
            data: RefCell::new(mock_data),
            page_addresses,
        })
    }

    #[test]
    fn setup() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        create_flash(
            &mut mock_data,
            [&page1 as *const _ as usize, &page2 as *const _ as usize],
        )
        .setup();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Setup)])
    }

    #[test]
    fn teardown() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        create_flash(
            &mut mock_data,
            [&page1 as *const _ as usize, &page2 as *const _ as usize],
        )
        .teardown();

        assert_eq!(mock_data.calls.logs(), [Some(Call::Teardown)])
    }

    #[test]
    fn read() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let mut page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        let flash = create_flash(
            &mut mock_data,
            [&page1 as *const _ as usize, &page2 as *const _ as usize],
        );

        assert_eq!(flash.read(StorageSlot::One), None);
        assert_eq!(flash.read(StorageSlot::Two), None);

        page1[2] = 0x1f0f;

        assert_eq!(flash.read(StorageSlot::One), Some(0x0f));
        assert_eq!(flash.read(StorageSlot::Two), None);

        page1[3] = 0x2f01;

        assert_eq!(flash.read(StorageSlot::One), Some(0x0f));
        assert_eq!(flash.read(StorageSlot::Two), Some(0x01));
    }

    #[test]
    fn write_when_page_has_enough_space() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        {
            let flash = create_flash(
                &mut mock_data,
                [&page1 as *const _ as usize, &page2 as *const _ as usize],
            );

            assert_eq!(flash.write(StorageSlot::One, 10).is_ok(), true);
        }

        assert_eq!(page1[..4], [0x0fff, 0xffff, 0x1f0a, 0xffff]);
        assert_eq!(
            mock_data.calls.logs(),
            [Some(Call::EnableWriteMode), Some(Call::DisableWriteMode)]
        )
    }

    #[test]
    fn write_when_page_is_full() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let mut page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        {
            let flash = create_flash(
                &mut mock_data,
                [&page1 as *const _ as usize, &page2 as *const _ as usize],
            );

            // Imitate fully populated page.
            page1[0] = 0x0fff;
            page1[1] = 0x0001;
            for i in 2..(PAGE_SIZE / 2) {
                page1[i] = 0x1f12;
            }

            assert_eq!(flash.write(StorageSlot::Two, 0x17).is_ok(), true);
        }

        assert_eq!(page2[..5], [0x0fff, 0xffff, 0x1f12, 0x2f17, 0xffff]);
        assert_eq!(
            mock_data.calls.logs(),
            [
                Some(Call::EnableWriteMode),
                Some(Call::DisableWriteMode),
                Some(Call::ErasePage(&page2 as *const _ as usize)),
                Some(Call::EnableWriteMode),
                Some(Call::DisableWriteMode)
            ]
        )
    }

    #[test]
    fn erase_all() {
        let mut mock_data = MockData::<Call, ()>::without_data();
        let page1: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page2: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];

        create_flash(
            &mut mock_data,
            [&page1 as *const _ as usize, &page2 as *const _ as usize],
        )
        .erase_all();

        assert_eq!(
            mock_data.calls.logs(),
            [
                Some(Call::ErasePage(&page1 as *const _ as usize)),
                Some(Call::ErasePage(&page2 as *const _ as usize))
            ]
        )
    }
}
