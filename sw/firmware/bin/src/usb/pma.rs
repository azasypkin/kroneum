use core::ops::Deref;

#[doc = r" USB PMA"]
#[repr(C)]
pub struct PacketMemoryAreaAccessor {
    // The PMA consists of 256 u16 words separated by u16 gaps, so lets
    // represent that as 512 u16 words which we'll only use every other of.
    cells: [vcell::VolatileCell<u16>; 512],
}

impl PacketMemoryAreaAccessor {
    pub fn clear(&self) {
        for i in 0..256 {
            self.set_u16(i * 2, 0);
        }
    }

    pub fn get_u16(&self, offset: usize) -> u16 {
        assert_eq!((offset & 0x01), 0);
        self.cells[offset].get()
    }

    pub fn set_u16(&self, offset: usize, val: u16) {
        assert_eq!((offset & 0x01), 0);
        self.cells[offset].set(val);
    }

    pub fn write_buffer_u8(&self, base_offset: usize, buf: &[u8]) {
        let mut last_value: u16 = 0;
        let mut last_offset: usize = 0;

        for (current_offset, value) in buf.iter().enumerate() {
            last_offset = current_offset;
            if current_offset & 1 == 0 {
                last_value = u16::from(*value);
            } else {
                self.set_u16(
                    (base_offset + current_offset) & !1,
                    last_value | (u16::from(*value) << 8),
                );
            }
        }

        if last_offset & 1 == 0 {
            self.set_u16(base_offset + last_offset, last_value);
        }
    }
}

pub struct PacketMemoryArea {}

impl PacketMemoryArea {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const PacketMemoryAreaAccessor {
        0x4000_6000 as *const _
    }
}

impl Deref for PacketMemoryArea {
    type Target = PacketMemoryAreaAccessor;
    fn deref(&self) -> &PacketMemoryAreaAccessor {
        unsafe { &*PacketMemoryArea::ptr() }
    }
}
