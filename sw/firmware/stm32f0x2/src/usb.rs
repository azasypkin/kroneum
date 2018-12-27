#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub ep0r: EP0R,
    #[doc = "0x04 - endpoint 1 register"]
    pub ep1r: EP1R,
    #[doc = "0x08 - endpoint 2 register"]
    pub ep2r: EP2R,
    #[doc = "0x0c - endpoint 3 register"]
    pub ep3r: EP3R,
    #[doc = "0x10 - endpoint 4 register"]
    pub ep4r: EP4R,
    #[doc = "0x14 - endpoint 5 register"]
    pub ep5r: EP5R,
    #[doc = "0x18 - endpoint 6 register"]
    pub ep6r: EP6R,
    #[doc = "0x1c - endpoint 7 register"]
    pub ep7r: EP7R,
    _reserved0: [u8; 32usize],
    #[doc = "0x40 - control register"]
    pub cntr: CNTR,
    #[doc = "0x44 - interrupt status register"]
    pub istr: ISTR,
    #[doc = "0x48 - frame number register"]
    pub fnr: FNR,
    #[doc = "0x4c - device address"]
    pub daddr: DADDR,
    #[doc = "0x50 - Buffer table address"]
    pub btable: BTABLE,
    #[doc = "0x54 - LPM control and status register"]
    pub lpmcsr: LPMCSR,
    #[doc = "0x58 - Battery charging detector"]
    pub bcdr: BCDR,
}
#[doc = "endpoint 0 register"]
pub struct EP0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "endpoint 0 register"]
pub mod ep0r;
#[doc = "endpoint 1 register"]
pub struct EP1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "endpoint 1 register"]
pub mod ep1r;
#[doc = "endpoint 2 register"]
pub struct EP2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "endpoint 2 register"]
pub mod ep2r;
#[doc = "endpoint 3 register"]
pub struct EP3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "endpoint 3 register"]
pub mod ep3r;
#[doc = "endpoint 4 register"]
pub struct EP4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "endpoint 4 register"]
pub mod ep4r;
#[doc = "endpoint 5 register"]
pub struct EP5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "endpoint 5 register"]
pub mod ep5r;
#[doc = "endpoint 6 register"]
pub struct EP6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "endpoint 6 register"]
pub mod ep6r;
#[doc = "endpoint 7 register"]
pub struct EP7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "endpoint 7 register"]
pub mod ep7r;
#[doc = "control register"]
pub struct CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cntr;
#[doc = "interrupt status register"]
pub struct ISTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "frame number register"]
pub struct FNR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "frame number register"]
pub mod fnr;
#[doc = "device address"]
pub struct DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "device address"]
pub mod daddr;
#[doc = "Buffer table address"]
pub struct BTABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Buffer table address"]
pub mod btable;
#[doc = "LPM control and status register"]
pub struct LPMCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LPM control and status register"]
pub mod lpmcsr;
#[doc = "Battery charging detector"]
pub struct BCDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Battery charging detector"]
pub mod bcdr;
