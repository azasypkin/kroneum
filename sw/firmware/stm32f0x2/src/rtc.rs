#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub tr: TR,
    #[doc = "0x04 - date register"]
    pub dr: DR,
    #[doc = "0x08 - control register"]
    pub cr: CR,
    #[doc = "0x0c - initialization and status register"]
    pub isr: ISR,
    #[doc = "0x10 - prescaler register"]
    pub prer: PRER,
    _reserved0: [u8; 8usize],
    #[doc = "0x1c - alarm A register"]
    pub alrmar: ALRMAR,
    _reserved1: [u8; 4usize],
    #[doc = "0x24 - write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - sub second register"]
    pub ssr: SSR,
    #[doc = "0x2c - shift control register"]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - timestamp time register"]
    pub tstr: TSTR,
    #[doc = "0x34 - timestamp date register"]
    pub tsdr: TSDR,
    #[doc = "0x38 - time-stamp sub second register"]
    pub tsssr: TSSSR,
    #[doc = "0x3c - calibration register"]
    pub calr: CALR,
    #[doc = "0x40 - tamper and alternate function configuration register"]
    pub tafcr: TAFCR,
    #[doc = "0x44 - alarm A sub second register"]
    pub alrmassr: ALRMASSR,
    _reserved2: [u8; 8usize],
    #[doc = "0x50 - backup register"]
    pub bkp0r: BKP0R,
    #[doc = "0x54 - backup register"]
    pub bkp1r: BKP1R,
    #[doc = "0x58 - backup register"]
    pub bkp2r: BKP2R,
    #[doc = "0x5c - backup register"]
    pub bkp3r: BKP3R,
    #[doc = "0x60 - backup register"]
    pub bkp4r: BKP4R,
}
#[doc = "time register"]
pub struct TR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "time register"]
pub mod tr;
#[doc = "date register"]
pub struct DR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "date register"]
pub mod dr;
#[doc = "control register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "control register"]
pub mod cr;
#[doc = "initialization and status register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "initialization and status register"]
pub mod isr;
#[doc = "prescaler register"]
pub struct PRER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "prescaler register"]
pub mod prer;
#[doc = "alarm A register"]
pub struct ALRMAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "write protection register"]
pub struct WPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "write protection register"]
pub mod wpr;
#[doc = "sub second register"]
pub struct SSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "sub second register"]
pub mod ssr;
#[doc = "shift control register"]
pub struct SHIFTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "shift control register"]
pub mod shiftr;
#[doc = "timestamp time register"]
pub struct TSTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "timestamp time register"]
pub mod tstr;
#[doc = "timestamp date register"]
pub struct TSDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "timestamp date register"]
pub mod tsdr;
#[doc = "time-stamp sub second register"]
pub struct TSSSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "time-stamp sub second register"]
pub mod tsssr;
#[doc = "calibration register"]
pub struct CALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "calibration register"]
pub mod calr;
#[doc = "tamper and alternate function configuration register"]
pub struct TAFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "tamper and alternate function configuration register"]
pub mod tafcr;
#[doc = "alarm A sub second register"]
pub struct ALRMASSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "backup register"]
pub struct BKP0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp0r;
#[doc = "backup register"]
pub struct BKP1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp1r;
#[doc = "backup register"]
pub struct BKP2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp2r;
#[doc = "backup register"]
pub struct BKP3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp3r;
#[doc = "backup register"]
pub struct BKP4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "backup register"]
pub mod bkp4r;
