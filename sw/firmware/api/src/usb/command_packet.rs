use crate::time::Time;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum CommandPacket {
    Unknown,
    Beep(u8),
    GetAlarm,
    Reset,
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
            2 => CommandPacket::GetAlarm,
            3 => CommandPacket::Reset,
            _ => CommandPacket::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beep_command() {
        assert_eq!(CommandPacket::from((0, 1, 0)), CommandPacket::Beep(1));
        assert_eq!(CommandPacket::from((0, 15, 0)), CommandPacket::Beep(15));
    }

    #[test]
    fn set_alarm_command() {
        assert_eq!(
            CommandPacket::from((1, 0x2112, 17)),
            CommandPacket::SetAlarm(Time {
                hours: 18,
                minutes: 33,
                seconds: 17,
            })
        );

        assert_eq!(
            CommandPacket::from((1, 0x1221, 1)),
            CommandPacket::SetAlarm(Time {
                hours: 33,
                minutes: 18,
                seconds: 1,
            })
        );
    }

    #[test]
    fn get_alarm_command() {
        assert_eq!(CommandPacket::from((2, 0, 0)), CommandPacket::GetAlarm);
        assert_eq!(CommandPacket::from((2, 11, 22)), CommandPacket::GetAlarm);
    }

    #[test]
    fn reset_command() {
        assert_eq!(CommandPacket::from((3, 0, 0)), CommandPacket::Reset);
    }

    #[test]
    fn unknown_command() {
        assert_eq!(CommandPacket::from((4, 0, 0)), CommandPacket::Unknown);
        assert_eq!(CommandPacket::from((10, 11, 22)), CommandPacket::Unknown);
    }
}
