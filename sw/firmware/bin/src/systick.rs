use cortex_m::peripheral::{syst::SystClkSource, SYST};
use kroneum_api::systick::SysTickHardware;

pub struct SystickHardwareImpl<'a> {
    syst: &'a mut SYST,
}

impl<'a> SystickHardwareImpl<'a> {
    pub fn new(syst: &'a mut SYST) -> Self {
        Self { syst }
    }
}

impl<'a> SysTickHardware for SystickHardwareImpl<'a> {
    fn configure(&mut self, reload_value: u32) {
        self.syst.set_clock_source(SystClkSource::Core);
        self.syst.set_reload(reload_value);
        self.syst.clear_current();
    }

    fn enable_counter(&mut self) {
        self.syst.enable_counter();
    }

    fn disable_counter(&mut self) {
        self.syst.disable_counter();
    }

    fn has_wrapped(&mut self) -> bool {
        self.syst.has_wrapped()
    }
}
