use crate::color::color::DEFAULT;
use crate::*;
use crate::{color::blod, DisplayType};
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
    /// Gets the display string as a `Cow` (clone on write).
    ///
    /// This method generates a formatted string that represents the text with
    /// the appropriate color and background color. If the text is bold, it applies
    /// bold formatting to the text color.
    ///
    /// # Returns
    /// - `Cow<'a, str>`: An owned copy of the formatted string.
    pub fn get_display_str_cow(&self) -> Cow<'a, str> {
        let text: &str = self.text.clone();
        let blod: bool = self.blod.clone();
        let text_color: String = if blod {
            self.text_color.blod()
        } else {
            self.text_color.to_string()
        };
        let text_bg_color: &String = &self.text_bg_color.get_str(DisplayType::Background);
        let colored_text: String = format!("{}{}{}{}", text_bg_color, text_color, text, DEFAULT);
        Cow::Owned(colored_text)
    }
}
