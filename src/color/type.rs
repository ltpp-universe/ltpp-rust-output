use std::fmt::{self, Display};

use crate::color::{
    blod::BLOD,
    color::{BLACK, BLUE, CYAN, DEFAULT, GREEN, MAGENTA, RED, WHITE, YELLOW},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TextColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BgColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub(crate) trait ColorDisplay {
    fn get_str(&self) -> &'static str;
    fn blod(&self) -> String;
}

impl Default for TextColor {
    fn default() -> Self {
        TextColor::White
    }
}

impl Default for BgColor {
    fn default() -> Self {
        BgColor::White
    }
}

impl ColorDisplay for TextColor {
    fn get_str(&self) -> &'static str {
        match self {
            TextColor::Red => RED,
            TextColor::Green => GREEN,
            TextColor::Blue => BLUE,
            TextColor::Yellow => YELLOW,
            TextColor::Black => BLACK,
            TextColor::White => WHITE,
            TextColor::Default => DEFAULT,
            TextColor::Magenta => MAGENTA,
            TextColor::Cyan => CYAN,
        }
    }

    fn blod(&self) -> String {
        format!("{}{}", self.to_string(), BLOD)
    }
}

impl ColorDisplay for BgColor {
    fn get_str(&self) -> &'static str {
        match self {
            BgColor::Red => RED,
            BgColor::Green => GREEN,
            BgColor::Blue => BLUE,
            BgColor::Yellow => YELLOW,
            BgColor::Black => BLACK,
            BgColor::White => WHITE,
            BgColor::Default => DEFAULT,
            BgColor::Magenta => MAGENTA,
            BgColor::Cyan => CYAN,
        }
    }

    fn blod(&self) -> String {
        format!("{}{}", self.to_string(), BLOD)
    }
}

impl Display for TextColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_str())
    }
}

impl Display for BgColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_str())
    }
}
