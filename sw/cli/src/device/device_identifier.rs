use serde_derive::{Deserialize, Serialize};
use std::fmt;

/// Describes main parameters of the Kroneum device.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub bus: u8,
    pub address: u8,
    #[serde(rename(serialize = "vendorID"))]
    pub vendor_id: u16,
    #[serde(rename(serialize = "productID"))]
    pub product_id: u16,
    pub manufacturer: String,
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Bus: {:03}, Addr: {:03}, VID: {:04x}, PID: {:04x}, Manufacturer: {}",
            self.bus, self.address, self.vendor_id, self.product_id, self.manufacturer
        )
    }
}
