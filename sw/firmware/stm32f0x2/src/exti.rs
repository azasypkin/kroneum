#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt mask register (EXTI_IMR)"]
    pub imr: IMR,
    #[doc = "0x04 - Event mask register (EXTI_EMR)"]
    pub emr: EMR,
    #[doc = "0x08 - Rising Trigger selection register (EXTI_RTSR)"]
    pub rtsr: RTSR,
    #[doc = "0x0c - Falling Trigger selection register (EXTI_FTSR)"]
    pub ftsr: FTSR,
    #[doc = "0x10 - Software interrupt event register (EXTI_SWIER)"]
    pub swier: SWIER,
    #[doc = "0x14 - Pending register (EXTI_PR)"]
    pub pr: PR,
}
#[doc = "Interrupt mask register (EXTI_IMR)"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt mask register (EXTI_IMR)"]
pub mod imr;
#[doc = "Event mask register (EXTI_EMR)"]
pub struct EMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event mask register (EXTI_EMR)"]
pub mod emr;
#[doc = "Rising Trigger selection register (EXTI_RTSR)"]
pub struct RTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Rising Trigger selection register (EXTI_RTSR)"]
pub mod rtsr;
#[doc = "Falling Trigger selection register (EXTI_FTSR)"]
pub struct FTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Falling Trigger selection register (EXTI_FTSR)"]
pub mod ftsr;
#[doc = "Software interrupt event register (EXTI_SWIER)"]
pub struct SWIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Software interrupt event register (EXTI_SWIER)"]
pub mod swier;
#[doc = "Pending register (EXTI_PR)"]
pub struct PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pending register (EXTI_PR)"]
pub mod pr;
