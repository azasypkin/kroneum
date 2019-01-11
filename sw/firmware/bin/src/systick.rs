use crate::config;
use cortex_m::peripheral::{syst::SystClkSource, SYST};

pub struct SysTick {}

impl SysTick {
    pub fn delay_us(systick: &mut SYST, us: u32) {
        let rvr = us * (config::CLOCK_SPEED / 1_000_000);

        assert!(rvr < (1 << 24));

        systick.set_clock_source(SystClkSource::Core);
        systick.set_reload(rvr);
        systick.clear_current();
        systick.enable_counter();

        while !systick.has_wrapped() {}

        systick.disable_counter();
    }

    pub fn delay_ms(systick: &mut SYST, ms: u32) {
        Self::delay_us(systick, ms * 1000);
    }
}
