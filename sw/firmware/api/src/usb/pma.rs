use super::{descriptors::MAX_PACKET_SIZE, endpoint::EndpointType};
use core::ops::Deref;

#[doc = r" USB PMA"]
#[repr(C)]
pub struct PacketMemoryAreaAccessor {
    // The PMA consists of 256 u16 words separated by u16 gaps, so lets
    // represent that as 512 u16 words which we'll only use every other of.
    cells: [vcell::VolatileCell<u16>; 256],
}

impl PacketMemoryAreaAccessor {
    pub fn init(&self, endpoints: &[EndpointType]) {
        for i in 0..256 {
            self.cells[i].set(0);
        }

        let base_address = endpoints.len() * 8;
        for endpoint in endpoints {
            let endpoint_index = Into::<u8>::into(*endpoint) as usize;

            // Set TX address.
            self.set_u16(
                endpoint_index * 8,
                (base_address + 2 * endpoint_index * MAX_PACKET_SIZE) as u16,
            );
            self.set_tx_count(*endpoint, 0);

            // Set RX address.
            self.set_u16(
                endpoint_index * 8 + 4,
                (base_address + (2 * endpoint_index + 1) as usize * MAX_PACKET_SIZE) as u16,
            );
            self.set_rx_count(*endpoint, 0);
        }
    }

    pub fn _tx_count(&self, endpoint: EndpointType) -> u16 {
        self.u16((Into::<u8>::into(endpoint) as usize) * 8 + 2)
    }

    pub fn set_tx_count(&self, endpoint: EndpointType, count: u16) {
        self.set_u16((Into::<u8>::into(endpoint) as usize) * 8 + 2, count)
    }

    pub fn rx_count(&self, endpoint: EndpointType) -> u16 {
        self.u16((Into::<u8>::into(endpoint) as usize) * 8 + 6) & 0x3ff
    }

    pub fn set_rx_count(&self, endpoint: EndpointType, count: u16) {
        // 32 byte size, 1 block = 64 bytes
        self.set_u16(
            (Into::<u8>::into(endpoint) as usize) * 8 + 6,
            0x8400 | count,
        )
    }

    pub fn read(&self, endpoint: EndpointType, offset: u16) -> u16 {
        assert_eq!((offset & 0x01), 0);
        let base_offset = self.u16((Into::<u8>::into(endpoint) * 8 + 4) as usize);
        self.u16((base_offset + offset) as usize)
    }

    pub fn write<'a, T: IntoIterator<Item = &'a u8>>(
        &self,
        endpoint: EndpointType,
        buf: T,
    ) -> usize {
        let base_offset = self.u16((Into::<u8>::into(endpoint) * 8) as usize) as usize;
        let mut offset = 0;

        let mut iter = buf.into_iter();
        while let Some(low) = iter.next() {
            let high = iter.next();
            self.set_u16(
                (base_offset + offset) & !1,
                (u16::from(*high.unwrap_or_else(|| &0)) << 8) | u16::from(*low),
            );

            offset += if high.is_none() { 1 } else { 2 };
        }

        offset
    }

    fn u16(&self, offset: usize) -> u16 {
        assert_eq!((offset & 0x01), 0);
        self.cells[offset >> 1].get()
    }

