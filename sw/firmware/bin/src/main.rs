#![no_main]
#![no_std]
#![feature(uniform_paths)]
#![feature(non_exhaustive)]

extern crate panic_semihosting;

// mod beeper;
// mod button;
mod config;
// mod rtc;
// mod systick;
mod usb;

use core::cell::RefCell;

use core::ptr::read_volatile;
use cortex_m::{
    asm,
    interrupt::{free, Mutex},
    Peripherals as CorePeripherals,
};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32f0x2::{interrupt, Peripherals};
// use cortex_m_semihosting::hprintln;

// use beeper::Beeper;
// use button::Button;
use usb::{
    pma::{PacketMemoryArea, PacketMemoryArea1},
    USB,
};

struct AppState {
    device_peripherals: Peripherals,
    core_peripherals: CorePeripherals,
    usb: usb::UsbState,
    pma: &'static mut PacketMemoryArea,
    reset_count: u8,
    ctr_count: u16,
    debug_displayed: bool,
}

static STATE: Mutex<RefCell<Option<AppState>>> = Mutex::new(RefCell::new(None));

fn system_init(peripherals: &Peripherals) {
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

    // ---------------------------

    // Enable clock for GPIO Port A.
    peripherals.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());

    // Switch PA11 and PA12 to alternate function mode, PA2, PA3 and PA4 to output.
    let moder_af = 0b10;
    let moder_out = 0b01;
    peripherals.GPIOA.moder.modify(|_, w| unsafe {
        w.moder2()
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

    // Set "high" output speed for PA11 and PA12.
    let speed_high = 0b11;
    peripherals
        .GPIOA
        .ospeedr
        .modify(|_, w| unsafe { w.ospeedr11().bits(speed_high).ospeedr12().bits(speed_high) });

    // Set alternative function #2 (USB) for PA11 and PA12.
    let af2_usb = 0b0010;
    peripherals
        .GPIOA
        .afrh
        .modify(|_, w| unsafe { w.afrh11().bits(af2_usb).afrh12().bits(af2_usb) });
}
/*
fn toggle_leds(p: &Peripherals, on: bool) {
    p.GPIOA.bsrr.write(|w| {
        if on {
            w.bs2().set_bit().bs3().set_bit().bs4().set_bit()
        } else {
            w.br2().set_bit().br3().set_bit().br4().set_bit()
        }
    });
}*/

fn blue_on(p: &Peripherals) {
    p.GPIOA.bsrr.write(|w| w.bs2().set_bit());
}

pub fn red_on(p: &Peripherals) {
    p.GPIOA.bsrr.write(|w| w.bs4().set_bit());
}

pub fn green_on(p: &Peripherals) {
    p.GPIOA.bsrr.write(|w| w.bs3().set_bit());
}

pub fn has_address() -> bool {
    const A: *mut u32 = (0x4000_6000) as *mut u32;
    const B: *mut u32 = (0x4000_6000 + 0x8) as *mut u32;
    let a = unsafe { read_volatile(A) };
    let b = unsafe { read_volatile(B) };

    (a >> 16) as u16 == 0x60 && (b >> 16) as u16 == 0x20
}

/*pub fn toggle_number(p: &Peripherals, num: u8) {
    toggle_leds(p, false);

    match num {
        0 => {},
        1 => red_on(p),
        2 => green_on(p),
        3 => {
            green_on(p);
            red_on(p);
        },
        4 => blue_on(p),
        5 => {
            blue_on(p);
            red_on(p);
        }
        6 => {
            blue_on(p);
            green_on(p);
        }
        _ => {
            red_on(p);
            green_on(p);
            blue_on(p);
        }
    }
}*/

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[entry]
fn main() -> ! {
    free(|cs| {
        let pma = unsafe { &mut *PacketMemoryArea1.get() };

        *STATE.borrow(cs).borrow_mut() = Some(AppState {
            device_peripherals: Peripherals::take().unwrap(),
            core_peripherals: cortex_m::Peripherals::take().unwrap(),
            usb: usb::UsbState::default(),
            pma,
            reset_count: 0,
            ctr_count: 0,
            debug_displayed: false,
        });
    });

    interrupt_free(|state| {
        system_init(&state.device_peripherals);

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

/*
#[interrupt]
fn EXTI0_1() {
    interrupt_free(|state| {
        //toggle_leds(&state.device_peripherals, false);

        USB::acquire(
            &mut state.core_peripherals,
            &state.device_peripherals,
            &mut state.usb,
            state.pma,
            |mut usb| {
                usb.stop();
            },
        );
    });
}
*/

#[interrupt]
fn USB() {
    interrupt_free(|state| {
        /* let istr = state.device_peripherals.USB.istr.read();
        let is_reset = istr.reset().bit_is_set();
        let is_ctr = istr.ctr().bit_is_set();
        let is_ovr = istr.pmaovr().bit_is_set();
        let is_err = istr.err().bit_is_set();*/

        USB::acquire(
            &mut state.core_peripherals,
            &state.device_peripherals,
            &mut state.usb,
            state.pma,
            |mut usb| {
                usb.interrupt();
            },
        );

        //if is_ctr {
        // toggle_number(&state.device_peripherals, state.device_peripherals.USB.ep0r.read().stat_rx().bits());
        /*if state.device_peripherals.USB.ep0r.read().stat_rx().bits() == 0b11 {
            blue_on(&state.device_peripherals);
        } else {
            green_on(&state.device_peripherals);
        };*/
        //}

        /*
        if !is_reset {
            //state.reset_count = state.reset_count + 1;
            toggle_number(&state.device_peripherals, 5);
        }*/

        /* if is_ctr {
            state.ctr_count = state.ctr_count + 1;
            toggle_number(&state.device_peripherals, state.ctr_count as u8);
            if state.err_count % 300 == 0 {
                toggle_number(&state.device_peripherals, (state.err_count / 1000) as u8);
            }
        }*/

        /*if is_reset && !has_address() {
            blue_on(&state.device_peripherals);
        }

        if !is_reset && state.reset_count > 0 && !has_address() && !state.debug_displayed {
            green_on(&state.device_peripherals);

            if is_ctr {
                red_on(&state.device_peripherals);
            }
        }*/

        /*  if is_reset {
            toggle_number(&state.device_peripherals, state.esof_count);
        }*/

        /*if is_reset {
            const A: *mut u32 = (0x4000_6000) as *mut u32;
            const B: *mut u32 = (0x4000_6000 + 0x8) as *mut u32;
            let a = unsafe { read_volatile(A) };
            let b = unsafe { read_volatile(B) };

            if (a >> 16) as u16 == 0x50 && (b >> 16) as u16 == 0x10 {
                blue_on(&state.device_peripherals);
            }

            state.reset_count = state.reset_count + 1;
        } else if state.reset_count {
            const A: *mut u32 = (0x4000_6000) as *mut u32;
            const B: *mut u32 = (0x4000_6000 + 0x8) as *mut u32;
            let a = unsafe { read_volatile(A) };
            let b = unsafe { read_volatile(B) };

            if (a >> 16) as u16 == 0x50 && (b >> 16) as u16 == 0x10 {
                green_on(&state.device_peripherals);
            }
        }*/
    });
}

#[exception]
fn DefaultHandler(irqn: i16) {
    // interrupt_free(|state| toggle_leds(&state.device_peripherals, false));

    panic!("unhandled exception (IRQn={})", irqn);
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    // interrupt_free(|state| toggle_leds(&state.device_peripherals, false));

    loop {}
}

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
