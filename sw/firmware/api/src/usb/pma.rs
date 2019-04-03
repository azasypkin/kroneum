use super::EndpointType;
use core::ops::Deref;

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
        self.set_u16((endpoint as usize) * 8, address)
    }

    pub fn _tx_count(&self, endpoint: EndpointType) -> u16 {
        self.u16((endpoint as usize) * 8 + 2)
    }

    pub fn set_tx_count(&self, endpoint: EndpointType, count: u16) {
        self.set_u16((endpoint as usize) * 8 + 2, count)
    }

    pub fn set_rx_addr(&self, endpoint: EndpointType, address: u16) {
        self.set_u16((endpoint as usize) * 8 + 4, address)
    }

    pub fn rx_count(&self, endpoint: EndpointType) -> u16 {
        self.u16((endpoint as usize) * 8 + 6) & 0x3ff
    }

    pub fn set_rx_count(&self, endpoint: EndpointType, count: u16) {
        // 32 byte size, 1 block = 64 bytes
        self.set_u16((endpoint as usize) * 8 + 6, 0x8400 | count)
    }

    pub fn read(&self, endpoint: EndpointType, offset: u16) -> u16 {
        assert_eq!((offset & 0x01), 0);
        match endpoint {
            EndpointType::Control => self.u16((CONTROL_OUT_PMA_ADDRESS + offset) as usize),
            EndpointType::Device => self.u16((DEVICE_OUT_PMA_ADDRESS + offset) as usize),
        }
    }

    pub fn write(&self, endpoint: EndpointType, buf: &[u8]) {
        match endpoint {
            EndpointType::Control => self.write_buffer_u8(CONTROL_IN_PMA_ADDRESS as usize, buf),
            EndpointType::Device => self.write_buffer_u8(DEVICE_IN_PMA_ADDRESS as usize, buf),
        }
    }

    fn u16(&self, offset: usize) -> u16 {
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
pub struct PacketMemoryArea {
    pub base_address: usize,
}

impl PacketMemoryArea {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr(base_address: usize) -> *const PacketMemoryAreaAccessor {
        base_address as *const _
    }
}

impl Deref for PacketMemoryArea {
    type Target = PacketMemoryAreaAccessor;
    fn deref(&self) -> &PacketMemoryAreaAccessor {
        unsafe { &*PacketMemoryArea::ptr(self.base_address) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_initializes() {
        let sandbox: [u16; 256] = [555; 256];
        let sandbox_address: usize = &sandbox as *const _ as usize;
        let pma = PacketMemoryArea {
            base_address: sandbox_address,
        };

        pma.init();

        assert_eq!(sandbox[0], CONTROL_IN_PMA_ADDRESS);
        assert_eq!(sandbox[2], CONTROL_OUT_PMA_ADDRESS);
        assert_eq!(pma.rx_count(EndpointType::Control), 0);
        assert_eq!(pma._tx_count(EndpointType::Control), 0);

        assert_eq!(sandbox[4], DEVICE_IN_PMA_ADDRESS);
        assert_eq!(sandbox[6], DEVICE_OUT_PMA_ADDRESS);
        assert_eq!(pma.rx_count(EndpointType::Device), 0);
        assert_eq!(pma._tx_count(EndpointType::Device), 0);
    }

    #[test]
    fn correctly_sets_count() {
        let sandbox: [u16; 256] = [555; 256];
        let sandbox_address: usize = &sandbox as *const _ as usize;
        let pma = PacketMemoryArea {
            base_address: sandbox_address,
        };

        pma.init();

        pma.set_tx_count(EndpointType::Control, 1);
        pma.set_rx_count(EndpointType::Control, 2);
        pma.set_tx_count(EndpointType::Device, 3);
        pma.set_rx_count(EndpointType::Device, 4);

        assert_eq!(1, pma._tx_count(EndpointType::Control));
        assert_eq!(1, sandbox[1]);
        assert_eq!(2, pma.rx_count(EndpointType::Control));
        assert_eq!(2, sandbox[3] & 0x00ff);

        assert_eq!(3, pma._tx_count(EndpointType::Device));
        assert_eq!(3, sandbox[5]);
        assert_eq!(4, pma.rx_count(EndpointType::Device));
        assert_eq!(4, sandbox[7] & 0x00ff);
    }

    #[test]
    fn correctly_reads_data() {
        let mut sandbox: [u16; 256] = [555; 256];
        let sandbox_address: usize = &sandbox as *const _ as usize;
        let pma = PacketMemoryArea {
            base_address: sandbox_address,
        };

        pma.init();

        sandbox[(CONTROL_OUT_PMA_ADDRESS >> 1) as usize] = 1;
        sandbox[(CONTROL_OUT_PMA_ADDRESS >> 1) as usize + 1] = 2;
        sandbox[(CONTROL_OUT_PMA_ADDRESS >> 1) as usize + 2] = 3;
        sandbox[(CONTROL_OUT_PMA_ADDRESS >> 1) as usize + 3] = 4;

        sandbox[(DEVICE_OUT_PMA_ADDRESS >> 1) as usize] = 5;
        sandbox[(DEVICE_OUT_PMA_ADDRESS >> 1) as usize + 1] = 6;
        sandbox[(DEVICE_OUT_PMA_ADDRESS >> 1) as usize + 2] = 7;
        sandbox[(DEVICE_OUT_PMA_ADDRESS >> 1) as usize + 3] = 8;

        assert_eq!(
            [1, 2, 3, 4, 5, 6, 7, 8],
            [
                pma.read(EndpointType::Control, 0),
                pma.read(EndpointType::Control, 2),
                pma.read(EndpointType::Control, 4),
                pma.read(EndpointType::Control, 6),
                pma.read(EndpointType::Device, 0),
                pma.read(EndpointType::Device, 2),
                pma.read(EndpointType::Device, 4),
                pma.read(EndpointType::Device, 6),
            ]
        );
    }

    #[test]
    fn correctly_writes_data() {
        let sandbox: [u16; 256] = [555; 256];
        let sandbox_address: usize = &sandbox as *const _ as usize;
        let pma = PacketMemoryArea {
            base_address: sandbox_address,
        };

        pma.init();

        pma.write(EndpointType::Control, &[1, 2, 3, 4]);
        pma.write(EndpointType::Device, &[5, 6, 7, 8]);

        assert_eq!(
            [1 | (2 << 8), 3 | (4 << 8), 5 | (6 << 8), 7 | (8 << 8)],
            [
                sandbox[(CONTROL_IN_PMA_ADDRESS >> 1) as usize],
                sandbox[(CONTROL_IN_PMA_ADDRESS >> 1) as usize + 1],
                sandbox[(DEVICE_IN_PMA_ADDRESS >> 1) as usize],
                sandbox[(DEVICE_IN_PMA_ADDRESS >> 1) as usize + 1]
            ]
        );
    }
}
