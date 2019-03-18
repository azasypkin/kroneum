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

use core::cell::RefCell;
use cortex_m::{
    interrupt::{free, Mutex},
    Peripherals as CorePeripherals,
};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32f0::stm32f0x2::{interrupt, Interrupt, Peripherals as DevicePeripherals};

static SYSTEM: Mutex<RefCell<Option<system::System>>> = Mutex::new(RefCell::new(None));

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[entry]
fn main() -> ! {
    let mut core_peripherals = CorePeripherals::take().expect("Can not take core peripherals");
    let mut device_peripherals =
        DevicePeripherals::take().expect("Can not take device peripherals");

    free(|cs| {
        configure(&mut device_peripherals, &mut core_peripherals);

        let mut system = system::System::new(
            device_peripherals,
            systick::get(core_peripherals.SYST),
            core_peripherals.SCB,
        );

        system.setup();

        *SYSTEM.borrow(cs).borrow_mut() = Some(system);
    });

    loop {
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn EXTI2_3() {
    interrupt_free(|system| system.on_button_press());
}

#[interrupt]
fn EXTI0_1() {
    interrupt_free(|system| system.on_button_press());
}

#[interrupt]
fn RTC() {
    interrupt_free(|system| system.on_rtc_alarm());
}

#[interrupt]
fn USB() {
    interrupt_free(|system| system.on_usb_packet());
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    panic!("hard fault (PC={})", _ef.pc);
}

fn interrupt_free<F>(f: F)
where
    F: FnOnce(&mut system::System),
{
    free(|cs| {
        if let Some(s) = SYSTEM.borrow(cs).borrow_mut().as_mut() {
            f(s);
        } else {
            panic!("Can not borrow application state!");
        }
    });
}

/// Initialize the system, configure clock, GPIOs and interrupts.
fn configure(device: &mut DevicePeripherals, core: &mut CorePeripherals) {
    // Remap PA9-10 to PA11-12 for USB.
    device.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());
    device
        .SYSCFG
        .cfgr1
        .modify(|_, w| w.pa11_pa12_rmp().set_bit().mem_mode().main_flash());

    // -----------Buttons----------------

    // Enable EXTI0 interrupt line for PA0 and EXTI2 for PA2.
    device
        .SYSCFG
        .exticr1
        .modify(|_, w| w.exti0().pa0().exti2().pa2());

    // Configure PA0/PA2 to trigger an interrupt event on the EXTI0/EXTI2 line on a rising edge.
    device
        .EXTI
        .rtsr
        .modify(|_, w| w.tr0().set_bit().tr2().set_bit());

    // Unmask the external interrupt line EXTI0\EXTI2 by setting the bit corresponding to the
    // EXTI0\EXTI2 "bit 0/2" in the EXT_IMR register.
    device
        .EXTI
        .imr
        .modify(|_, w| w.mr0().set_bit().mr2().set_bit());

    // ---------GPIO------------------

    // Enable clock for GPIO Port A, B and F.
    device
        .RCC
        .ahbenr
        .modify(|_, w| w.iopaen().set_bit().iopben().set_bit().iopfen().set_bit());

    // Switch PA0 (button), PA2 (button), PA7 (beeper), PA11 and PA12 (usb) to alternate function
    // mode and PA1, PA3-6 to AIN to reduce power consumption.
    device.GPIOA.moder.modify(|_, w| {
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
    device
        .GPIOB
        .moder
        .modify(|_, w| w.moder1().analog().moder8().analog());
    device
        .GPIOF
        .moder
        .modify(|_, w| w.moder0().analog().moder1().analog());

    device
        .RCC
        .ahbenr
        .modify(|_, w| w.iopben().clear_bit().iopfen().clear_bit());

    // Enable pull-down for PA0 and PA2.
    device
        .GPIOA
        .pupdr
        .modify(|_, w| w.pupdr0().pull_down().pupdr2().pull_down());

    // Set "high" output speed for PA7, PA11 and PA12.
    device.GPIOA.ospeedr.modify(|_, w| {
        w.ospeedr7()
            .very_high_speed()
            .ospeedr11()
            .very_high_speed()
            .ospeedr12()
            .very_high_speed()
    });

    // Set alternative function #2 for PA0 (WKUP1), PA2 (WKUP4) and PA7 (TIM1_CH1N).
    device
        .GPIOA
        .afrl
        .modify(|_, w| w.afrl0().af2().afrl2().af2().afrl7().af2());

    // Set alternative function #2 (USB) for PA11 and PA12.
    device
        .GPIOA
        .afrh
        .modify(|_, w| w.afrh11().af2().afrh12().af2());

    // Set priority for the interrupts
    unsafe {
        core.NVIC.set_priority(Interrupt::EXTI0_1, 1);
        core.NVIC.set_priority(Interrupt::EXTI2_3, 1);
        core.NVIC.set_priority(Interrupt::RTC, 2);
    }

    // Enable the interrupt in the NVIC.
    core.NVIC.enable(Interrupt::EXTI0_1);
    core.NVIC.enable(Interrupt::EXTI2_3);
    core.NVIC.enable(Interrupt::RTC);
    core.NVIC.enable(Interrupt::USB);
}
