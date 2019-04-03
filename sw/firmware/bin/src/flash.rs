use kroneum_api::flash::{Flash, FlashHardware};
use stm32f0::stm32f0x2::Peripherals;

pub struct FlashHardwareImpl<'a> {
    p: &'a Peripherals,
}

impl<'a> FlashHardwareImpl<'a> {
    /// Disables or enables Flash write protection.
    fn toggle_write_protection(&self, enable_write_protection: bool) {
        let is_protected = self.p.FLASH.cr.read().lock().bit_is_set();
        if enable_write_protection && !is_protected {
            self.p.FLASH.cr.write(|w| w.lock().set_bit());
        } else if is_protected {
            self.p.FLASH.keyr.write(|w| unsafe { w.bits(0x4567_0123) });
            self.p.FLASH.keyr.write(|w| unsafe { w.bits(0xCDEF_89AB) });
        }
    }

    fn busy_wait_until_ready(&self) {
        // Wait until Flash is not busy.
        while self.p.FLASH.sr.read().bsy().bit_is_set() {}
    }
}

impl<'a> FlashHardware for FlashHardwareImpl<'a> {
    fn setup(&self) {}

    fn teardown(&self) {}

    fn erase_page(&self, page_address: usize) {
        self.busy_wait_until_ready();
        self.toggle_write_protection(false);

        self.p.FLASH.cr.modify(|_, w| w.per().set_bit());
        self.p
            .FLASH
            .ar
            .write(|w| unsafe { w.bits(page_address as u32) });
        self.p.FLASH.cr.modify(|_, w| w.strt().set_bit());

        self.busy_wait_until_ready();

        self.p.FLASH.cr.modify(|_, w| w.per().clear_bit());

        self.toggle_write_protection(true);
    }

    fn enable_write_mode(&self) {
        self.busy_wait_until_ready();

        self.toggle_write_protection(false);

        self.p.FLASH.cr.modify(|_, w| w.pg().set_bit());
    }

    fn disable_write_mode(&self) {
        self.busy_wait_until_ready();

        self.p.FLASH.cr.modify(|_, w| w.pg().clear_bit());

        self.toggle_write_protection(true);
    }
}

pub fn create(p: &Peripherals) -> Flash<FlashHardwareImpl> {
    Flash::new(FlashHardwareImpl { p })
}
