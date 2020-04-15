use array::Array;
use core::convert::TryFrom;
use usb::usb_error::USBError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MediaKey {
    VolumeUp = 0x01,
    VolumeDown = 0x02,
    Mute = 0x04,
    NextTrack = 0x08,
    PreviousTrack = 0x10,
    PlayPause = 0x20,
    Stop = 0x40,
}

impl TryFrom<u8> for MediaKey {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x01 => MediaKey::VolumeUp,
            0x02 => MediaKey::VolumeDown,
            0x04 => MediaKey::Mute,
            0x08 => MediaKey::NextTrack,
            0x10 => MediaKey::PreviousTrack,
            0x20 => MediaKey::PlayPause,
            0x40 => MediaKey::Stop,
            _ => return Err(()),
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KeyboardCommand {
    Key(u8, u8, u8),
    Media(MediaKey, u8),
}

impl TryFrom<Array<u8>> for KeyboardCommand {
    type Error = USBError;

    fn try_from(mut value: Array<u8>) -> Result<Self, Self::Error> {
        match (value.shift(), value.len()) {
            (Some(0x1), 3) => Ok(KeyboardCommand::Key(value[0], value[1], value[2])),
            (Some(0x2), 2) => {
                if let Ok(media_key) = MediaKey::try_from(value[0]) {
                    Ok(KeyboardCommand::Media(media_key, value[1]))
                } else {
                    Err(USBError::InvalidCommand)
                }
            }
            _ => Err(USBError::InvalidCommand),
        }
    }
}

impl TryFrom<&[u8]> for KeyboardCommand {
    type Error = USBError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Array::from(slice))
    }
}

impl From<KeyboardCommand> for Array<u8> {
    fn from(packet: KeyboardCommand) -> Self {
        match packet {
            KeyboardCommand::Key(modifiers, key_code, delay) => {
                [1, modifiers, key_code, delay].as_ref().into()
            }
            KeyboardCommand::Media(media_key, delay) => [2, media_key as u8, delay].as_ref().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_command() {
        assert_eq!(
            KeyboardCommand::try_from([1, 1, 1, 1].as_ref()),
            Ok(KeyboardCommand::Key(1, 1, 1))
        );

        assert_eq!(
            KeyboardCommand::try_from([1, 3, 4, 5].as_ref()),
            Ok(KeyboardCommand::Key(3, 4, 5))
        );

        assert_eq!(
            Array::from(KeyboardCommand::Key(5, 6, 7)).as_ref(),
            [1, 5, 6, 7]
        );
    }

    #[test]
    fn media_key_command() {
        assert_eq!(
            KeyboardCommand::try_from([2, 1, 1].as_ref()),
            Ok(KeyboardCommand::Media(MediaKey::VolumeUp, 1))
        );
        assert_eq!(
            Array::from(KeyboardCommand::Media(MediaKey::VolumeUp, 2)).as_ref(),
            [2, 1, 2]
        );

        assert_eq!(
            KeyboardCommand::try_from([2, 2, 1].as_ref()),
            Ok(KeyboardCommand::Media(MediaKey::VolumeDown, 1))
        );
        assert_eq!(
            Array::from(KeyboardCommand::Media(MediaKey::VolumeDown, 2)).as_ref(),
            [2, 2, 2]
        );

        assert_eq!(
            KeyboardCommand::try_from([2, 4, 1].as_ref()),
            Ok(KeyboardCommand::Media(MediaKey::Mute, 1))
        );
        assert_eq!(
            Array::from(KeyboardCommand::Media(MediaKey::Mute, 2)).as_ref(),
            [2, 4, 2]
        );

        assert_eq!(
            KeyboardCommand::try_from([2, 8, 1].as_ref()),
            Ok(KeyboardCommand::Media(MediaKey::NextTrack, 1))
        );
        assert_eq!(
            Array::from(KeyboardCommand::Media(MediaKey::NextTrack, 2)).as_ref(),
            [2, 8, 2]
        );

        assert_eq!(
            KeyboardCommand::try_from([2, 16, 1].as_ref()),
            Ok(KeyboardCommand::Media(MediaKey::PreviousTrack, 1))
        );
        assert_eq!(
            Array::from(KeyboardCommand::Media(MediaKey::PreviousTrack, 2)).as_ref(),
            [2, 16, 2]
        );

        assert_eq!(
            KeyboardCommand::try_from([2, 32, 1].as_ref()),
            Ok(KeyboardCommand::Media(MediaKey::PlayPause, 1))
        );
        assert_eq!(
            Array::from(KeyboardCommand::Media(MediaKey::PlayPause, 2)).as_ref(),
            [2, 32, 2]
        );

        assert_eq!(
            KeyboardCommand::try_from([2, 64, 1].as_ref()),
            Ok(KeyboardCommand::Media(MediaKey::Stop, 1))
        );
        assert_eq!(
            Array::from(KeyboardCommand::Media(MediaKey::Stop, 2)).as_ref(),
            [2, 64, 2]
        );
    }

    #[test]
    fn invalid_command() {
        assert_eq!(
            KeyboardCommand::try_from([0].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([0, 1].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([0, 1, 2].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([1].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([1, 2].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([2].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([2, 1].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([2, 3, 1].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([2, 5, 1].as_ref()),
            Err(USBError::InvalidCommand)
        );
        assert_eq!(
            KeyboardCommand::try_from([2, 6, 4].as_ref()),
            Err(USBError::InvalidCommand)
        );

        assert_eq!(
            KeyboardCommand::try_from([2, 10, 5].as_ref()),
            Err(USBError::InvalidCommand)
        );
    }
}
