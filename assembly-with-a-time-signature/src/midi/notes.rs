use crate::vm::Note;
use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy)]
pub struct Channel(pub(super) u8);

#[derive(Debug, Clone, Copy)]
pub struct Velocity(pub(super) u8);

pub struct U14(pub(super) u8, pub(super) u8);

#[repr(i64)]
#[derive(Debug, Clone, Copy, TryFromPrimitive)]
pub enum Tone {
    A0 = 21,
    AS0,
    B0,
    C1,
    CS1,
    D1,
    DS1,
    E1,
    F1,
    FS1,
    G1,
    GS1,
    A1,
    AS1,
    B1,
    C2,
    CS2,
    D2,
    DS2,
    E2,
    F2,
    FS2,
    G2,
    GS2,
    A2,
    AS2,
    B2,
    C3,
    CS3,
    D3,
    DS3,
    E3,
    F3,
    FS3,
    G3,
    GS3,
    A3,
    AS3,
    B3,
    C4,
    CS4,
    D4,
    DS4,
    E4,
    F4,
    FS4,
    G4,
    GS4,
    A4,
    AS4,
    B4,
    C5,
    CS5,
    D5,
    DS5,
    E5,
    F5,
    FS5,
    G5,
    GS5,
    A5,
    AS5,
    B5,
    C6,
    CS6,
    D6,
    DS6,
    E6,
    F6,
    FS6,
    G6,
    GS6,
    A6,
    AS6,
    B6,
    C7,
    CS7,
    D7,
    DS7,
    E7,
    F7,
    FS7,
    G7,
    GS7,
    A7,
    AS7,
    B7,
    C8,
    CS8,
    D8,
    DS8,
    E8,
    F8,
    FS8,
    G8,
    GS8,
    A8,
    AS8,
    B8,
    C9,
    CS9,
    D9,
    DS9,
    E9,
    F9,
    FS9,
    G9,
}

impl From<Note> for Tone {
    fn from(n: Note) -> Self {
        match n {
            Note::A => Tone::A4,
            Note::B => Tone::B4,
            Note::C => Tone::C4,
            Note::D => Tone::D4,
            Note::E => Tone::E4,
            Note::F => Tone::F4,
            Note::G => Tone::G4,
        }
    }
}

impl Tone {
    pub fn add(self, m: i64) -> Self {
        let s = self as i64;
        let new = s + m;
        if new > Self::G9 as i64 {
            Self::G9
        } else if new < Self::A0 as i64 {
            Self::A0
        } else {
            Self::try_from(new).unwrap()
        }
    }
}

impl From<u16> for U14 {
    fn from(n: u16) -> Self {
        let low = (n & 0x7F) as u8;
        let high = ((n >> 7) & 0x7F) as u8;
        U14(high, low)
    }
}

impl From<u8> for U14 {
    fn from(n: u8) -> Self {
        let low = n & 0x7F;
        let high = (n >> 7) & 0x7F;
        U14(high, low)
    }
}

impl From<i32> for U14 {
    fn from(n: i32) -> Self {
        let n = n as u16;
        U14::from(n)
    }
}

macro_rules! midi_from {
    ($type: ty) => {
        impl From<u8> for $type {
            fn from(value: u8) -> Self {
                Self(value)
            }
        }
    };
}

midi_from!(Channel);
midi_from!(Velocity);

impl From<u8> for Tone {
    fn from(value: u8) -> Self {
        match value {
            21 => Tone::A0,
            22 => Tone::AS0,
            23 => Tone::B0,
            24 => Tone::C1,
            25 => Tone::CS1,
            26 => Tone::D1,
            27 => Tone::DS1,
            28 => Tone::E1,
            29 => Tone::F1,
            30 => Tone::FS1,
            31 => Tone::G1,
            32 => Tone::GS1,
            33 => Tone::A1,
            34 => Tone::AS1,
            35 => Tone::B1,
            36 => Tone::C2,
            37 => Tone::CS2,
            38 => Tone::D2,
            39 => Tone::DS2,
            40 => Tone::E2,
            41 => Tone::F2,
            42 => Tone::FS2,
            43 => Tone::G2,
            44 => Tone::GS2,
            45 => Tone::A2,
            46 => Tone::AS2,
            47 => Tone::B2,
            48 => Tone::C3,
            49 => Tone::CS3,
            50 => Tone::D3,
            51 => Tone::DS3,
            52 => Tone::E3,
            53 => Tone::F3,
            54 => Tone::FS3,
            55 => Tone::G3,
            56 => Tone::GS3,
            57 => Tone::A3,
            58 => Tone::AS3,
            59 => Tone::B3,
            60 => Tone::C4,
            61 => Tone::CS4,
            62 => Tone::D4,
            63 => Tone::DS4,
            64 => Tone::E4,
            65 => Tone::F4,
            66 => Tone::FS4,
            67 => Tone::G4,
            68 => Tone::GS4,
            69 => Tone::A4,
            70 => Tone::AS4,
            71 => Tone::B4,
            72 => Tone::C5,
            73 => Tone::CS5,
            74 => Tone::D5,
            75 => Tone::DS5,
            76 => Tone::E5,
            77 => Tone::F5,
            78 => Tone::FS5,
            79 => Tone::G5,
            80 => Tone::GS5,
            81 => Tone::A5,
            82 => Tone::AS5,
            83 => Tone::B5,
            84 => Tone::C6,
            85 => Tone::CS6,
            86 => Tone::D6,
            87 => Tone::DS6,
            88 => Tone::E6,
            89 => Tone::F6,
            90 => Tone::FS6,
            91 => Tone::G6,
            92 => Tone::GS6,
            93 => Tone::A6,
            94 => Tone::AS6,
            95 => Tone::B6,
            96 => Tone::C7,
            97 => Tone::CS7,
            98 => Tone::D7,
            99 => Tone::DS7,
            100 => Tone::E7,
            101 => Tone::F7,
            102 => Tone::FS7,
            103 => Tone::G7,
            104 => Tone::GS7,
            105 => Tone::A7,
            106 => Tone::AS7,
            107 => Tone::B7,
            108 => Tone::C8,
            109 => Tone::CS8,
            110 => Tone::D8,
            111 => Tone::DS8,
            112 => Tone::E8,
            113 => Tone::F8,
            114 => Tone::FS8,
            115 => Tone::G8,
            116 => Tone::GS8,
            117 => Tone::A8,
            118 => Tone::AS8,
            119 => Tone::B8,
            120 => Tone::C9,
            121 => Tone::CS9,
            122 => Tone::D9,
            123 => Tone::DS9,
            124 => Tone::E9,
            125 => Tone::F9,
            126 => Tone::FS9,
            127 => Tone::G9,
            _ => panic!("Invalid note value"),
        }
    }
}
