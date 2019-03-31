use super::storage_page_status::StoragePageStatus;
use core::{mem, ops::Range};

#[derive(Debug)]
pub struct StoragePageFullError<'a> {
    pub current_page: &'a StoragePage,
    pub next_page: &'a StoragePage,
}

/// Describes storage page. The page has the following shape:
/// 0xffff - page header (status)
/// 0xffff - page header (size hint)
/// 0x1f01 - page value (slot 0x1f = slot#1 and value 0x01)
/// 0x2f0a - page value (slot 0x2f = slot#2 and value 0x0a)
/// ......
/// Due to the way flash memory works we can override value in a particular slot (1s can only change
/// to 0s), so write new value with the same virtual address. So that when we read value from storage
/// we are looking for the __latest__ value with the specified virtual address.
#[doc = r"Flash EEPROM emulation page"]
#[derive(Debug)]
pub struct StoragePage {
    pub address: usize,
    pub size: usize,
}

impl StoragePage {
    /// Retrieves page status (first two bytes of the page).
    pub(super) fn status(&self) -> StoragePageStatus {
        self.u16(self.address).into()
    }

    /// Sets page status (first two bytes of the page).
    pub(super) fn set_status(&self, status: StoragePageStatus) {
        self.set_u16(self.address, status.into())
    }

    /// Retrieves page size hint. The hint is 3rd and 4th bytes of the page and suggests the last page
    /// sector that is not fully empty. Since hint is 16 bit value it allows us to split page into
    /// 16 equal sectors, when bit is cleared that means sector is used. E.g. when page size is 1024
    /// bytes, hint can describe 16 sectors where each sector is 64 bytes wide. When page is fully
    /// erased hint value is 0xffff or 0b1111_1111_1111_1111 meaning that only first sector probably
    /// contains values, so range is 0 - 63, 0x0111_1111_1111_1111 means that first sector is full and
    /// second sector is used, so range is 0 - 127 and so on. When we iterate through page to find a
    /// value we can rely on hint and scan only non-pristine sectors.
    fn size_hint(&self) -> u16 {
        self.u16(self.address + mem::size_of::<u16>())
    }

    /// Updates page size hint.
    fn set_size_hint(&self, size_hint: u16) {
        self.set_u16(self.address + mem::size_of::<u16>(), size_hint);
    }

    /// Read the value using specified virtual address. The value is stored in u16 memory cell (0x1f01),
    /// where two most significant bits are virtual address (0x1f) and least significant bits are
    /// the value itself (0x01). So we have u8 virtual address and u8 value. If value isn't found
    /// `None` is returned.
    pub(super) fn read(&self, virtual_address: u8) -> Option<u8> {
        assert_ne!(virtual_address, 0xff);

        for offset in self.search_range().rev().step_by(mem::size_of::<u16>()) {
            let value = self.u16(self.address + offset);
            // Skip empty values (0xffff) and iterate back to the start of th range.
            if value != 0xffff && (value >> 8) as u8 == virtual_address {
                return Some((value & 0xff) as u8);
            }
        }

        None
    }

    /// Writes value with the specified virtual address. The value is stored in u16 memory cell (0x1f01),
    /// where two most significant bits are virtual address (0x1f) and least significant bits are
    /// the value itself (0x01). So we have u8 virtual address and u8 value.
    /// If page is full error is returned.
    pub(super) fn write(&self, virtual_address: u8, value: u8) -> Result<(), ()> {
        assert_ne!(virtual_address, 0xff);

        // Find the first empty (0xffff) cell in the memory to write new value to.
        let search_range = self.search_range();
        let offset = self
            .search_range()
            .rev()
            .step_by(mem::size_of::<u16>())
            .find(|offset| self.u16(self.address + offset) != 0xffff)
            .map(|offset| offset + mem::size_of::<u16>())
            .unwrap_or_else(|| {
                // There are two cases when we don't find the place to write:
                // 1. the page is fully erased
                // 2. the last sector is full and we should extend search range.
                if self.u16(self.address + search_range.start) == 0xffff {
                    search_range.start
                } else {
                    search_range.end + 1
                }
            });

        if offset >= self.size {
            return Err(());
        }

        // Try to extend search range (only if needed).
        self.extend_search_range(offset);

        Ok(self.set_u16(
            self.address + offset,
            ((virtual_address as u16) << 8) | value as u16,
        ))
    }

    /// Reads u16 from the specified address.
    fn u16(&self, address: usize) -> u16 {
        unsafe { core::ptr::read(address as *mut u16) }
    }

