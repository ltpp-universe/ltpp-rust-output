use color::blod::{BLOD, UNBLOD};
use color::color::RESET;

use crate::color::color::DEFAULT;
use crate::DisplayType;
use crate::*;
use std::borrow::Cow;

/// Represents a text with formatting options.
#[derive(Debug, Clone, PartialEq)]
pub struct Text<'a> {
    /// The actual text content.
    pub text: &'a str,
    /// The color of the text.
    pub text_color: ColorType,
    /// The background color of the text.
    pub text_bg_color: ColorType,
    /// Whether the text should be bold.
    pub blod: bool,
}

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Text {
            text: "",
            text_color: ColorType::default(),
            text_bg_color: ColorType::default(),
            blod: false,
        }
    }
}

impl<'a> Text<'a> {
    /// 创建文本结构体
    ///
    /// # 参数
    /// - `Text`: 文本结构体
    ///
    /// # 返回值
    /// - `Text`: 文本结构体
    pub fn new_from(text: &Text<'a>) -> Self {
        Self { ..text.clone() }
    }

    /// Gets the display string as a `Cow` (clone on write).
    ///
    /// This method generates a formatted string that represents the text with
    /// the appropriate color and background color. If the text is bold, it applies
    /// bold formatting to the text color.
    ///
    /// # Returns
    /// - `Cow<'a, str>`: An owned copy of the formatted string.
    pub fn get_display_str_cow(&self) -> Cow<'a, str> {
        let text: &str = self.text;
        let blod: bool = self.blod.clone();
        let text_color: &String = &self.text_color.to_string();
        let text_bg_color: &String = &self.text_bg_color.get_str(DisplayType::Background);
        let colored_text: String = if blod {
            format!(
                "{}{}{}{}{}{}",
                text_bg_color, text_color, BLOD, text, UNBLOD, RESET
            )
        } else {
            format!("{}{}{}{}", text_bg_color, text_color, text, RESET)
        };
        Cow::Owned(colored_text)
    }
}
