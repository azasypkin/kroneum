#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration register 1"]
    pub syscfg_cfgr1: SYSCFG_CFGR1,
    _reserved0: [u8; 4usize],
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub syscfg_exticr1: SYSCFG_EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub syscfg_exticr2: SYSCFG_EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub syscfg_exticr3: SYSCFG_EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub syscfg_exticr4: SYSCFG_EXTICR4,
    #[doc = "0x18 - configuration register 2"]
    pub syscfg_cfgr2: SYSCFG_CFGR2,
    #[doc = "0x1c - control and status register"]
    pub comp_csr: COMP_CSR,
}
#[doc = "configuration register 1"]
pub struct SYSCFG_CFGR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "configuration register 1"]
pub mod syscfg_cfgr1;
#[doc = "external interrupt configuration register 1"]
pub struct SYSCFG_EXTICR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 1"]
pub mod syscfg_exticr1;
#[doc = "external interrupt configuration register 2"]
pub struct SYSCFG_EXTICR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 2"]
pub mod syscfg_exticr2;
#[doc = "external interrupt configuration register 3"]
pub struct SYSCFG_EXTICR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 3"]
pub mod syscfg_exticr3;
#[doc = "external interrupt configuration register 4"]
pub struct SYSCFG_EXTICR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "external interrupt configuration register 4"]
pub mod syscfg_exticr4;
#[doc = "configuration register 2"]
pub struct SYSCFG_CFGR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "configuration register 2"]
pub mod syscfg_cfgr2;
#[doc = "control and status register"]
pub struct COMP_CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control and status register"]
pub mod comp_csr;
