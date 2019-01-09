pub enum RequestDirection {
    HostToDevice,
    DeviceToHost,
}

pub enum RequestKind {
    Standard,
    Class,
    Vendor,
    Reserved,
}

pub enum RequestRecipient {
    Device,
    Interface,
    Endpoint,
    Other,
    Reserved,
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
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
                _ => panic!("Unreachable"),
            },
            // Bit 7
            dir: match request_type & 0x80 {
                0x00 => RequestDirection::HostToDevice,
                0x80 => RequestDirection::DeviceToHost,
                _ => panic!("Unreachable"),
            },
            // Bits 6:5
            kind: match request_type & 0x60 {
                0x00 => RequestKind::Standard,
                0x20 => RequestKind::Class,
                0x40 => RequestKind::Vendor,
                0x60 => RequestKind::Reserved,
                _ => panic!("Unreachable"),
            },
            // Bits 4:0
            recipient: match request_type & 0x1f {
                0x00 => RequestRecipient::Device,
                0x01 => RequestRecipient::Interface,
                0x02 => RequestRecipient::Endpoint,
                0x03 => RequestRecipient::Other,
                0x04...0x1f => RequestRecipient::Reserved,
                _ => panic!("Unreachable"),
            },
            value,
            index,
            length: data_length,
        }
    }
}