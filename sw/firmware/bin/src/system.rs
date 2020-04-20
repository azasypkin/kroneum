use crate::hal::stm32::{
    Interrupt, Peripherals, ADC, CRS, EXTI, FLASH, PWR, RTC, SPI1, TIM1, TIM2, USB,
};
use crate::radio::SPIBus;
use cortex_m::interrupt::CriticalSection;
use cortex_m::peripheral::{NVIC, SCB};
use kroneum_api::system::SystemHardware;
use stm32f0xx_hal::{
    gpio::{
        gpioa::{PA0, PA2, PA3, PA4, PA5, PA6, PA7},
        Analog, GpioExt, Input, PullDown,
    },
    rcc::{Rcc, RccExt},
};

pub struct SystemHardwareImpl {
    pub(crate) adc: ADC,
    pub(crate) crs: CRS,
    pub(crate) exti: EXTI,
    pub(crate) flash: FLASH,
    pub(crate) pa0: PA0<Input<PullDown>>,
    pub(crate) pa2: PA2<Input<PullDown>>,
    pub(crate) pa3: Option<PA3<Analog>>,
    pub(crate) pa4: Option<PA4<Analog>>,
    pub(crate) pa5: Option<PA5<Analog>>,
    pub(crate) pa6: Option<PA6<Analog>>,
    pub(crate) pa7: Option<PA7<Analog>>,
    pub(crate) pwr: PWR,
    pub(crate) rcc: Rcc,
    pub(crate) rtc: RTC,
    scb: SCB,
    pub(crate) spi: Option<SPI1>,
    pub(crate) spi_bus: Option<SPIBus>,
    pub(crate) tim1: TIM1,
    pub(crate) tim2: TIM2,
    pub(crate) usb: USB,
}

