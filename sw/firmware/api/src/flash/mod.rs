pub mod storage;
mod storage_page;
mod storage_page_status;
pub mod storage_slot;

use self::{storage::Storage, storage_page::StoragePage, storage_slot::StorageSlot};

const PAGE_ADDRESSES: [usize; 2] = [0x0800_7800, 0x0800_7C00];

/// Describes the Flash hardware management interface.
pub trait FlashHardware {
    /// Initializes hardware if needed.
    fn setup(&self);

    /// Releases hardware if needed.
    fn teardown(&self);

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
        Flash {
            hw,
            storage: Storage {
                pages: [
                    StoragePage {
                        address: PAGE_ADDRESSES[0],
                        size: 1024,
                    },
                    StoragePage {
                        address: PAGE_ADDRESSES[1],
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
}

#[cfg(test)]
mod tests {}
