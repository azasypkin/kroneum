#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Debug MCU Configuration Register"]
    pub cr: CR,
    #[doc = "0x08 - Debug MCU APB1 freeze register"]
    pub apb1_fz: APB1_FZ,
    #[doc = "0x0c - Debug MCU APB2 freeze register"]
    pub apb2_fz: APB2_FZ,
}
#[doc = "MCU Device ID Code Register"]
pub struct IDCODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "Debug MCU Configuration Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "Debug MCU APB1 freeze register"]
pub struct APB1_FZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug MCU APB1 freeze register"]
pub mod apb1_fz;
#[doc = "Debug MCU APB2 freeze register"]
pub struct APB2_FZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Debug MCU APB2 freeze register"]
pub mod apb2_fz;
