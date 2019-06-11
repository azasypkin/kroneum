/// Indicates direction of the USB request.
#[derive(Debug, PartialOrd, PartialEq)]
pub enum RequestDirection {
    /// Host is requesting device.
    HostToDevice,
    /// Device is querying host.
    DeviceToHost,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum RequestKind {
    Standard,
    Class,
    Vendor,
    Reserved,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum RequestRecipient {
    Device,
    Interface,
    Endpoint,
    Other,
    Reserved,
}

/// Description of the request.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Request {
    GetStatus = 0x00,
    ClearFeature = 0x01,
    Two = 0x02,
    SetFeature = 0x03,
    SetAddress = 0x05,
    GetDescriptor = 0x06,
    SetDescriptor = 0x07,
    GetConfiguration = 0x08,
    SetConfiguration = 0x09,
    GetInterface = 0x0A,
    SetInterface = 0x0B,
    SynchFrame = 0x0C,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct SetupPacket {
    pub request: Request,
    pub dir: RequestDirection,
    pub kind: RequestKind,
    pub recipient: RequestRecipient,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

impl From<(u16, u16, u16, u16)> for SetupPacket {
    #[inline]
    fn from((request_header, value, index, data_length): (u16, u16, u16, u16)) -> Self {
        let request_type = (request_header & 0x00ff) as u8;
        let request = ((request_header & 0xff00) >> 8) as u8;

        SetupPacket {
            request: match request {
                0x00 => Request::GetStatus,
                0x01 => Request::ClearFeature,
                0x02 => Request::Two,
                0x03 => Request::SetFeature,
                0x05 => Request::SetAddress,
                0x06 => Request::GetDescriptor,
                0x07 => Request::SetDescriptor,
                0x08 => Request::GetConfiguration,
                0x09 => Request::SetConfiguration,
                0x0A => Request::GetInterface,
                0x0B => Request::SetInterface,
                0x0C => Request::SynchFrame,
                _ => unreachable!(),
            },
            // Bit 7
            dir: match request_type & 0x80 {
                0x00 => RequestDirection::HostToDevice,
                0x80 => RequestDirection::DeviceToHost,
                _ => unreachable!(),
            },
            // Bits 6:5
            kind: match request_type & 0x60 {
                0x00 => RequestKind::Standard,
                0x20 => RequestKind::Class,
                0x40 => RequestKind::Vendor,
                0x60 => RequestKind::Reserved,
                _ => unreachable!(),
            },
            // Bits 4:0
            recipient: match request_type & 0x1f {
                0x00 => RequestRecipient::Device,
                0x01 => RequestRecipient::Interface,
                0x02 => RequestRecipient::Endpoint,
                0x03 => RequestRecipient::Other,
                0x04..=0x1f => RequestRecipient::Reserved,
                _ => unreachable!(),
            },
            value,
            index,
            length: data_length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_to_device_request() {
        assert_eq!(
            SetupPacket::from((0x0000, 1, 2, 3)),
            SetupPacket {
                request: Request::GetStatus,
                dir: RequestDirection::HostToDevice,
                kind: RequestKind::Standard,
                recipient: RequestRecipient::Device,
                value: 1,
                index: 2,
                length: 3,
            }
        );
    }

    #[test]
    fn device_to_host_request() {
        assert_eq!(
            SetupPacket::from((0x0082, 1, 2, 3)),
            SetupPacket {
                request: Request::GetStatus,
                dir: RequestDirection::DeviceToHost,
                kind: RequestKind::Standard,
                recipient: RequestRecipient::Endpoint,
                value: 1,
                index: 2,
                length: 3,
            }
        );
    }

    #[test]
    fn get_descriptor_request() {
        assert_eq!(
            SetupPacket::from((0x0600, 1, 2, 3)),
            SetupPacket {
                request: Request::GetDescriptor,
                dir: RequestDirection::HostToDevice,
                kind: RequestKind::Standard,
                recipient: RequestRecipient::Device,
                value: 1,
                index: 2,
                length: 3,
            }
        );
    }
}
