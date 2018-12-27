#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN_MCR"]
    pub can_mcr: CAN_MCR,
    #[doc = "0x04 - CAN_MSR"]
    pub can_msr: CAN_MSR,
    #[doc = "0x08 - CAN_TSR"]
    pub can_tsr: CAN_TSR,
    #[doc = "0x0c - CAN_RF0R"]
    pub can_rf0r: CAN_RF0R,
    #[doc = "0x10 - CAN_RF1R"]
    pub can_rf1r: CAN_RF1R,
    #[doc = "0x14 - CAN_IER"]
    pub can_ier: CAN_IER,
    #[doc = "0x18 - CAN_ESR"]
    pub can_esr: CAN_ESR,
    #[doc = "0x1c - CAN BTR"]
    pub can_btr: CAN_BTR,
    _reserved0: [u8; 352usize],
    #[doc = "0x180 - CAN_TI0R"]
    pub can_ti0r: CAN_TI0R,
    #[doc = "0x184 - CAN_TDT0R"]
    pub can_tdt0r: CAN_TDT0R,
    #[doc = "0x188 - CAN_TDL0R"]
    pub can_tdl0r: CAN_TDL0R,
    #[doc = "0x18c - CAN_TDH0R"]
    pub can_tdh0r: CAN_TDH0R,
    #[doc = "0x190 - CAN_TI1R"]
    pub can_ti1r: CAN_TI1R,
    #[doc = "0x194 - CAN_TDT1R"]
    pub can_tdt1r: CAN_TDT1R,
    #[doc = "0x198 - CAN_TDL1R"]
    pub can_tdl1r: CAN_TDL1R,
    #[doc = "0x19c - CAN_TDH1R"]
    pub can_tdh1r: CAN_TDH1R,
    #[doc = "0x1a0 - CAN_TI2R"]
    pub can_ti2r: CAN_TI2R,
    #[doc = "0x1a4 - CAN_TDT2R"]
    pub can_tdt2r: CAN_TDT2R,
    #[doc = "0x1a8 - CAN_TDL2R"]
    pub can_tdl2r: CAN_TDL2R,
    #[doc = "0x1ac - CAN_TDH2R"]
    pub can_tdh2r: CAN_TDH2R,
    #[doc = "0x1b0 - CAN_RI0R"]
    pub can_ri0r: CAN_RI0R,
    #[doc = "0x1b4 - CAN_RDT0R"]
    pub can_rdt0r: CAN_RDT0R,
    #[doc = "0x1b8 - CAN_RDL0R"]
    pub can_rdl0r: CAN_RDL0R,
    #[doc = "0x1bc - CAN_RDH0R"]
    pub can_rdh0r: CAN_RDH0R,
    #[doc = "0x1c0 - CAN_RI1R"]
    pub can_ri1r: CAN_RI1R,
    #[doc = "0x1c4 - CAN_RDT1R"]
    pub can_rdt1r: CAN_RDT1R,
    #[doc = "0x1c8 - CAN_RDL1R"]
    pub can_rdl1r: CAN_RDL1R,
    #[doc = "0x1cc - CAN_RDH1R"]
    pub can_rdh1r: CAN_RDH1R,
    _reserved1: [u8; 48usize],
    #[doc = "0x200 - CAN_FMR"]
    pub can_fmr: CAN_FMR,
    #[doc = "0x204 - CAN_FM1R"]
    pub can_fm1r: CAN_FM1R,
    _reserved2: [u8; 4usize],
    #[doc = "0x20c - CAN_FS1R"]
    pub can_fs1r: CAN_FS1R,
    _reserved3: [u8; 4usize],
    #[doc = "0x214 - CAN_FFA1R"]
    pub can_ffa1r: CAN_FFA1R,
    _reserved4: [u8; 4usize],
    #[doc = "0x21c - CAN_FA1R"]
    pub can_fa1r: CAN_FA1R,
    _reserved5: [u8; 32usize],
    #[doc = "0x240 - Filter bank 0 register 1"]
    pub f0r1: F0R1,
    #[doc = "0x244 - Filter bank 0 register 2"]
    pub f0r2: F0R2,
    #[doc = "0x248 - Filter bank 1 register 1"]
    pub f1r1: F1R1,
    #[doc = "0x24c - Filter bank 1 register 2"]
    pub f1r2: F1R2,
    #[doc = "0x250 - Filter bank 2 register 1"]
    pub f2r1: F2R1,
    #[doc = "0x254 - Filter bank 2 register 2"]
    pub f2r2: F2R2,
    #[doc = "0x258 - Filter bank 3 register 1"]
    pub f3r1: F3R1,
    #[doc = "0x25c - Filter bank 3 register 2"]
    pub f3r2: F3R2,
    #[doc = "0x260 - Filter bank 4 register 1"]
    pub f4r1: F4R1,
    #[doc = "0x264 - Filter bank 4 register 2"]
    pub f4r2: F4R2,
    #[doc = "0x268 - Filter bank 5 register 1"]
    pub f5r1: F5R1,
    #[doc = "0x26c - Filter bank 5 register 2"]
    pub f5r2: F5R2,
    #[doc = "0x270 - Filter bank 6 register 1"]
    pub f6r1: F6R1,
    #[doc = "0x274 - Filter bank 6 register 2"]
    pub f6r2: F6R2,
    #[doc = "0x278 - Filter bank 7 register 1"]
    pub f7r1: F7R1,
    #[doc = "0x27c - Filter bank 7 register 2"]
    pub f7r2: F7R2,
    #[doc = "0x280 - Filter bank 8 register 1"]
    pub f8r1: F8R1,
    #[doc = "0x284 - Filter bank 8 register 2"]
    pub f8r2: F8R2,
    #[doc = "0x288 - Filter bank 9 register 1"]
    pub f9r1: F9R1,
    #[doc = "0x28c - Filter bank 9 register 2"]
    pub f9r2: F9R2,
    #[doc = "0x290 - Filter bank 10 register 1"]
    pub f10r1: F10R1,
    #[doc = "0x294 - Filter bank 10 register 2"]
    pub f10r2: F10R2,
    #[doc = "0x298 - Filter bank 11 register 1"]
    pub f11r1: F11R1,
    #[doc = "0x29c - Filter bank 11 register 2"]
    pub f11r2: F11R2,
    #[doc = "0x2a0 - Filter bank 4 register 1"]
    pub f12r1: F12R1,
    #[doc = "0x2a4 - Filter bank 12 register 2"]
    pub f12r2: F12R2,
    #[doc = "0x2a8 - Filter bank 13 register 1"]
    pub f13r1: F13R1,
    #[doc = "0x2ac - Filter bank 13 register 2"]
    pub f13r2: F13R2,
    #[doc = "0x2b0 - Filter bank 14 register 1"]
    pub f14r1: F14R1,
    #[doc = "0x2b4 - Filter bank 14 register 2"]
    pub f14r2: F14R2,
    #[doc = "0x2b8 - Filter bank 15 register 1"]
    pub f15r1: F15R1,
    #[doc = "0x2bc - Filter bank 15 register 2"]
    pub f15r2: F15R2,
    #[doc = "0x2c0 - Filter bank 16 register 1"]
    pub f16r1: F16R1,
    #[doc = "0x2c4 - Filter bank 16 register 2"]
    pub f16r2: F16R2,
    #[doc = "0x2c8 - Filter bank 17 register 1"]
    pub f17r1: F17R1,
    #[doc = "0x2cc - Filter bank 17 register 2"]
    pub f17r2: F17R2,
    #[doc = "0x2d0 - Filter bank 18 register 1"]
    pub f18r1: F18R1,
    #[doc = "0x2d4 - Filter bank 18 register 2"]
    pub f18r2: F18R2,
    #[doc = "0x2d8 - Filter bank 19 register 1"]
    pub f19r1: F19R1,
    #[doc = "0x2dc - Filter bank 19 register 2"]
    pub f19r2: F19R2,
    #[doc = "0x2e0 - Filter bank 20 register 1"]
    pub f20r1: F20R1,
    #[doc = "0x2e4 - Filter bank 20 register 2"]
    pub f20r2: F20R2,
    #[doc = "0x2e8 - Filter bank 21 register 1"]
    pub f21r1: F21R1,
    #[doc = "0x2ec - Filter bank 21 register 2"]
    pub f21r2: F21R2,
    #[doc = "0x2f0 - Filter bank 22 register 1"]
    pub f22r1: F22R1,
    #[doc = "0x2f4 - Filter bank 22 register 2"]
    pub f22r2: F22R2,
    #[doc = "0x2f8 - Filter bank 23 register 1"]
    pub f23r1: F23R1,
    #[doc = "0x2fc - Filter bank 23 register 2"]
    pub f23r2: F23R2,
    #[doc = "0x300 - Filter bank 24 register 1"]
    pub f24r1: F24R1,
    #[doc = "0x304 - Filter bank 24 register 2"]
    pub f24r2: F24R2,
    #[doc = "0x308 - Filter bank 25 register 1"]
    pub f25r1: F25R1,
    #[doc = "0x30c - Filter bank 25 register 2"]
    pub f25r2: F25R2,
    #[doc = "0x310 - Filter bank 26 register 1"]
    pub f26r1: F26R1,
    #[doc = "0x314 - Filter bank 26 register 2"]
    pub f26r2: F26R2,
    #[doc = "0x318 - Filter bank 27 register 1"]
    pub f27r1: F27R1,
    #[doc = "0x31c - Filter bank 27 register 2"]
    pub f27r2: F27R2,
}
#[doc = "CAN_MCR"]
pub struct CAN_MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_MCR"]
pub mod can_mcr;
#[doc = "CAN_MSR"]
pub struct CAN_MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_MSR"]
pub mod can_msr;
#[doc = "CAN_TSR"]
pub struct CAN_TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TSR"]
pub mod can_tsr;
#[doc = "CAN_RF0R"]
pub struct CAN_RF0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_RF0R"]
pub mod can_rf0r;
#[doc = "CAN_RF1R"]
pub struct CAN_RF1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_RF1R"]
pub mod can_rf1r;
#[doc = "CAN_IER"]
pub struct CAN_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_IER"]
pub mod can_ier;
#[doc = "CAN_ESR"]
pub struct CAN_ESR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_ESR"]
pub mod can_esr;
#[doc = "CAN BTR"]
pub struct CAN_BTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN BTR"]
pub mod can_btr;
#[doc = "CAN_TI0R"]
pub struct CAN_TI0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TI0R"]
pub mod can_ti0r;
#[doc = "CAN_TDT0R"]
pub struct CAN_TDT0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TDT0R"]
pub mod can_tdt0r;
#[doc = "CAN_TDL0R"]
pub struct CAN_TDL0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TDL0R"]
pub mod can_tdl0r;
#[doc = "CAN_TDH0R"]
pub struct CAN_TDH0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TDH0R"]
pub mod can_tdh0r;
#[doc = "CAN_TI1R"]
pub struct CAN_TI1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TI1R"]
pub mod can_ti1r;
#[doc = "CAN_TDT1R"]
pub struct CAN_TDT1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TDT1R"]
pub mod can_tdt1r;
#[doc = "CAN_TDL1R"]
pub struct CAN_TDL1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TDL1R"]
pub mod can_tdl1r;
#[doc = "CAN_TDH1R"]
pub struct CAN_TDH1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TDH1R"]
pub mod can_tdh1r;
#[doc = "CAN_TI2R"]
pub struct CAN_TI2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TI2R"]
pub mod can_ti2r;
#[doc = "CAN_TDT2R"]
pub struct CAN_TDT2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TDT2R"]
pub mod can_tdt2r;
#[doc = "CAN_TDL2R"]
pub struct CAN_TDL2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TDL2R"]
pub mod can_tdl2r;
#[doc = "CAN_TDH2R"]
pub struct CAN_TDH2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_TDH2R"]
pub mod can_tdh2r;
#[doc = "CAN_RI0R"]
pub struct CAN_RI0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_RI0R"]
pub mod can_ri0r;
#[doc = "CAN_RDT0R"]
pub struct CAN_RDT0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_RDT0R"]
pub mod can_rdt0r;
#[doc = "CAN_RDL0R"]
pub struct CAN_RDL0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_RDL0R"]
pub mod can_rdl0r;
#[doc = "CAN_RDH0R"]
pub struct CAN_RDH0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_RDH0R"]
pub mod can_rdh0r;
#[doc = "CAN_RI1R"]
pub struct CAN_RI1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_RI1R"]
pub mod can_ri1r;
#[doc = "CAN_RDT1R"]
pub struct CAN_RDT1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_RDT1R"]
pub mod can_rdt1r;
#[doc = "CAN_RDL1R"]
pub struct CAN_RDL1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_RDL1R"]
pub mod can_rdl1r;
#[doc = "CAN_RDH1R"]
pub struct CAN_RDH1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_RDH1R"]
pub mod can_rdh1r;
#[doc = "CAN_FMR"]
pub struct CAN_FMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_FMR"]
pub mod can_fmr;
#[doc = "CAN_FM1R"]
pub struct CAN_FM1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_FM1R"]
pub mod can_fm1r;
#[doc = "CAN_FS1R"]
pub struct CAN_FS1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_FS1R"]
pub mod can_fs1r;
#[doc = "CAN_FFA1R"]
pub struct CAN_FFA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_FFA1R"]
pub mod can_ffa1r;
#[doc = "CAN_FA1R"]
pub struct CAN_FA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN_FA1R"]
pub mod can_fa1r;
#[doc = "Filter bank 0 register 1"]
pub struct F0R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 0 register 1"]
pub mod f0r1;
#[doc = "Filter bank 0 register 2"]
pub struct F0R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 0 register 2"]
pub mod f0r2;
#[doc = "Filter bank 1 register 1"]
pub struct F1R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 1 register 1"]
pub mod f1r1;
#[doc = "Filter bank 1 register 2"]
pub struct F1R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 1 register 2"]
pub mod f1r2;
#[doc = "Filter bank 2 register 1"]
pub struct F2R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 2 register 1"]
pub mod f2r1;
#[doc = "Filter bank 2 register 2"]
pub struct F2R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 2 register 2"]
pub mod f2r2;
#[doc = "Filter bank 3 register 1"]
pub struct F3R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 3 register 1"]
pub mod f3r1;
#[doc = "Filter bank 3 register 2"]
pub struct F3R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 3 register 2"]
pub mod f3r2;
#[doc = "Filter bank 4 register 1"]
pub struct F4R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 4 register 1"]
pub mod f4r1;
#[doc = "Filter bank 4 register 2"]
pub struct F4R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 4 register 2"]
pub mod f4r2;
#[doc = "Filter bank 5 register 1"]
pub struct F5R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 5 register 1"]
pub mod f5r1;
#[doc = "Filter bank 5 register 2"]
pub struct F5R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 5 register 2"]
pub mod f5r2;
#[doc = "Filter bank 6 register 1"]
pub struct F6R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 6 register 1"]
pub mod f6r1;
#[doc = "Filter bank 6 register 2"]
pub struct F6R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 6 register 2"]
pub mod f6r2;
#[doc = "Filter bank 7 register 1"]
pub struct F7R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 7 register 1"]
pub mod f7r1;
#[doc = "Filter bank 7 register 2"]
pub struct F7R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 7 register 2"]
pub mod f7r2;
#[doc = "Filter bank 8 register 1"]
pub struct F8R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 8 register 1"]
pub mod f8r1;
#[doc = "Filter bank 8 register 2"]
pub struct F8R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 8 register 2"]
pub mod f8r2;
#[doc = "Filter bank 9 register 1"]
pub struct F9R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 9 register 1"]
pub mod f9r1;
#[doc = "Filter bank 9 register 2"]
pub struct F9R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 9 register 2"]
pub mod f9r2;
#[doc = "Filter bank 10 register 1"]
pub struct F10R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 10 register 1"]
pub mod f10r1;
#[doc = "Filter bank 10 register 2"]
pub struct F10R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 10 register 2"]
pub mod f10r2;
#[doc = "Filter bank 11 register 1"]
pub struct F11R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 11 register 1"]
pub mod f11r1;
#[doc = "Filter bank 11 register 2"]
pub struct F11R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 11 register 2"]
pub mod f11r2;
#[doc = "Filter bank 4 register 1"]
pub struct F12R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 4 register 1"]
pub mod f12r1;
#[doc = "Filter bank 12 register 2"]
pub struct F12R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 12 register 2"]
pub mod f12r2;
#[doc = "Filter bank 13 register 1"]
pub struct F13R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 13 register 1"]
pub mod f13r1;
#[doc = "Filter bank 13 register 2"]
pub struct F13R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 13 register 2"]
pub mod f13r2;
#[doc = "Filter bank 14 register 1"]
pub struct F14R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 14 register 1"]
pub mod f14r1;
#[doc = "Filter bank 14 register 2"]
pub struct F14R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 14 register 2"]
pub mod f14r2;
#[doc = "Filter bank 15 register 1"]
pub struct F15R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 15 register 1"]
pub mod f15r1;
#[doc = "Filter bank 15 register 2"]
pub struct F15R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 15 register 2"]
pub mod f15r2;
#[doc = "Filter bank 16 register 1"]
pub struct F16R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 16 register 1"]
pub mod f16r1;
#[doc = "Filter bank 16 register 2"]
pub struct F16R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 16 register 2"]
pub mod f16r2;
#[doc = "Filter bank 17 register 1"]
pub struct F17R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 17 register 1"]
pub mod f17r1;
#[doc = "Filter bank 17 register 2"]
pub struct F17R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 17 register 2"]
pub mod f17r2;
#[doc = "Filter bank 18 register 1"]
pub struct F18R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 18 register 1"]
pub mod f18r1;
#[doc = "Filter bank 18 register 2"]
pub struct F18R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 18 register 2"]
pub mod f18r2;
#[doc = "Filter bank 19 register 1"]
pub struct F19R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 19 register 1"]
pub mod f19r1;
#[doc = "Filter bank 19 register 2"]
pub struct F19R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 19 register 2"]
pub mod f19r2;
#[doc = "Filter bank 20 register 1"]
pub struct F20R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 20 register 1"]
pub mod f20r1;
#[doc = "Filter bank 20 register 2"]
pub struct F20R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 20 register 2"]
pub mod f20r2;
#[doc = "Filter bank 21 register 1"]
pub struct F21R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 21 register 1"]
pub mod f21r1;
#[doc = "Filter bank 21 register 2"]
pub struct F21R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 21 register 2"]
pub mod f21r2;
#[doc = "Filter bank 22 register 1"]
pub struct F22R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 22 register 1"]
pub mod f22r1;
#[doc = "Filter bank 22 register 2"]
pub struct F22R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 22 register 2"]
pub mod f22r2;
#[doc = "Filter bank 23 register 1"]
pub struct F23R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 23 register 1"]
pub mod f23r1;
#[doc = "Filter bank 23 register 2"]
pub struct F23R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 23 register 2"]
pub mod f23r2;
#[doc = "Filter bank 24 register 1"]
pub struct F24R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 24 register 1"]
pub mod f24r1;
#[doc = "Filter bank 24 register 2"]
pub struct F24R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 24 register 2"]
pub mod f24r2;
#[doc = "Filter bank 25 register 1"]
pub struct F25R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 25 register 1"]
pub mod f25r1;
#[doc = "Filter bank 25 register 2"]
pub struct F25R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 25 register 2"]
pub mod f25r2;
#[doc = "Filter bank 26 register 1"]
pub struct F26R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 26 register 1"]
pub mod f26r1;
#[doc = "Filter bank 26 register 2"]
pub struct F26R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 26 register 2"]
pub mod f26r2;
#[doc = "Filter bank 27 register 1"]
pub struct F27R1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 27 register 1"]
pub mod f27r1;
#[doc = "Filter bank 27 register 2"]
pub struct F27R2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter bank 27 register 2"]
pub mod f27r2;
