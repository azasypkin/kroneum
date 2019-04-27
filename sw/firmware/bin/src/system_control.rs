use cortex_m::peripheral::SCB;
use kroneum_api::system_control::SystemControlHardware;
use stm32f0::stm32f0x2::pwr::CR;

pub struct SystemControlHardwareImpl<'a> {
    cr: &'a CR,
    scb: &'a mut SCB,
}

impl<'a> SystemControlHardwareImpl<'a> {
    pub fn new(cr: &'a CR, scb: &'a mut SCB) -> Self {
        Self { cr, scb }
    }
}

impl<'a> SystemControlHardwareImpl<'a> {
    fn toggle_standby_mode(&mut self, on: bool) {
        // Toggle STANDBY mode.
        self.cr.modify(|_, w| w.pdds().bit(on));

        self.cr.modify(|_, w| w.cwuf().set_bit());

        // Toggle SLEEPDEEP bit of Cortex-M0 System Control Register.
        if on {
            self.scb.set_sleepdeep();
        } else {
            self.scb.clear_sleepdeep();
        }
    }
}

impl<'a> SystemControlHardware for SystemControlHardwareImpl<'a> {
    fn enter_standby_mode(&mut self) {
        self.toggle_standby_mode(true);
    }

    fn exit_standby_mode(&mut self) {
        self.toggle_standby_mode(false);
    }

    fn reset(&mut self) {
        self.scb.system_reset();
    }
}
