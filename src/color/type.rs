use std::fmt::{self, Display};

use crate::color::{
    blod::BLOD,
    color::{BLACK, BLUE, CYAN, DEFAULT, GREEN, MAGENTA, RED, WHITE, YELLOW},
};

use super::color::{BG_BLACK, BG_BLUE, BG_CYAN, BG_GREEN, BG_MAGENTA, BG_RED, BG_WHITE, BG_YELLOW};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ColorType {
    Text,
    Background,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
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
    fn get_str(&self, is_text_color: ColorType) -> &'static str;
    fn blod(&self) -> String;
}

impl Default for Color {
    fn default() -> Self {
        Color::Default
    }
}

impl ColorDisplay for Color {
    fn get_str(&self, is_text_color: ColorType) -> &'static str {
        match is_text_color {
            ColorType::Text => match self {
                Color::Red => RED,
                Color::Green => GREEN,
                Color::Blue => BLUE,
                Color::Yellow => YELLOW,
                Color::Black => BLACK,
                Color::White => WHITE,
                Color::Default => DEFAULT,
                Color::Magenta => MAGENTA,
                Color::Cyan => CYAN,
            },
            _ => match self {
                Color::Red => BG_RED,
                Color::Green => BG_GREEN,
                Color::Blue => BG_BLUE,
                Color::Yellow => BG_YELLOW,
                Color::Black => BG_BLACK,
                Color::White => BG_WHITE,
                Color::Default => DEFAULT,
                Color::Magenta => BG_MAGENTA,
                Color::Cyan => BG_CYAN,
            },
        }
    }

    fn blod(&self) -> String {
        format!("{}{}", self.to_string(), BLOD)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_str(ColorType::Text))
    }
}
