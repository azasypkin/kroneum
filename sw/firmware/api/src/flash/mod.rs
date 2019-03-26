pub mod storage;

use self::storage::{Storage, StorageSlot};

/// Describes the Flash hardware management interface.
pub trait FlashHardware {
    /// Initializes hardware if needed.
    fn setup(&self);

    /// Releases hardware if needed.
    fn teardown(&self);

    /// Makes peripheral to enter `write` mode.
    fn before_write(&self);

    /// Makes peripheral to exit `write` mode.
    fn after_write(&self);
}

pub struct Flash<T: FlashHardware> {
    hw: T,
    storage: Storage,
}

impl<T: FlashHardware> Flash<T> {
    pub fn new(hw: T) -> Self {
        Flash {
            hw,
            storage: Storage::default(),
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
        self.hw.before_write();
        let result = self.storage.write(slot, value);
        self.hw.after_write();

        result
    }
}

#[cfg(test)]
mod tests {}
