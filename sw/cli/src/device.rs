use std::fmt;

pub const KRONEUM_VID: u16 = 0xffff;
pub const KRONEUM_PID: u16 = 0xffff;
pub const REPORT_SIZE: usize = 6;

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

pub trait Device {
    fn get_identifier(&self) -> DeviceIdentifier;
    fn get_manufacturer(&self) -> Result<String, String>;
    fn send_data(&self, data: &[u8]) -> Result<(), String>;
    fn read_data(&self) -> Result<(usize, [u8; REPORT_SIZE]), String>;
}