impl SystemHardwareImpl {
    pub fn init(p: Peripherals, scb: SCB, mut nvic: NVIC, cs: &CriticalSection) -> Self {
        let Peripherals {
            ADC: adc,
            CRS: crs,
            EXTI: exti,
            FLASH: mut flash,
            GPIOA: gpio_a,
            GPIOB: gpio_b,
            GPIOF: gpio_f,
            PWR: pwr,
            RCC: rcc,
            RTC: rtc,
            SYSCFG: syscfg,
            SPI1: spi,
            TIM1: tim1,
            TIM2: tim2,
            USB: usb,
            ..
        } = p;

        let mut rcc = rcc.configure().freeze(&mut flash);
        rcc.regs.apb2enr.modify(|_, w| w.syscfgen().enabled());

        syscfg.cfgr1.modify(|_, w| w.mem_mode().main_flash());
        // Remap PA9-10 to PA11-12 for USB.
        syscfg.cfgr1.modify(|_, w| w.pa11_pa12_rmp().remapped());

        // Enable EXTI0 interrupt line for PA0 and EXTI2 for PA2:
        // 1. Configure PA0/PA2 to trigger an interrupt event on the EXTI0/EXTI2 line on a rising edge
        // 2. Unmask the external interrupt line EXTI0\EXTI2 by setting the bit corresponding to the
        //    EXTI0\EXTI2 "bit 0/2" in the EXT_IMR register.
        // 3. Enable wakers (WKUP1 - PA0, WKUP4 - PA2).
        syscfg.exticr1.modify(|_, w| w.exti0().pa0().exti2().pa2());
        exti.rtsr.modify(|_, w| w.tr0().set_bit().tr2().set_bit());
        exti.imr.modify(|_, w| w.mr0().set_bit().mr2().set_bit());
        pwr.csr.modify(|_, w| w.ewup1().set_bit().ewup4().set_bit());

        // Configure port A: switch PA0 (button I) and PA2 (button X) to being pull-down inputs,
        // PA11 (USB DP) and PA12 (USB DM) to alternate function mode #0 and PA1, PA3-7, PA13-14 to
        // analog mode to reduce power consumption since these are not used.
        let gpio_a = gpio_a.split(&mut rcc);
        let pa0 = gpio_a.pa0.into_pull_down_input(cs);
        let pa2 = gpio_a.pa2.into_pull_down_input(cs);
        gpio_a.pa11.into_alternate_af0(cs);
        gpio_a.pa12.into_alternate_af0(cs);
        gpio_a.pa1.into_analog(cs);
        let pa3 = gpio_a.pa3.into_analog(cs);
        let pa4 = gpio_a.pa4.into_analog(cs);
        let pa5 = gpio_a.pa5.into_analog(cs);
        let pa6 = gpio_a.pa6.into_analog(cs);
        let pa7 = gpio_a.pa7.into_analog(cs);
        gpio_a.pa13.into_analog(cs);
        gpio_a.pa14.into_analog(cs);

        // Configure port B: PB1 is used by beeper, so switch it into alternate function #2 (TIM1_CH3N)
        // and PB8 to analog mode to reduce power consumption since it's not used.
        let gpio_b = gpio_b.split(&mut rcc);
        gpio_b.pb1.into_alternate_af2(cs);
        gpio_b.pb8.into_analog(cs);

        // Configure port F: neither of its pins used, so switch all of them to analog mode to
        // reduce power consumption and disable clock.
        let gpio_f = gpio_f.split(&mut rcc);
        gpio_f.pf0.into_analog(cs);
        gpio_f.pf1.into_analog(cs);
        rcc.regs.ahbenr.modify(|_, w| w.iopfen().disabled());

        // Configure and enable interrupts.
        unsafe {
            nvic.set_priority(Interrupt::EXTI0_1, 1);
            nvic.set_priority(Interrupt::EXTI2_3, 1);
            nvic.set_priority(Interrupt::RTC, 1);
            nvic.set_priority(Interrupt::TIM2, 1);

            NVIC::unmask(Interrupt::EXTI0_1);
            NVIC::unmask(Interrupt::EXTI2_3);
            NVIC::unmask(Interrupt::RTC);
            NVIC::unmask(Interrupt::USB);
            NVIC::unmask(Interrupt::TIM2);
        }

        rcc.regs.apb2enr.modify(|_, w| w.syscfgen().disabled());

        Self {
            crs,
            adc,
            exti,
            flash,
            pa0,
            pa2,
            pa3: Some(pa3),
            pa4: Some(pa4),
            pa5: Some(pa5),
            pa6: Some(pa6),
            pa7: Some(pa7),
            pwr,
            rcc,
            rtc,
            scb,
            spi: Some(spi),
            spi_bus: None,
            tim1,
            tim2,
            usb,
        }
    }

    fn toggle_deep_sleep(&mut self, on: bool) {
        // Toggle SLEEPDEEP bit of Cortex-M0 System Control Register
        // and enter Standby mode when the CPU enters Deep Sleep.
        if on {
            self.scb.set_sleepdeep();
            self.pwr.cr.modify(|_, w| w.pdds().standby_mode());
        } else {
            self.scb.clear_sleepdeep();
            self.pwr.cr.modify(|_, w| w.pdds().stop_mode());
        }

        self.pwr.cr.modify(|_, w| w.cwuf().set_bit());
    }
}

impl SystemHardware for SystemHardwareImpl {
    fn enter_deep_sleep(&mut self) {
        self.toggle_deep_sleep(true);
    }

    fn exit_deep_sleep(&mut self) {
        self.toggle_deep_sleep(false);
    }

    fn reset(&mut self) {
        SCB::sys_reset();
    }

    fn device_id(&self) -> &'static [u8; 12] {
        stm32_device_signature::device_id()
    }

    fn device_id_hex(&self) -> &'static str {
        stm32_device_signature::device_id_hex()
    }

    fn flash_size_kb(&self) -> u16 {
        stm32_device_signature::flash_size_kb()
    }
}
