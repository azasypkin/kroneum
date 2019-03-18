use cortex_m::peripheral::{syst::SystClkSource, SYST};

pub type SysTick = kroneum_api::systick::SysTick<SystickHardwareImpl>;

pub struct SystickHardwareImpl {
    syst: SYST,
}

impl kroneum_api::systick::SysTickHardware for SystickHardwareImpl {
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

pub fn get(syst: SYST) -> kroneum_api::systick::SysTick<SystickHardwareImpl> {
    kroneum_api::systick::SysTick::create(SystickHardwareImpl { syst })
}
