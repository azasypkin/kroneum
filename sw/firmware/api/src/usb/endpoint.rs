use super::usb_error::USBError;
use core::convert::TryFrom;

/// Describes supported USB endpoints:
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum EndpointType {
    /// Reserved control endpoint.
    Control,
    /// Custom device specific endpoint.
    Device(DeviceEndpoint),
}

/// Types of device specific endpoints.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DeviceEndpoint {
    /// Endpoint reserved for Kroneum management via custom HID reports.
    System,
    /// Endpoint used to emulate HID keyboard.
    Keyboard,
}

impl EndpointType {
    /// Checks whether specific endpoint type is the Control one.
    pub fn is_control(&self) -> bool {
        matches!(self, EndpointType::Control)
    }
}

impl TryFrom<u8> for EndpointType {
    type Error = USBError;

    /// Tries to detect endpoint type by its numeric address/ID (only for 0..2).
    fn try_from(identifier: u8) -> Result<Self, Self::Error> {
        match identifier {
            0 => Ok(EndpointType::Control),
            1 => Ok(EndpointType::Device(DeviceEndpoint::System)),
            2 => Ok(EndpointType::Device(DeviceEndpoint::Keyboard)),
            _ => Err(USBError::InvalidEndpoint),
        }
    }
}

impl Into<u8> for EndpointType {
    /// Converts EndpointType into endpoint ID used to work with USB registers.
    fn into(self) -> u8 {
        match self {
            EndpointType::Control => 0,
            EndpointType::Device(DeviceEndpoint::System) => 1,
            EndpointType::Device(DeviceEndpoint::Keyboard) => 2,
        }
    }
}

/// Determines the direction this endpoint is currently used for.
#[derive(Copy, Clone)]
pub enum EndpointDirection {
    /// Endpoint is used to receive data FROM the HOST.
    Receive,
    /// Endpoint is used to transmit data TO the HOST.
    Transmit,
}

/// Defines possible _physical_ states of the endpoint (either control or device).
#[derive(Copy, Clone)]
pub enum EndpointStatus {
    /// Endpoint is disabled.
    Disabled = 0x0,
    /// Endpoint is stalled/halted.
    Stall = 0x1,
    /// Endpoint is working, but temporarily has no data to send.
    Nak = 0x2,
    /// Endpoint is in a valid state and ready to receive/transmit data.
    Valid = 0x3,
}

/// Defines possible _logical_ statuses for the control endpoint.
#[derive(Copy, Clone)]
pub(crate) enum ControlEndpointStatus {
    /// Control endpoint is idling.
    Idle,
    /// Control endpoint received setup packet of the specified length.
    Setup(u16),
    /// Control endpoint is sending data TO the HOST.
    DataIn,
    /// Control endpoint is receiving data FROM the HOST.
    DataOut,
    /// Control endpoint is sending status TO the HOST.
    StatusIn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_endpoint() {
        assert_eq!(EndpointType::try_from(0), Ok(EndpointType::Control));
        assert_eq!(Into::<u8>::into(EndpointType::Control), 0);
    }

    #[test]
    fn device_endpoint() {
        assert_eq!(
            EndpointType::try_from(1),
            Ok(EndpointType::Device(DeviceEndpoint::System))
        );
        assert_eq!(
            Into::<u8>::into(EndpointType::Device(DeviceEndpoint::System)),
            1
        );

        assert_eq!(
            EndpointType::try_from(2),
            Ok(EndpointType::Device(DeviceEndpoint::Keyboard))
        );
        assert_eq!(
            Into::<u8>::into(EndpointType::Device(DeviceEndpoint::Keyboard)),
            2
        );
    }

    #[test]
    fn invalid_endpoint() {
        for id in 3..=8 {
            assert_eq!(EndpointType::try_from(id), Err(USBError::InvalidEndpoint));
        }
    }

    #[test]
    fn endpoint_status() {
        assert_eq!(EndpointStatus::Disabled as u8, 0x0);
        assert_eq!(EndpointStatus::Stall as u8, 0x1);
        assert_eq!(EndpointStatus::Nak as u8, 0x2);
        assert_eq!(EndpointStatus::Valid as u8, 0x3);
    }
}
