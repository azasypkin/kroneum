use cortex_m::peripheral::{syst::SystClkSource, SYST};
use kroneum_api::systick::SysTickHardware;

pub struct SystickHardwareImpl {
    syst: SYST,
}

impl SystickHardwareImpl {
    pub fn new(syst: SYST) -> Self {
        Self { syst }
    }
}

impl SysTickHardware for SystickHardwareImpl {
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

    fn enable_interrupt(&mut self) {
        self.syst.enable_interrupt();
    }

    fn disable_interrupt(&mut self) {
        self.syst.disable_interrupt();
    }
}
