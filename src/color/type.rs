use crate::color::{
    blod::BLOD,
    color::{
        BG_BLACK, BG_BLUE, BG_CYAN, BG_GREEN, BG_MAGENTA, BG_RED, BG_WHITE, BG_YELLOW, BLACK, BLUE,
        CYAN, DEFAULT, GREEN, MAGENTA, RED, WHITE, YELLOW,
    },
};
use std::{
    borrow::Cow,
    fmt::{self, Display},
};

use super::utils::{color256_bg_color, color256_fg_color, rgb_bg_color, rgb_fg_color};

#[derive(Debug, Clone, PartialEq)]
pub enum ColorType {
    Rgb(u8, u8, u8),
    Color256(u8),
    Use(Color),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DisplayType {
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
    fn get_str(&self, is_text_color: DisplayType) -> String;
    fn blod(&self) -> String;
}

impl Default for Color {
    fn default() -> Self {
        Color::Default
    }
}

impl ColorDisplay for Color {
    fn get_str(&self, is_text_color: DisplayType) -> String {
        let str = match is_text_color {
            DisplayType::Text => match self {
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
        };
        str.to_string()
    }

    fn blod(&self) -> String {
        format!("{}{}", self.to_string(), BLOD)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_str(DisplayType::Text))
    }
}

impl Display for ColorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_str(DisplayType::Text))
    }
}

impl ColorDisplay for ColorType {
    fn get_str(&self, is_text_color: DisplayType) -> String {
        match self {
            ColorType::Color256(fg) => match is_text_color {
                DisplayType::Text => color256_fg_color(*fg),
                DisplayType::Background => color256_bg_color(*fg),
            },
            ColorType::Rgb(r, g, b) => match is_text_color {
                DisplayType::Text => rgb_fg_color(*r, *g, *b),
                DisplayType::Background => rgb_bg_color(*r, *g, *b),
            },
            ColorType::Use(color) => color.get_str(is_text_color.clone()),
        }
    }
    fn blod(&self) -> String {
        format!("{}{}", self.to_string(), BLOD)
    }
}

impl Default for ColorType {
    fn default() -> Self {
        ColorType::Use(Color::Default)
    }
}
