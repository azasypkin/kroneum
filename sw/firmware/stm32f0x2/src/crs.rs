#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register"]
    pub cr: CR,
    #[doc = "0x04 - configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x08 - interrupt and status register"]
    pub isr: ISR,
    #[doc = "0x0c - interrupt flag clear register"]
    pub icr: ICR,
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
#[doc = "interrupt and status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "interrupt flag clear register"]
pub struct ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt flag clear register"]
pub mod icr;
