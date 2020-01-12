/// Note durations based on `200 b/m` (beats per minute), see https://msu.edu/course/asc/232/song_project/dectalk_pages/note_to_%20ms.html.
pub const NOTE_1_8_DURATION: u8 = 50;
pub const NOTE_1_4_DURATION: u8 = NOTE_1_8_DURATION * 2;
pub const NOTE_1_2_DURATION: u8 = NOTE_1_4_DURATION * 2;

#[derive(Debug, Copy, Clone)]
pub enum Note {
    C0 = 0x10,
    CSharp0 = 0x20,
    D0 = 0x30,
    DSharp0 = 0x40,
    E0 = 0x50,
    F0 = 0x60,
    FSharp0 = 0x70,
    G0 = 0x80,
    GSharp0 = 0x90,
    A0 = 0xA0,
    ASharp0 = 0xB0,
    B0 = 0xC0,

    C1 = 0x11,
    CSharp1 = 0x21,
    D1 = 0x31,
    DSharp1 = 0x41,
    E1 = 0x51,
    F1 = 0x61,
    FSharp1 = 0x71,
    G1 = 0x81,
    GSharp1 = 0x91,
    A1 = 0xA1,
    ASharp1 = 0xB1,
    B1 = 0xC1,

    C2 = 0x12,
    CSharp2 = 0x22,
    D2 = 0x32,
    DSharp2 = 0x42,
    E2 = 0x52,
    F2 = 0x62,
    FSharp2 = 0x72,
    G2 = 0x82,
    GSharp2 = 0x92,
    A2 = 0xA2,
    ASharp2 = 0xB2,
    B2 = 0xC2,

    C3 = 0x13,
    CSharp3 = 0x23,
    D3 = 0x33,
    DSharp3 = 0x43,
    E3 = 0x53,
    F3 = 0x63,
    FSharp3 = 0x73,
    G3 = 0x83,
    GSharp3 = 0x93,
    A3 = 0xA3,
    ASharp3 = 0xB3,
    B3 = 0xC3,

    C4 = 0x14,
    CSharp4 = 0x24,
    D4 = 0x34,
    DSharp4 = 0x44,
    E4 = 0x54,
    F4 = 0x64,
    FSharp4 = 0x74,
    G4 = 0x84,
    GSharp4 = 0x94,
    A4 = 0xA4,
    ASharp4 = 0xB4,
    B4 = 0xC4,

    C5 = 0x15,
    CSharp5 = 0x25,
    D5 = 0x35,
    DSharp5 = 0x45,
    E5 = 0x55,
    F5 = 0x65,
    FSharp5 = 0x75,
    G5 = 0x85,
    GSharp5 = 0x95,
    A5 = 0xA5,
    ASharp5 = 0xB5,
    B5 = 0xC5,

    C6 = 0x16,
    CSharp6 = 0x26,
    D6 = 0x36,
    DSharp6 = 0x46,
    E6 = 0x56,
    F6 = 0x66,
    FSharp6 = 0x76,
    G6 = 0x86,
    GSharp6 = 0x96,
    A6 = 0xA6,
    ASharp6 = 0xB6,
    B6 = 0xC6,

    C7 = 0x17,
    CSharp7 = 0x27,
    D7 = 0x37,
    DSharp7 = 0x47,
    E7 = 0x57,
    F7 = 0x67,
    FSharp7 = 0x77,
    G7 = 0x87,
    GSharp7 = 0x97,
    A7 = 0xA7,
    ASharp7 = 0xB7,
    B7 = 0xC7,

    C8 = 0x18,
    CSharp8 = 0x28,
    D8 = 0x38,
    DSharp8 = 0x48,
    E8 = 0x58,
    F8 = 0x68,
    FSharp8 = 0x78,
    G8 = 0x88,
    GSharp8 = 0x98,
    A8 = 0xA8,
    ASharp8 = 0xB8,
    B8 = 0xC8,

    Silence = 0x00,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn silence_note_evaluates_to_zero() {
        assert_eq!(Note::Silence as u8, 0);
    }

    #[test]
    fn durations_are_properly_calculated() {
        assert_eq!(NOTE_1_8_DURATION, 50);
        assert_eq!(NOTE_1_4_DURATION, 100);
        assert_eq!(NOTE_1_2_DURATION, 200);
    }
}
