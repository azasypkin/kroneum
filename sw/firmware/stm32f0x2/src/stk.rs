#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SysTick control and status register"]
    pub csr: CSR,
    #[doc = "0x04 - SysTick reload value register"]
    pub rvr: RVR,
    #[doc = "0x08 - SysTick current value register"]
    pub cvr: CVR,
    #[doc = "0x0c - SysTick calibration value register"]
    pub calib: CALIB,
}
#[doc = "SysTick control and status register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick control and status register"]
pub mod csr;
#[doc = "SysTick reload value register"]
pub struct RVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick reload value register"]
pub mod rvr;
#[doc = "SysTick current value register"]
pub struct CVR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick current value register"]
pub mod cvr;
#[doc = "SysTick calibration value register"]
pub struct CALIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SysTick calibration value register"]
pub mod calib;
