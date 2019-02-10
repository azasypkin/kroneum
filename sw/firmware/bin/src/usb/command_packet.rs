use crate::rtc::Time;

pub enum CommandPacket {
    Unknown,
    Beep(u8),
    SetAlarm(Time),
}

impl From<(u16, u16, u16)> for CommandPacket {
    fn from((header, data_1, data_2): (u16, u16, u16)) -> Self {
        let command_type_byte = header & 0xff;
        match command_type_byte {
            0 => CommandPacket::Beep((data_1 & 0xff) as u8),
            1 => CommandPacket::SetAlarm(Time {
                hours: (data_1 & 0xff) as u8,
                minutes: ((data_1 & 0xff00) >> 8) as u8,
                seconds: (data_2 & 0xff) as u8,
            }),
            _ => CommandPacket::Unknown,
        }
    }
}
