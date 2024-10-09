#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ColorId {
    White = 0,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}
impl ColorId {
    pub fn to_number(self) -> u16 {
        1 << self as u8
    }

    pub fn from_number_overflow(num: u32) -> ColorId {
        let num = num % 16;
        let num = num as u8;
        ColorId::from_number_or_panic(num)
    }

    fn from_number_or_panic(num: u8) -> ColorId {
        match num {
            0 => ColorId::White,
            1 => ColorId::Orange,
            2 => ColorId::Magenta,
            3 => ColorId::LightBlue,
            4 => ColorId::Yellow,
            5 => ColorId::Lime,
            6 => ColorId::Pink,
            7 => ColorId::Gray,
            8 => ColorId::LightGray,
            9 => ColorId::Cyan,
            10 => ColorId::Purple,
            11 => ColorId::Blue,
            12 => ColorId::Brown,
            13 => ColorId::Green,
            14 => ColorId::Red,
            15 => ColorId::Black,
            _ => panic!(),
        }
    }
}
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    PosX,
    PosY,
    NegX,
    NegY,
}
impl Direction {
    pub fn to_dxdy(self) -> (isize, isize) {
        match self {
            Direction::PosX => (1, 0),
            Direction::PosY => (0, 1),
            Direction::NegX => (-1, 0),
            Direction::NegY => (0, -1),
        }
    }
}

/// as if a pixel, a basic display part of computer craft monitor
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsIfPixel {
    text: char,
    pub background_color: ColorId,
    pub text_color: ColorId,
}
impl AsIfPixel {
    /// returns `None` if `text` is not within the ASCII range
    pub const fn new(text: char, background_color: ColorId, text_color: ColorId) -> Option<Self> {
        if !text.is_ascii() {
            None
        } else {
            Some(AsIfPixel {
                text,
                background_color,
                text_color,
            })
        }
    }
    pub const fn colored_whitespace(color: ColorId) -> Self {
        AsIfPixel {
            text: ' ',
            background_color: color,
            text_color: color,
        }
    }
    pub fn text(&self) -> char {
        self.text
    }
}
