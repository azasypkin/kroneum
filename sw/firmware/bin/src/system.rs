use crate::hal::stm32::Interrupt;
use crate::hal::stm32::Peripherals;
use cortex_m::peripheral::{NVIC, SCB};
use kroneum_api::system::SystemHardware;

pub struct SystemHardwareImpl {
    pub p: Peripherals,
    scb: SCB,
    nvic: NVIC,
}

impl SystemHardwareImpl {
    pub fn new(p: Peripherals, scb: SCB, nvic: NVIC) -> Self {
        Self { p, scb, nvic }
    }
}

impl SystemHardwareImpl {
    fn toggle_deep_sleep(&mut self, on: bool) {
        // Toggle SLEEPDEEP bit of Cortex-M0 System Control Register
        // and enter Standby mode when the CPU enters Deep Sleep.
        if on {
            self.scb.set_sleepdeep();
            self.p.PWR.cr.modify(|_, w| w.pdds().standby_mode());
        } else {
            self.scb.clear_sleepdeep();
            self.p.PWR.cr.modify(|_, w| w.pdds().stop_mode());
        }

        self.p.PWR.cr.modify(|_, w| w.cwuf().set_bit());
    }

    fn setup_buttons(&mut self) {
        // Enable EXTI0 interrupt line for PA0 and EXTI2 for PA2.
        self.p
            .SYSCFG
            .exticr1
            .modify(|_, w| w.exti0().pa0().exti2().pa2());

        // Configure PA0/PA2 to trigger an interrupt event on the EXTI0/EXTI2 line on a rising edge.
        self.p
            .EXTI
            .rtsr
            .modify(|_, w| w.tr0().set_bit().tr2().set_bit());

        // Unmask the external interrupt line EXTI0\EXTI2 by setting the bit corresponding to the
        // EXTI0\EXTI2 "bit 0/2" in the EXT_IMR register.
        self.p
            .EXTI
            .imr
            .modify(|_, w| w.mr0().set_bit().mr2().set_bit());

        // Enable clock for GPIO Port A.
        self.p.RCC.ahbenr.modify(|_, w| w.iopaen().enabled());

        // Switch PA0 (button) and PA2 (button) to alternate function mode.
        self.p
            .GPIOA
            .moder
            .modify(|_, w| w.moder0().alternate().moder2().alternate());

        // Enable pull-down for PA0 and PA2.
        self.p
            .GPIOA
            .pupdr
            .modify(|_, w| w.pupdr0().pull_down().pupdr2().pull_down());

        // Set alternative function #2 for PA0 (WKUP1) and PA2 (WKUP4).
        self.p
            .GPIOA
            .afrl
            .modify(|_, w| w.afrl0().af2().afrl2().af2());

        // Enable wakers.
        self.p
            .PWR
            .csr
            .modify(|_, w| w.ewup1().set_bit().ewup4().set_bit());
    }

    fn setup_usb(&mut self) {
        // Remap PA9-10 to PA11-12 for USB.
        self.p
            .SYSCFG
            .cfgr1
            .modify(|_, w| w.pa11_pa12_rmp().remapped());

        // Enable clock for GPIO Port A.
        self.p.RCC.ahbenr.modify(|_, w| w.iopaen().enabled());

        // Switch  PA11 and PA12 (usb) to alternate function mode.
        self.p
            .GPIOA
            .moder
            .modify(|_, w| w.moder11().alternate().moder12().alternate());

        // Set "high" output speed for PA11 and PA12.
        self.p.GPIOA.ospeedr.modify(|_, w| {
            w.ospeedr11()
                .very_high_speed()
                .ospeedr12()
                .very_high_speed()
        });
    }

    fn setup_beeper(&mut self) {
        // Enable clock for GPIO Port B.
        self.p.RCC.ahbenr.modify(|_, w| w.iopben().enabled());

        // Switch PB1 (beeper) to alternate function mode
        self.p.GPIOB.moder.modify(|_, w| w.moder1().alternate());

        // Set "high" output speed for PB1.
        self.p
            .GPIOB
            .ospeedr
            .modify(|_, w| w.ospeedr1().very_high_speed());

        // Set alternative function #2 for PB1 (TIM1_CH3N).
        self.p.GPIOB.afrl.modify(|_, w| w.afrl1().af2());
    }
}

impl SystemHardware for SystemHardwareImpl {
    fn setup(&mut self) {
        self.p.RCC.apb2enr.modify(|_, w| w.syscfgen().enabled());
        self.p.SYSCFG.cfgr1.modify(|_, w| w.mem_mode().main_flash());

        self.setup_usb();
        self.setup_buttons();
        self.setup_beeper();

        // ---------GPIO------------------

        // Switch  PA1, PA3-7 to AIN to reduce power consumption.
        self.p.GPIOA.moder.modify(|_, w| {
            w.moder1()
                .analog()
                .moder3()
                .analog()
                .moder4()
                .analog()
                .moder5()
                .analog()
                .moder6()
                .analog()
                .moder7()
                .analog()
                .moder9()
                .analog()
                .moder10()
                .analog()
                .moder13()
                .analog()
                .moder14()
                .analog()
        });

        // Switch PB8 to AIN to reduce power consumption.
        self.p.GPIOB.moder.modify(|_, w| w.moder8().analog());

        // Enable clock for GPIO Port F.
        self.p.RCC.ahbenr.modify(|_, w| w.iopfen().enabled());

        // Enable AIN for GPIO F to reduce power consumption.
        self.p
            .GPIOF
            .moder
            .modify(|_, w| w.moder0().analog().moder1().analog());

        self.p.RCC.ahbenr.modify(|_, w| w.iopfen().disabled());

        // Configure and enable interrupts.
        unsafe {
            self.nvic.set_priority(Interrupt::EXTI0_1, 1);
            self.nvic.set_priority(Interrupt::EXTI2_3, 1);
            self.nvic.set_priority(Interrupt::RTC, 1);
            self.nvic.set_priority(Interrupt::TIM2, 1);

            NVIC::unmask(Interrupt::EXTI0_1);
            NVIC::unmask(Interrupt::EXTI2_3);
            NVIC::unmask(Interrupt::RTC);
            NVIC::unmask(Interrupt::USB);
            NVIC::unmask(Interrupt::TIM2);
        }

        self.p.RCC.apb2enr.modify(|_, w| w.syscfgen().disabled());
    }

    fn enter_deep_sleep(&mut self) {
        self.toggle_deep_sleep(true);
    }

    fn exit_deep_sleep(&mut self) {
        self.toggle_deep_sleep(false);
    }

    fn reset(&mut self) {
        SCB::sys_reset();
    }
}
