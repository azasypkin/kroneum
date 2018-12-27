#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x04 - interrupt enable register"]
    pub ier: IER,
    #[doc = "0x08 - control register"]
    pub cr: CR,
    #[doc = "0x0c - configuration register 1"]
    pub cfgr1: CFGR1,
    #[doc = "0x10 - configuration register 2"]
    pub cfgr2: CFGR2,
    #[doc = "0x14 - sampling time register"]
    pub smpr: SMPR,
    _reserved0: [u8; 8usize],
    #[doc = "0x20 - watchdog threshold register"]
    pub tr: TR,
    _reserved1: [u8; 4usize],
    #[doc = "0x28 - channel selection register"]
    pub chselr: CHSELR,
    _reserved2: [u8; 20usize],
    #[doc = "0x40 - data register"]
    pub dr: DR,
    _reserved3: [u8; 708usize],
    #[doc = "0x308 - common configuration register"]
    pub ccr: CCR,
}
#[doc = "interrupt and status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "interrupt enable register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "configuration register 1"]
pub struct CFGR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "configuration register 1"]
pub mod cfgr1;
#[doc = "configuration register 2"]
pub struct CFGR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "configuration register 2"]
pub mod cfgr2;
#[doc = "sampling time register"]
pub struct SMPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "sampling time register"]
pub mod smpr;
#[doc = "watchdog threshold register"]
pub struct TR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "watchdog threshold register"]
pub mod tr;
#[doc = "channel selection register"]
pub struct CHSELR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "channel selection register"]
pub mod chselr;
#[doc = "data register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "data register"]
pub mod dr;
#[doc = "common configuration register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "common configuration register"]
pub mod ccr;
