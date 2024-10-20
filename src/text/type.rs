use crate::{
    color::{
        blod,
        color::DEFAULT,
        r#type::{Color, ColorDisplay},
    },
    ColorType,
};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct Text<'a> {
    pub text: &'a str,
    pub text_color: Color,
    pub text_bg_color: Color,
    pub blod: bool,
}

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Text {
            text: "",
            text_color: Color::default(),
            text_bg_color: Color::default(),
            blod: false,
        }
    }
}

impl<'a> Text<'a> {
    pub fn get_display_str_cow(&self) -> Cow<'a, str> {
        let text: &str = self.text.clone();
        let blod: bool = self.blod.clone();
        let text_color: String = if blod {
            self.text_color.blod()
        } else {
            self.text_color.to_string()
        };
        let text_bg_color: &str = self.text_bg_color.get_str(ColorType::Background);
        let colored_text: String = format!("{}{}{}{}", text_bg_color, text_color, text, DEFAULT);
        Cow::Owned(colored_text)
    }
}
