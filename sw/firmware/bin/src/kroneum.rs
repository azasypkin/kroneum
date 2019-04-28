use crate::system;

use cortex_m::peripheral::SCB;
use stm32f0::stm32f0x2::Peripherals;

use crate::system::SystemHardwareImpl;
use crate::systick::SystickHardwareImpl;
use kroneum_api::{
    system::{System, SystemState},
    systick::SysTick,
};

pub struct Kroneum {
    p: Peripherals,
    scb: SCB,
    systick: SysTick<SystickHardwareImpl>,
    state: SystemState,
}

impl Kroneum {
    pub fn create(p: Peripherals, scb: SCB, systick: SysTick<SystickHardwareImpl>) -> Self {
        let mut kroneum = Kroneum {
            p,
            scb,
            state: SystemState::default(),
            systick,
        };

        kroneum.system().setup();

        kroneum
    }

    /// Creates an instance of `System` controller.
    pub fn system(&mut self) -> System<SystemHardwareImpl, SystickHardwareImpl> {
        System::new(
            system::SystemHardwareImpl::new(&self.p, &mut self.scb),
            &mut self.state,
            &mut self.systick,
        )
    }
}
