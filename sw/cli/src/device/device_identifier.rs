use std::fmt;

/// Describes main parameters of the Kroneum device.
pub struct DeviceIdentifier {
    pub bus: u8,
    pub address: u8,
    pub vendor_id: u16,
    pub product_id: u16,
}

impl fmt::Display for DeviceIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bus: {:03}, Addr: {:03}, VID: {:04x}, PID: {:04x})",
            self.bus, self.address, self.vendor_id, self.product_id
        )
    }
}
