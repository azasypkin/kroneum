#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

mod beeper;
mod buttons;
mod rtc;
mod system;
mod systick;
mod usb;

use rtfm::app;
use rtfm::Peripherals as CorePeripherals;
use stm32f0::stm32f0x2::Peripherals as DevicePeripherals;

use system::{System, SystemState};

pub struct Peripherals {
    device: DevicePeripherals,
    systick: cortex_m::peripheral::SYST,
}

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[app(device = stm32f0::stm32f0x2)]
const APP: () = {
    static mut STATE: SystemState = ();
    static mut PERIPHERALS: Peripherals = ();

    #[init]
    fn init() {
        let _core: CorePeripherals = core;
        let _device: DevicePeripherals = device;

        // Remap PA9-10 to PA11-12 for USB.
        _device.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
        _device
            .SYSCFG
            .cfgr1
            .modify(|_, w| w.pa11_pa12_rmp().set_bit().mem_mode().main_flash());

        // -----------Buttons----------------

        // Enable EXTI0 interrupt line for PA0 and EXTI2 for PA2.
        _device
            .SYSCFG
            .exticr1
            .modify(|_, w| w.exti0().pa0().exti2().pa2());

        // Configure PA0/PA2 to trigger an interrupt event on the EXTI0/EXTI2 line on a rising edge.
        _device
            .EXTI
            .rtsr
            .modify(|_, w| w.tr0().set_bit().tr2().set_bit());

        // Unmask the external interrupt line EXTI0\EXTI2 by setting the bit corresponding to the
        // EXTI0\EXTI2 "bit 0/2" in the EXT_IMR register.
        _device
            .EXTI
            .imr
            .modify(|_, w| w.mr0().set_bit().mr2().set_bit());

        // ---------GPIO------------------

        // Enable clock for GPIO Port A, B and F.
        _device
            .RCC
            .ahbenr
            .modify(|_, w| w.iopaen().set_bit().iopben().set_bit().iopfen().set_bit());

        // Switch PA0 (button), PA2 (button), PA7 (beeper), PA11 and PA12 (usb) to alternate function
        // mode and PA1, PA3-6 to AIN to reduce power consumption.
        _device.GPIOA.moder.modify(|_, w| {
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
                .alternate()
                .moder11()
                .alternate()
                .moder12()
                .alternate()
        });

        // Enable AIN for GPIO B and F to reduce power consumption.
        _device
            .GPIOB
            .moder
            .modify(|_, w| w.moder1().analog().moder8().analog());
        _device
            .GPIOF
            .moder
            .modify(|_, w| w.moder0().analog().moder1().analog());

        _device
            .RCC
            .ahbenr
            .modify(|_, w| w.iopben().clear_bit().iopfen().clear_bit());

        // Enable pull-down for PA0 and PA2.
        _device
            .GPIOA
            .pupdr
            .modify(|_, w| w.pupdr0().pull_down().pupdr2().pull_down());

        // Set "high" output speed for PA7, PA11 and PA12.
        _device.GPIOA.ospeedr.modify(|_, w| {
            w.ospeedr7()
                .very_high_speed()
                .ospeedr11()
                .very_high_speed()
                .ospeedr12()
                .very_high_speed()
        });

        // Set alternative function #2 for PA0 (WKUP1), PA2 (WKUP4) and PA7 (TIM1_CH1N).
        _device
            .GPIOA
            .afrl
            .modify(|_, w| w.afrl0().af2().afrl2().af2().afrl7().af2());

        // Set alternative function #2 (USB) for PA11 and PA12.
        _device
            .GPIOA
            .afrh
            .modify(|_, w| w.afrh11().af2().afrh12().af2());

        // Toggle SLEEPDEEP bit of Cortex-M0 System Control Register.
        _core.SCB.set_sleepdeep();

        let mut peripherals = Peripherals {
            device: _device,
            systick: _core.SYST,
        };

        let mut state = SystemState::default();
        System::acquire(&mut peripherals, &mut state, |mut system| system.setup());

        STATE = state;
        PERIPHERALS = peripherals;
    }

    #[idle(resources = [STATE, PERIPHERALS])]
    fn idle() -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[interrupt(resources = [STATE, PERIPHERALS])]
    fn EXTI2_3() {
        System::acquire(
            &mut resources.PERIPHERALS,
            &mut resources.STATE,
            |mut system| system.on_button_press(),
        );
    }

    #[interrupt(resources = [STATE, PERIPHERALS])]
    fn EXTI0_1() {
        System::acquire(
            &mut resources.PERIPHERALS,
            &mut resources.STATE,
            |mut system| system.on_button_press(),
        );
    }

    #[interrupt(resources = [STATE, PERIPHERALS])]
    fn RTC() {
        System::acquire(
            &mut resources.PERIPHERALS,
            &mut resources.STATE,
            |mut system| {
                system.on_rtc_alarm();
            },
        );
    }

    #[interrupt(resources = [STATE, PERIPHERALS])]
    fn USB() {
        System::acquire(
            &mut resources.PERIPHERALS,
            &mut resources.STATE,
            |mut system| {
                system.on_usb_packet();
            },
        );
    }
};
