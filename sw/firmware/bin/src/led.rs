use crate::{systick::SysTick, AppPeripherals};

/// Defines the color to operate on.
pub enum LEDColor {
    Blue,
    Green,
    Red,
}

pub struct LED<'a> {
    p: &'a mut AppPeripherals,
}

impl<'a> LED<'a> {
    fn new(p: &'a mut AppPeripherals) -> Self {
        LED { p }
    }

    pub fn blink(&mut self, color: &LEDColor) {
        self.turn_off(color);

        self.turn_on(color);
        SysTick::delay_ms(&mut self.p.core.SYST, 250);
        self.turn_off(color)
    }

    pub fn turn_on(&self, color: &LEDColor) {
        let reg = &self.p.device.GPIOA.bsrr;
        match color {
            LEDColor::Green => reg.write(|w| w.bs3().set_bit()),
            LEDColor::Red => reg.write(|w| w.bs4().set_bit()),
            LEDColor::Blue => reg.write(|w| w.bs5().set_bit()),
        }
    }

    pub fn turn_off(&self, color: &LEDColor) {
        let reg = &self.p.device.GPIOA.bsrr;
        match color {
            LEDColor::Green => reg.write(|w| w.br3().set_bit()),
            LEDColor::Red => reg.write(|w| w.br4().set_bit()),
            LEDColor::Blue => reg.write(|w| w.br5().set_bit()),
        }
    }

    pub fn acquire<F, R>(p: &mut AppPeripherals, f: F) -> R
    where
        F: FnOnce(LED) -> R,
    {
        f(LED::new(p))
    }
}
