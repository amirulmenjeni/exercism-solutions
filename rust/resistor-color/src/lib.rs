use core::fmt;

use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, Clone, Copy, IntEnum, IntoEnumIterator, PartialEq, PartialOrd)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value as u8) {
        Ok(v) => v.to_string(),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut v = ResistorColor::into_enum_iter()
        .collect::<Vec<ResistorColor>>();
    v.sort_by(|a, b| {
        a.partial_cmp(b).unwrap()
    });
    v
}
