#![no_main]
#![no_std]
#![feature(uniform_paths)]
#![feature(non_exhaustive)]

extern crate panic_semihosting;

mod button;
mod config;
mod systick;
mod usb;

use core::cell::RefCell;
use cortex_m::{
    interrupt::{free, Mutex},
    Peripherals as CorePeripherals,
};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32f0x2::{interrupt, Peripherals};

use button::Button;
use usb::{pma::PacketMemoryArea, USB};
use systick::SysTick;

struct AppState {
    device_peripherals: Peripherals,
    core_peripherals: CorePeripherals,
    usb: usb::UsbState,
    pma: PacketMemoryArea,
}

static STATE: Mutex<RefCell<Option<AppState>>> = Mutex::new(RefCell::new(None));

fn interrupt_free<F>(f: F) -> ()
where
    F: FnOnce(&mut AppState),
{
    free(|cs| {
        if let Some(s) = STATE.borrow(cs).borrow_mut().as_mut() {
            f(s);
        } else {
            panic!("Can not borrow peripherals!");
        }
    });
}

/// Initialize the system, configure clock, GPIOs and interrupts.
fn system_init(peripherals: &Peripherals) {
    // -------USB------------
    // Enable HSI48.
    peripherals.RCC.cr2.modify(|_, w| w.hsi48on().set_bit());
    while peripherals.RCC.cr2.read().hsi48rdy().bit_is_clear() {}

    // Use HSI48 as HCLK source.
    let sw_as_hsi48: u8 = 0b11;
    peripherals
        .RCC
        .cfgr
        .modify(|_, w| unsafe { w.sw().bits(sw_as_hsi48) });
    while peripherals.RCC.cfgr.read().sws().bits() != sw_as_hsi48 {}

    // Enable clock recovery system from USB SOF frames.
    peripherals.RCC.apb1enr.modify(|_, w| w.crsen().set_bit());

    // Before configuration, reset CRS registers to their default values.
    peripherals.RCC.apb1rstr.modify(|_, w| w.crsrst().set_bit());
    peripherals
        .RCC
        .apb1rstr
        .modify(|_, w| w.crsrst().clear_bit());

    // Configure Frequency Error Measurement.

    // Enable Automatic trimming.
    peripherals.CRS.cr.modify(|_, w| w.autotrimen().set_bit());
    // Enable Frequency error counter.
    peripherals.CRS.cr.modify(|_, w| w.cen().set_bit());

    // Remap PA9-10 to PA11-12 for USB.
    peripherals
        .RCC
        .apb2enr
        .modify(|_, w| w.syscfgen().set_bit());
    peripherals
        .SYSCFG_COMP
        .syscfg_cfgr1
        .modify(|_, w| unsafe { w.pa11_pa12_rmp().set_bit().mem_mode().bits(0) });

    // -----------Buttons----------------

    // Enable EXTI0 interrupt line for PA0.
    peripherals
        .SYSCFG_COMP
        .syscfg_exticr1
        .modify(|_, w| unsafe { w.exti0().bits(0) });

    // Configure PA0 to trigger an interrupt event on the EXTI0 line on a rising edge.
    peripherals.EXTI.rtsr.modify(|_, w| w.tr0().set_bit());

    // Unmask the external interrupt line EXTI0 by setting the bit corresponding to the
    // EXTI0 "bit 0" in the EXT_IMR register.
    peripherals.EXTI.imr.modify(|_, w| w.mr0().set_bit());

    // ---------GPIO------------------

    // Enable clock for GPIO Port A.
    peripherals.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());

    // Switch PA0, PA11 and PA12 to alternate function mode, PA2, PA3 and PA4 to output.
    let moder_af = 0b10;
    let moder_out = 0b01;
    peripherals.GPIOA.moder.modify(|_, w| unsafe {
        w.moder0()
            .bits(moder_af)
            .moder2()
            .bits(moder_out)
            .moder3()
            .bits(moder_out)
            .moder4()
            .bits(moder_out)
            .moder11()
            .bits(moder_af)
            .moder12()
            .bits(moder_af)
    });

    // Enable pull-down for PA0.
    peripherals
        .GPIOA
        .pupdr
        .modify(|_, w| unsafe { w.pupdr0().bits(0b10) });

    // Set "high" output speed for PA11 and PA12.
    let speed_high = 0b11;
    peripherals
        .GPIOA
        .ospeedr
        .modify(|_, w| unsafe { w.ospeedr11().bits(speed_high).ospeedr12().bits(speed_high) });

    // Set alternative function #2 (WKUP1) for PA0.
    let af2_wkup = 0b0010;
    peripherals
        .GPIOA
        .afrl
        .modify(|_, w| unsafe { w.afrl0().bits(af2_wkup) });

    // Set alternative function #2 (USB) for PA11 and PA12.
    let af2_usb = 0b0010;
    peripherals
        .GPIOA
        .afrh
        .modify(|_, w| unsafe { w.afrh11().bits(af2_usb).afrh12().bits(af2_usb) });
}

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[entry]
fn main() -> ! {
    free(|cs| {
        *STATE.borrow(cs).borrow_mut() = Some(AppState {
            device_peripherals: Peripherals::take().unwrap(),
            core_peripherals: cortex_m::Peripherals::take().unwrap(),
            usb: usb::UsbState::default(),
            pma: PacketMemoryArea {},
        });
    });

    interrupt_free(|state| {
        system_init(&state.device_peripherals);

        Button::acquire(
            &mut state.core_peripherals,
            &state.device_peripherals,
            |mut button| button.start(),
        );

        USB::acquire(
            &mut state.core_peripherals,
            &state.device_peripherals,
            &mut state.usb,
            &mut state.pma,
            |mut usb| usb.start(),
        );
    });

    loop {}
}

#[interrupt]
fn EXTI0_1() {
    interrupt_free(|state| {
        state.device_peripherals.GPIOA.bsrr.write(|w| w.bs2().set_bit());
        SysTick::delay_ms(&mut state.core_peripherals.SYST, 300);
        state.device_peripherals.GPIOA.bsrr.write(|w| w.br2().set_bit());

        Button::acquire(
            &mut state.core_peripherals,
            &state.device_peripherals,
            |button| {
                button.clear_pending_interrupt();
            },
        );
    });
}

#[interrupt]
fn USB() {
    interrupt_free(|state| {
        USB::acquire(
            &mut state.core_peripherals,
            &state.device_peripherals,
            &mut state.usb,
            &state.pma,
            |mut usb| {
                usb.interrupt();
            },
        );
    });
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    loop {}
}
