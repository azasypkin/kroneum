use core::marker::PhantomData;
use radio::{
    constants::MAX_REGISTER_VALUE_SIZE,
    pipes::{
        Pipe0Register, Pipe1Register, Pipe2Register, Pipe3Register, Pipe4Register, Pipe5Register,
    },
    registers::Register,
};

pub trait PipeAddress {
    fn register_addr() -> u8;
}

impl PipeAddress for Pipe0Register {
    fn register_addr() -> u8 {
        0x0A
    }
}

impl PipeAddress for Pipe1Register {
    fn register_addr() -> u8 {
        0x0B
    }
}

impl PipeAddress for Pipe2Register {
    fn register_addr() -> u8 {
        0x0C
    }
}

impl PipeAddress for Pipe3Register {
    fn register_addr() -> u8 {
        0x0D
    }
}

impl PipeAddress for Pipe4Register {
    fn register_addr() -> u8 {
        0x0E
    }
}

impl PipeAddress for Pipe5Register {
    fn register_addr() -> u8 {
        0x0F
    }
}

/// Auto Acknowledgment register.
pub struct RxPipeAddressRegister<TPipe: PipeAddress>([u8; 5], core::marker::PhantomData<TPipe>);
impl<TPipe: PipeAddress> Default for RxPipeAddressRegister<TPipe> {
    fn default() -> Self {
        RxPipeAddressRegister {
            0: [u8::default(); MAX_REGISTER_VALUE_SIZE],
            1: PhantomData,
        }
    }
}

impl<TPipe: PipeAddress> Register for RxPipeAddressRegister<TPipe> {
    type TRaw = [u8; 5];

    fn address() -> u8 {
        TPipe::register_addr()
    }

    fn raw(&self) -> Self::TRaw {
        self.0
    }

    fn from_raw(buffer: Self::TRaw) -> Self {
        Self {
            0: buffer,
            1: PhantomData,
        }
    }
}
