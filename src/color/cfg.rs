use crate::color::{blod::BLOD, color::DEFAULT, r#type::BgColor};

use super::r#type::{ColorDisplay, TextColor};

#[test]
fn test_text_color() {
    let text_color: TextColor = TextColor::Default;
    let text_color_str: &String = &text_color.to_string();
    assert_eq!(text_color_str, DEFAULT);
}

#[test]
fn test_text_color_get_str() {
    let text_color_str: &str = TextColor::Default.get_str();
    let res_text_color_str: &String = &TextColor::Default.to_string();
    assert_eq!(text_color_str, res_text_color_str);
}

#[test]
fn test_text_color_blod() {
    let text_color_str: String = TextColor::Default.blod();
    let ans_text_color_str: String = format!("{}{}", TextColor::Default, BLOD);
    assert_eq!(text_color_str, ans_text_color_str);
}

#[test]
fn test_bg_color() {
    let bg_color: BgColor = BgColor::Default;
    let bg_color_str: &String = &bg_color.to_string();
    assert_eq!(bg_color_str, DEFAULT);
}

#[test]
fn test_bg_color_get_str() {
    let bg_color_str: &str = BgColor::Default.get_str();
    let ans_bg_color_str: &String = &TextColor::Default.to_string();
    assert_eq!(bg_color_str, ans_bg_color_str);
}

#[test]
fn test_bg_color_blod() {
    let bg_color_str: String = BgColor::Default.blod();
    let ans_bg_color_str: String = format!("{}{}", BgColor::Default, BLOD);
    assert_eq!(bg_color_str, ans_bg_color_str);
}
