use std::{fmt, time::Duration};

pub const KRONEUM_VID: u16 = 0xffff;
pub const KRONEUM_PID: u16 = 0xffff;
pub const REPORT_SIZE: usize = 6;
const MAX_ALARM_SECONDS: u64 = 3600 * 24;

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
    fn write(&self, data: &[u8]) -> Result<(), String>;
    fn read(&self) -> Result<(usize, [u8; REPORT_SIZE]), String>;

    fn beep(&self, beeps_n: u8) -> Result<(), String> {
        self.write([0, 0, beeps_n].as_ref())
    }

    fn get_alarm(&self) -> Result<Duration, String> {
        // Send `GetAlarm` report and then read data device sent in response report.
        self.write([2].as_ref())
            .and_then(|_| self.read())
            .map(|(_, data)| {
                Duration::from_secs(
                    u64::from(data[0]) * 3600 + u64::from(data[1]) * 60 + u64::from(data[2]),
                )
            })
    }

    fn set_alarm(&self, duration: Duration) -> Result<(), String> {
        let duration_sec = duration.as_secs();
        if duration_sec >= MAX_ALARM_SECONDS {
            return Err("Alarm is limited to 23h 59m 59s".to_string());
        }

        let hours = duration_sec / 3600;
        let minutes = (duration_sec - 3600 * hours) / 60;
        let seconds = duration_sec - 3600 * hours - 60 * minutes;

        self.write([1, 0, hours as u8, minutes as u8, seconds as u8].as_ref())
    }
}

pub trait DeviceContext<'a, D: Device, C = Self> {
    fn create() -> Result<C, String>;
    fn open(&'a self) -> Result<D, String>;
    fn close(&'a self, device: D) -> Result<(), String>;
}
