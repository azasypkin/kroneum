use super::{
    super::constants::MAX_PAYLOAD_SIZE,
    super::pipes::{
        Pipe0Register, Pipe1Register, Pipe2Register, Pipe3Register, Pipe4Register, Pipe5Register,
    },
    super::Register,
};
use bit_field::BitField;
use core::marker::PhantomData;

pub trait PipePayloadWidth {
    fn register_addr() -> u8;
}

impl PipePayloadWidth for Pipe0Register {
    fn register_addr() -> u8 {
        0x11
    }
}

impl PipePayloadWidth for Pipe1Register {
    fn register_addr() -> u8 {
        0x12
    }
}

impl PipePayloadWidth for Pipe2Register {
    fn register_addr() -> u8 {
        0x13
    }
}

impl PipePayloadWidth for Pipe3Register {
    fn register_addr() -> u8 {
        0x14
    }
}

impl PipePayloadWidth for Pipe4Register {
    fn register_addr() -> u8 {
        0x15
    }
}

impl PipePayloadWidth for Pipe5Register {
    fn register_addr() -> u8 {
        0x16
    }
}

/// Auto Acknowledgment register.
pub struct RxPipePayloadWidthRegister<TPipe: PipePayloadWidth>(
    [u8; 1],
    core::marker::PhantomData<TPipe>,
);

impl<TPipe: PipePayloadWidth> RxPipePayloadWidthRegister<TPipe> {
    /// Get pipe payload width.
    pub fn get(&self) -> u8 {
        self.0[0].get_bits(0..=5)
    }

    /// Set pipe payload width.
    pub fn set(&mut self, value: u8) -> &mut Self {
        self.0[0].set_bits(
            0..=5,
            if value > MAX_PAYLOAD_SIZE as u8 {
                MAX_PAYLOAD_SIZE as u8
            } else {
                value
            },
        );
        self
    }
}

impl<TPipe: PipePayloadWidth> Default for RxPipePayloadWidthRegister<TPipe> {
    fn default() -> Self {
        RxPipePayloadWidthRegister {
            0: [u8::default()],
            1: PhantomData,
        }
    }
}

impl<TPipe: PipePayloadWidth> Register for RxPipePayloadWidthRegister<TPipe> {
    type TRaw = [u8; 1];

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