    /// Writes u16 to the specified address.
    fn set_u16(&self, address: usize, val: u16) {
        unsafe { core::ptr::write(address as *mut u16, val) }
    }

    /// Defines a range of page where values are located based on size hint.
    fn search_range(&self) -> Range<usize> {
        let sector_size = (self.size / 16) as u32;
        Range {
            // Skip 4 header's bytes.
            start: 2 * mem::size_of::<u16>(),
            end: ((sector_size - 1) + (self.size_hint().leading_zeros() * sector_size)) as usize,
        }
    }

    /// Extends search range if needed based on the sector where latest value is located.
    fn extend_search_range(&self, upper_bound_offset: usize) {
        let current_hint = self.size_hint();
        let current_bound = current_hint.leading_zeros();

        let sector_size = (self.size / 16) as u32;
        let required_bound = upper_bound_offset as u32 / sector_size;
        if required_bound > current_bound {
            self.set_size_hint(current_hint >> (required_bound - current_bound));
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
        let memory_sandbox: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page = StoragePage {
            address: &memory_sandbox as *const _ as usize,
            size: PAGE_SIZE,
        };

        let memory_slice = &memory_sandbox[..6];
        assert_eq!(
            memory_slice,
            [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff]
        );
        assert_eq!(page.read(0x1f), None);
    }

    #[test]
    fn correctly_writes_and_reads() {
        let memory_sandbox: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page = StoragePage {
            address: &memory_sandbox as *const _ as usize,
            size: PAGE_SIZE,
        };

        let memory_slice = &memory_sandbox[..6];

        assert_eq!(page.read(0x1f), None);
        assert_eq!(page.write(0x1f, 2).is_ok(), true);
        assert_eq!(page.read(0x1f), Some(2));
        assert_eq!(
            memory_slice,
            [0xffff, 0xffff, 0x1f02, 0xffff, 0xffff, 0xffff]
        );

        assert_eq!(page.write(0x1f, 3).is_ok(), true);
        assert_eq!(page.read(0x1f), Some(3));
        assert_eq!(
            memory_slice,
            [0xffff, 0xffff, 0x1f02, 0x1f03, 0xffff, 0xffff]
        );

        assert_eq!(page.write(0x2f, 4).is_ok(), true);
        assert_eq!(page.read(0x1f), Some(3));
        assert_eq!(page.read(0x2f), Some(4));
        assert_eq!(
            memory_slice,
            [0xffff, 0xffff, 0x1f02, 0x1f03, 0x2f04, 0xffff]
        );
    }

    #[test]
    fn fails_when_page_is_full() {
        let memory_sandbox: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page = StoragePage {
            address: &memory_sandbox as *const _ as usize,
            size: PAGE_SIZE,
        };

        // Fill all memory slots, but the latest one.
        assert_eq!(page.write(0x1f, 1).is_ok(), true);
        assert_eq!(page.write(0x2f, 2).is_ok(), true);
        for _ in 0..506 {
            assert_eq!(page.write(0x2f, 3).is_ok(), true);
        }
        assert_eq!(page.write(0x3f, 4).is_ok(), true);

        assert_eq!(page.read(0x1f), Some(1));
        assert_eq!(page.read(0x2f), Some(3));
        assert_eq!(page.read(0x3f), Some(4));

        let memory_slice = &memory_sandbox[508..];
        assert_eq!(memory_slice, [0x2f03, 0x2f03, 0x3f04, 0xffff]);

        // Fill last slot
        assert_eq!(page.write(0x5f, 15).is_ok(), true);
        assert_eq!(memory_slice, [0x2f03, 0x2f03, 0x3f04, 0x5f0f]);

        // Now we can't write anymore
        let write_result = page.write(0x1f, 1);
        assert_eq!(write_result.is_err(), true);
    }

    #[test]
    fn correctly_set_status() {
        let mut memory_sandbox: [u16; PAGE_SIZE / 2] = [0xffff; PAGE_SIZE / 2];
        let page = StoragePage {
            address: &memory_sandbox as *const _ as usize,
            size: PAGE_SIZE,
        };

        assert_eq!(page.status(), StoragePageStatus::Erased);

        page.set_status(StoragePageStatus::Active);
        assert_eq!(page.status(), StoragePageStatus::Active);
        assert_eq!(memory_sandbox[..2], [0x0fff, 0xffff]);

        page.set_status(StoragePageStatus::Full);
        assert_eq!(page.status(), StoragePageStatus::Full);
        assert_eq!(memory_sandbox[..2], [0x00ff, 0xffff]);

        memory_sandbox[0] = 0xffff;
        assert_eq!(page.status(), StoragePageStatus::Erased);
    }
}
