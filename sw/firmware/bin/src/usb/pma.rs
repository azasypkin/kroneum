use bare_metal::Peripheral;
use core::ops::Deref;
use core::ptr::{read_volatile, write_volatile};

// TODO: make this take-able? or at least move into the main usb part
pub const PacketMemoryArea1: Peripheral<PacketMemoryArea> = unsafe { Peripheral::new(0x4000_6000) };
//const BTABLE: usize = 0;

#[repr(C)]
pub struct PacketMemoryArea {
    pub pma_area: PMA_Area,
}

impl PacketMemoryArea {
    pub fn clear(&mut self) {
        for i in 0..256 {
            self.pma_area.set_u16(i * 2, 0);
        }
    }
}

impl Deref for PacketMemoryArea {
    type Target = PMA_Area;
    fn deref(&self) -> &PMA_Area {
        &self.pma_area
    }
}

#[repr(C)]
pub struct PMA_Area {
    // The PMA consists of 256 u16 words separated by u16 gaps, so lets
    // represent that as 512 u16 words which we'll only use every other of.
    words: [vcell::VolatileCell<u16>; 512],
}

impl PMA_Area {
    // TODO: use operator overloading and just impl [] access, without double-counting
    pub fn get_u16(&self, offset: usize) -> u16 {
        assert_eq!((offset & 0x01), 0);
        return unsafe { read_volatile((0x4000_6000 + offset) as *mut u16) };
        // self.words[offset].get()
    }

    pub fn set_u16(&self, offset: usize, val: u16) {
        assert_eq!((offset & 0x01), 0);
        unsafe { write_volatile((0x4000_6000 + offset) as *mut u16, val) };
        // self.words[offset].set(val);
    }

    pub fn write_buffer_u8(&self, base: usize, buf: &[u8]) {
        let mut last: u16 = 0;
        let mut off: usize = 0;

        for (ofs, v) in buf.iter().enumerate() {
            off = ofs;
            if ofs & 1 == 0 {
                last = u16::from(*v);
            } else {
                self.set_u16((base + ofs) & !1, last | (u16::from(*v) << 8));
            }
        }

        if off & 1 == 0 {
            //self.set_u16(base + (off * 2), last);
            self.set_u16(base + off, last);
        }
    }
}
