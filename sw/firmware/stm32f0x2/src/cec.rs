#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x08 - Tx data register"]
    pub txdr: TXDR,
    #[doc = "0x0c - Rx Data Register"]
    pub rxdr: RXDR,
    #[doc = "0x10 - Interrupt and Status Register"]
    pub isr: ISR,
    #[doc = "0x14 - interrupt enable register"]
    pub ier: IER,
}
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "configuration register"]
pub struct CFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "Tx data register"]
pub struct TXDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Tx data register"]
pub mod txdr;
#[doc = "Rx Data Register"]
pub struct RXDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rx Data Register"]
pub mod rxdr;
#[doc = "Interrupt and Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt and Status Register"]
pub mod isr;
#[doc = "interrupt enable register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt enable register"]
pub mod ier;
