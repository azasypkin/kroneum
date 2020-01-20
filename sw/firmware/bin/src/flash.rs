use crate::hal::stm32::Peripherals;
use crate::system::SystemHardwareImpl;
use kroneum_api::flash::FlashHardware;

/// Sector 7, page 30 and 31 of STM32F04x flash memory.
const PAGE_ADDRESSES: [usize; 2] = [0x0800_7800, 0x0800_7C00];

/// Disables or enables Flash write protection.
fn toggle_write_protection(p: &Peripherals, enable_write_protection: bool) {
    let is_protected = p.FLASH.cr.read().lock().bit_is_set();
    if enable_write_protection && !is_protected {
        p.FLASH.cr.write(|w| w.lock().locked());
    } else if is_protected {
        p.FLASH.keyr.write(|w| w.fkeyr().bits(0x4567_0123));
        p.FLASH.keyr.write(|w| w.fkeyr().bits(0xCDEF_89AB));
    }
}

fn busy_wait_until_ready(p: &Peripherals) {
    // Wait until Flash is not busy.
    while p.FLASH.sr.read().bsy().is_active() {}
}

impl FlashHardware for SystemHardwareImpl {
    fn page_addresses(&self) -> [usize; 2] {
        PAGE_ADDRESSES
    }

    fn erase_page(&self, page_address: usize) {
        busy_wait_until_ready(&self.p);
        toggle_write_protection(&self.p, false);

        self.p.FLASH.cr.modify(|_, w| w.per().page_erase());
        self.p.FLASH.ar.write(|w| w.far().bits(page_address as u32));
        self.p.FLASH.cr.modify(|_, w| w.strt().start());

        busy_wait_until_ready(&self.p);

        self.p.FLASH.cr.modify(|_, w| w.per().clear_bit());

        toggle_write_protection(&self.p, true);
    }

    fn enable_write_mode(&self) {
        busy_wait_until_ready(&self.p);

        toggle_write_protection(&self.p, false);

        self.p.FLASH.cr.modify(|_, w| w.pg().program());
    }

    fn disable_write_mode(&self) {
        busy_wait_until_ready(&self.p);

        self.p.FLASH.cr.modify(|_, w| w.pg().clear_bit());

        toggle_write_protection(&self.p, true);
    }
}
