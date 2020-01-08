use crate::{
    beeper::BeeperHardwareImpl, buttons::ButtonsHardwareImpl, flash::FlashHardwareImpl,
    rtc::RTCHardwareImpl, timer::TimerHardwareImpl, usb::USBHardwareImpl,
};
use cortex_m::peripheral::SCB;
use kroneum_api::system::SystemHardware;
use stm32f0::stm32f0x2::Peripherals;

pub struct SystemHardwareImpl<'a> {
    p: &'a Peripherals,
    scb: &'a mut SCB,
}

impl<'a> SystemHardwareImpl<'a> {
    pub fn new(p: &'a Peripherals, scb: &'a mut SCB) -> Self {
        Self { p, scb }
    }
}

impl<'a> SystemHardwareImpl<'a> {
    fn toggle_deep_sleep(&mut self, on: bool) {
        // Toggle SLEEPDEEP bit of Cortex-M0 System Control Register.
        if on {
            self.scb.set_sleepdeep();
        } else {
            self.scb.clear_sleepdeep();
        }

        // Enter Standby mode when the CPU enters Deep Sleep.
        self.p.PWR.cr.modify(|_, w| w.pdds().bit(on));

        self.p.PWR.cr.modify(|_, w| w.cwuf().set_bit());
    }
}

impl<'a> SystemHardware for SystemHardwareImpl<'a> {
    type B = ButtonsHardwareImpl<'a>;
    type F = FlashHardwareImpl<'a>;
    type P = BeeperHardwareImpl<'a>;
    type R = RTCHardwareImpl<'a>;
    type U = USBHardwareImpl<'a>;
    type T = TimerHardwareImpl<'a>;

    fn setup(&self) {
        // Remap PA9-10 to PA11-12 for USB.
        self.p.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
        self.p
            .SYSCFG
            .cfgr1
            .modify(|_, w| w.pa11_pa12_rmp().set_bit().mem_mode().main_flash());

        // -----------Buttons----------------

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

        // ---------GPIO------------------

        // Enable clock for GPIO Port A, B and F.
        self.p
            .RCC
            .ahbenr
            .modify(|_, w| w.iopaen().set_bit().iopben().set_bit().iopfen().set_bit());

        // Switch PA0 (button), PA11 and PA12 (usb) to alternate function mode and
        // PA1, PA3-7 to AIN to reduce power consumption.
        self.p.GPIOA.moder.modify(|_, w| {
            w.moder0()
                .alternate()
                .moder1()
                .analog()
                .moder2()
                .alternate()
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
                .moder11()
                .alternate()
                .moder12()
                .alternate()
                .moder13()
                .analog()
                .moder14()
                .analog()
        });

        // Switch PB1 (beeper) to alternate function mode and PB8 to AIN to reduce power consumption.
        self.p
            .GPIOB
            .moder
            .modify(|_, w| w.moder1().alternate().moder8().analog());

        // Enable AIN for GPIO F to reduce power consumption.
        self.p
            .GPIOF
            .moder
            .modify(|_, w| w.moder0().analog().moder1().analog());

        self.p.RCC.ahbenr.modify(|_, w| w.iopfen().clear_bit());

        // Enable pull-down for PA0 and PA2.
        self.p
            .GPIOA
            .pupdr
            .modify(|_, w| w.pupdr0().pull_down().pupdr2().pull_down());

        // Set "high" output speed for PA11 and PA12.
        self.p.GPIOA.ospeedr.modify(|_, w| {
            w.ospeedr11()
                .very_high_speed()
                .ospeedr12()
                .very_high_speed()
        });

        // Set "high" output speed for PB1.
        self.p
            .GPIOB
            .ospeedr
            .modify(|_, w| w.ospeedr1().very_high_speed());

        // Set alternative function #2 for PA0 (WKUP1) and PA2 (WKUP4).
        self.p
            .GPIOA
            .afrl
            .modify(|_, w| w.afrl0().af2().afrl2().af2());

        // Set alternative function #2 for PB1 (TIM1_CH3N).
        self.p.GPIOB.afrl.modify(|_, w| w.afrl1().af2());
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

    fn beeper(&self) -> Self::P {
        BeeperHardwareImpl::new(&self.p)
    }

    fn buttons(&self) -> Self::B {
        ButtonsHardwareImpl::new(&self.p)
    }

    fn flash(&self) -> Self::F {
        FlashHardwareImpl::new(&self.p)
    }

    fn rtc(&self) -> Self::R {
        RTCHardwareImpl::new(&self.p)
    }

    fn timer(&self) -> Self::T {
        TimerHardwareImpl::new(&self.p)
    }

    fn usb(&self) -> Self::U {
        USBHardwareImpl::new(&self.p)
    }
}
