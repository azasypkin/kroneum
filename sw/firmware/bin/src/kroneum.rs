use crate::system;

use cortex_m::peripheral::{Peripherals as CorePeripherals, SCB, SYST};
use stm32f0::stm32f0x2::{Interrupt, Peripherals as DevicePeripherals};

use crate::system::SystemHardwareImpl;
use crate::systick::SystickHardwareImpl;
use kroneum_api::{
    system::{System, SystemState},
    systick::SysTick,
};

pub type KroneumSystem<'a> = System<'a, SystemHardwareImpl<'a>, SystickHardwareImpl<'a>>;

pub struct Kroneum {
    device: DevicePeripherals,
    core_scb: SCB,
    core_syst: SYST,
    state: SystemState,
}

impl Kroneum {
    pub fn create(device: DevicePeripherals, mut core: CorePeripherals) -> Self {
        let mut kroneum = Kroneum {
            device,
            core_scb: core.SCB,
            core_syst: core.SYST,
            state: SystemState::default(),
        };

        kroneum.system().setup();

        // Configure and enable interrupts.
        unsafe {
            core.NVIC.set_priority(Interrupt::EXTI0_1, 1);
            core.NVIC.set_priority(Interrupt::EXTI2_3, 1);
            core.NVIC.set_priority(Interrupt::RTC, 1);
        }

        core.NVIC.enable(Interrupt::EXTI0_1);
        core.NVIC.enable(Interrupt::EXTI2_3);
        core.NVIC.enable(Interrupt::RTC);
        core.NVIC.enable(Interrupt::USB);

        kroneum
    }

    /// Creates an instance of `System` controller.
    pub fn system(&mut self) -> KroneumSystem {
        System::new(
            system::SystemHardwareImpl::new(&self.device, &mut self.core_scb),
            &mut self.state,
            SysTick::new(SystickHardwareImpl::new(&mut self.core_syst)),
        )
    }
}
