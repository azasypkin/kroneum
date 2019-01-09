use super::EndpointType;
use core::ops::Deref;

const BTABLE_ADDRESS: usize = 0x4000_6000;
const CONTROL_OUT_PMA_ADDRESS: u16 = 0x10;
const CONTROL_IN_PMA_ADDRESS: u16 = 0x50;
const DEVICE_IN_PMA_ADDRESS: u16 = 0x90;
const DEVICE_OUT_PMA_ADDRESS: u16 = 0xD0;

#[doc = r" USB PMA"]
#[repr(C)]
pub struct PacketMemoryAreaAccessor {
    // The PMA consists of 256 u16 words separated by u16 gaps, so lets
    // represent that as 512 u16 words which we'll only use every other of.
    cells: [vcell::VolatileCell<u16>; 256],
}

impl PacketMemoryAreaAccessor {
    pub fn init(&self) {
        for i in 0..256 {
            self.cells[i].set(0);
        }

        self.set_tx_addr(EndpointType::Control, CONTROL_IN_PMA_ADDRESS);
        self.set_tx_count(EndpointType::Control, 0);
        self.set_rx_addr(EndpointType::Control, CONTROL_OUT_PMA_ADDRESS);
        self.set_rx_count(EndpointType::Control, 0);

        self.set_tx_addr(EndpointType::Device, DEVICE_IN_PMA_ADDRESS);
        self.set_tx_count(EndpointType::Device, 0);
        self.set_rx_addr(EndpointType::Device, DEVICE_OUT_PMA_ADDRESS);
        self.set_rx_count(EndpointType::Device, 0);
    }

    pub fn set_tx_addr(&self, endpoint: EndpointType, address: u16) {
        return self.set_u16((endpoint as usize) * 8, address);
    }

    pub fn get_tx_count(&self, endpoint: EndpointType) -> u16 {
        return self.get_u16((endpoint as usize) * 8 + 2);
    }

    pub fn set_tx_count(&self, endpoint: EndpointType, count: u16) {
        return self.set_u16((endpoint as usize) * 8 + 2, count);
    }

    pub fn set_rx_addr(&self, endpoint: EndpointType, address: u16) {
        return self.set_u16((endpoint as usize) * 8 + 4, address);
    }

    pub fn get_rx_count(&self, endpoint: EndpointType) -> u16 {
        return self.get_u16((endpoint as usize) * 8 + 6) & 0x3ff;
    }

    pub fn set_rx_count(&self, endpoint: EndpointType, count: u16) {
        // 32 byte size, 1 block = 64 bytes
        return self.set_u16((endpoint as usize) * 8 + 6, 0x8400 | count);
    }

    pub fn read(&self, endpoint: EndpointType, offset: u16) -> u16 {
        assert_eq!((offset & 0x01), 0);
        match endpoint {
            EndpointType::Control => self.get_u16((CONTROL_OUT_PMA_ADDRESS + offset) as usize),
            EndpointType::Device => self.get_u16((DEVICE_OUT_PMA_ADDRESS + offset) as usize),
        }
    }

    pub fn write(&self, endpoint: EndpointType, buf: &[u8]) {
        match endpoint {
            EndpointType::Control => self.write_buffer_u8(CONTROL_IN_PMA_ADDRESS as usize, buf),
            EndpointType::Device => self.write_buffer_u8(DEVICE_OUT_PMA_ADDRESS as usize, buf),
        }
    }

    fn get_u16(&self, offset: usize) -> u16 {
        assert_eq!((offset & 0x01), 0);
        self.cells[offset >> 1].get()
    }

    fn set_u16(&self, offset: usize, val: u16) {
        assert_eq!((offset & 0x01), 0);
        self.cells[offset >> 1].set(val);
    }

    fn write_buffer_u8(&self, base_offset: usize, buf: &[u8]) {
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

#[repr(C)]
pub struct PacketMemoryArea {}

impl PacketMemoryArea {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const PacketMemoryAreaAccessor {
        BTABLE_ADDRESS as *const _
    }
}

impl Deref for PacketMemoryArea {
    type Target = PacketMemoryAreaAccessor;
    fn deref(&self) -> &PacketMemoryAreaAccessor {
        unsafe { &*PacketMemoryArea::ptr() }
    }
}