    fn set_u16(&self, offset: usize, val: u16) {
        assert_eq!((offset & 0x01), 0);
        self.cells[offset >> 1].set(val);
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
    use super::{
        super::{endpoint::DeviceEndpoint, SUPPORTED_ENDPOINTS},
        *,
    };

    const CONTROL_IN_PMA_ADDRESS: u16 = 0x18;
    const CONTROL_OUT_PMA_ADDRESS: u16 = 0x58;
    const DEVICE_SYSTEM_IN_PMA_ADDRESS: u16 = 0x98;
    const DEVICE_SYSTEM_OUT_PMA_ADDRESS: u16 = 0xD8;
    const DEVICE_KEYBOARD_IN_PMA_ADDRESS: u16 = 0x118;
    const DEVICE_KEYBOARD_OUT_PMA_ADDRESS: u16 = 0x158;

    #[test]
    fn correctly_initializes() {
        let sandbox: [u16; 256] = [555; 256];
        let sandbox_address: usize = &sandbox as *const _ as usize;
        let pma = PacketMemoryArea {
            base_address: sandbox_address,
        };

        pma.init(&SUPPORTED_ENDPOINTS);

        assert_eq!(sandbox[0], CONTROL_IN_PMA_ADDRESS);
        assert_eq!(sandbox[2], CONTROL_OUT_PMA_ADDRESS);
        assert_eq!(pma.rx_count(EndpointType::Control), 0);
        assert_eq!(pma._tx_count(EndpointType::Control), 0);

        assert_eq!(sandbox[4], DEVICE_SYSTEM_IN_PMA_ADDRESS);
        assert_eq!(sandbox[6], DEVICE_SYSTEM_OUT_PMA_ADDRESS);
        assert_eq!(
            pma.rx_count(EndpointType::Device(DeviceEndpoint::System)),
            0
        );
        assert_eq!(
            pma._tx_count(EndpointType::Device(DeviceEndpoint::System)),
            0
        );

        assert_eq!(sandbox[8], DEVICE_KEYBOARD_IN_PMA_ADDRESS);
        assert_eq!(sandbox[10], DEVICE_KEYBOARD_OUT_PMA_ADDRESS);
        assert_eq!(
            pma.rx_count(EndpointType::Device(DeviceEndpoint::Keyboard)),
            0
        );
        assert_eq!(
            pma._tx_count(EndpointType::Device(DeviceEndpoint::Keyboard)),
            0
        );
    }

    #[test]
    fn correctly_sets_count() {
        let sandbox: [u16; 256] = [555; 256];
        let sandbox_address: usize = &sandbox as *const _ as usize;
        let pma = PacketMemoryArea {
            base_address: sandbox_address,
        };

        pma.init(&SUPPORTED_ENDPOINTS);

        pma.set_tx_count(EndpointType::Control, 1);
        pma.set_rx_count(EndpointType::Control, 2);
        pma.set_tx_count(EndpointType::Device(DeviceEndpoint::System), 3);
        pma.set_rx_count(EndpointType::Device(DeviceEndpoint::System), 4);
        pma.set_tx_count(EndpointType::Device(DeviceEndpoint::Keyboard), 5);
        pma.set_rx_count(EndpointType::Device(DeviceEndpoint::Keyboard), 6);

        assert_eq!(1, pma._tx_count(EndpointType::Control));
        assert_eq!(1, sandbox[1]);
        assert_eq!(2, pma.rx_count(EndpointType::Control));
        assert_eq!(2, sandbox[3] & 0x00ff);

        assert_eq!(
            3,
            pma._tx_count(EndpointType::Device(DeviceEndpoint::System))
        );
        assert_eq!(3, sandbox[5]);
        assert_eq!(
            4,
            pma.rx_count(EndpointType::Device(DeviceEndpoint::System))
        );
        assert_eq!(4, sandbox[7] & 0x00ff);

        assert_eq!(
            5,
            pma._tx_count(EndpointType::Device(DeviceEndpoint::Keyboard))
        );
        assert_eq!(5, sandbox[9]);
        assert_eq!(
            6,
            pma.rx_count(EndpointType::Device(DeviceEndpoint::Keyboard))
        );
        assert_eq!(6, sandbox[11] & 0x00ff);
    }

    #[test]
    fn correctly_reads_data() {
        let mut sandbox: [u16; 256] = [555; 256];
        let sandbox_address: usize = &sandbox as *const _ as usize;
        let pma = PacketMemoryArea {
            base_address: sandbox_address,
        };

        pma.init(&SUPPORTED_ENDPOINTS);

        sandbox[(CONTROL_OUT_PMA_ADDRESS >> 1) as usize] = 1;
        sandbox[(CONTROL_OUT_PMA_ADDRESS >> 1) as usize + 1] = 2;
        sandbox[(CONTROL_OUT_PMA_ADDRESS >> 1) as usize + 2] = 3;
        sandbox[(CONTROL_OUT_PMA_ADDRESS >> 1) as usize + 3] = 4;

        sandbox[(DEVICE_SYSTEM_OUT_PMA_ADDRESS >> 1) as usize] = 5;
        sandbox[(DEVICE_SYSTEM_OUT_PMA_ADDRESS >> 1) as usize + 1] = 6;
        sandbox[(DEVICE_SYSTEM_OUT_PMA_ADDRESS >> 1) as usize + 2] = 7;
        sandbox[(DEVICE_SYSTEM_OUT_PMA_ADDRESS >> 1) as usize + 3] = 8;

        sandbox[(DEVICE_KEYBOARD_OUT_PMA_ADDRESS >> 1) as usize] = 9;
        sandbox[(DEVICE_KEYBOARD_OUT_PMA_ADDRESS >> 1) as usize + 1] = 10;
        sandbox[(DEVICE_KEYBOARD_OUT_PMA_ADDRESS >> 1) as usize + 2] = 11;
        sandbox[(DEVICE_KEYBOARD_OUT_PMA_ADDRESS >> 1) as usize + 3] = 12;

        assert_eq!(
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [
                pma.read(EndpointType::Control, 0),
                pma.read(EndpointType::Control, 2),
                pma.read(EndpointType::Control, 4),
                pma.read(EndpointType::Control, 6),
                pma.read(EndpointType::Device(DeviceEndpoint::System), 0),
                pma.read(EndpointType::Device(DeviceEndpoint::System), 2),
                pma.read(EndpointType::Device(DeviceEndpoint::System), 4),
                pma.read(EndpointType::Device(DeviceEndpoint::System), 6),
                pma.read(EndpointType::Device(DeviceEndpoint::Keyboard), 0),
                pma.read(EndpointType::Device(DeviceEndpoint::Keyboard), 2),
                pma.read(EndpointType::Device(DeviceEndpoint::Keyboard), 4),
                pma.read(EndpointType::Device(DeviceEndpoint::Keyboard), 6),
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

        pma.init(&SUPPORTED_ENDPOINTS);

        assert_eq!(pma.write(EndpointType::Control, &[1, 2, 3, 4]), 4);
        assert_eq!(
            pma.write(EndpointType::Device(DeviceEndpoint::System), &[5, 6, 7, 8]),
            4
        );
        assert_eq!(
            pma.write(
                EndpointType::Device(DeviceEndpoint::Keyboard),
                &[9, 10, 11, 12],
            ),
            4
        );

        assert_eq!(
            [
                1 | (2 << 8),
                3 | (4 << 8),
                5 | (6 << 8),
                7 | (8 << 8),
                9 | (10 << 8),
                11 | (12 << 8)
            ],
            [
                sandbox[(CONTROL_IN_PMA_ADDRESS >> 1) as usize],
                sandbox[(CONTROL_IN_PMA_ADDRESS >> 1) as usize + 1],
                sandbox[(DEVICE_SYSTEM_IN_PMA_ADDRESS >> 1) as usize],
                sandbox[(DEVICE_SYSTEM_IN_PMA_ADDRESS >> 1) as usize + 1],
                sandbox[(DEVICE_KEYBOARD_IN_PMA_ADDRESS >> 1) as usize],
                sandbox[(DEVICE_KEYBOARD_IN_PMA_ADDRESS >> 1) as usize + 1]
            ]
        );
    }

    #[test]
    fn correctly_writes_even_data() {
        let sandbox: [u16; 256] = [0; 256];
        let sandbox_address: usize = &sandbox as *const _ as usize;
        let pma = PacketMemoryArea {
            base_address: sandbox_address,
        };

        pma.init(&SUPPORTED_ENDPOINTS);

        assert_eq!(pma.write(EndpointType::Control, &[]), 0);
        assert_eq!(pma.write(EndpointType::Control, &[1, 2, 3]), 3);
        assert_eq!(
            pma.write(EndpointType::Device(DeviceEndpoint::System), &[4, 5, 6]),
            3
        );
        assert_eq!(
            pma.write(EndpointType::Device(DeviceEndpoint::Keyboard), &[7, 8, 9],),
            3
        );

        assert_eq!(
            [1 | (2 << 8), 3, 4 | (5 << 8), 6, 7 | (8 << 8), 9],
            [
                sandbox[(CONTROL_IN_PMA_ADDRESS >> 1) as usize],
                sandbox[(CONTROL_IN_PMA_ADDRESS >> 1) as usize + 1],
                sandbox[(DEVICE_SYSTEM_IN_PMA_ADDRESS >> 1) as usize],
                sandbox[(DEVICE_SYSTEM_IN_PMA_ADDRESS >> 1) as usize + 1],
                sandbox[(DEVICE_KEYBOARD_IN_PMA_ADDRESS >> 1) as usize],
                sandbox[(DEVICE_KEYBOARD_IN_PMA_ADDRESS >> 1) as usize + 1]
            ]
        );
    }
}
