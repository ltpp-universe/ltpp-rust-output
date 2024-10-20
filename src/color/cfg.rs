use crate::*;
use color::{blod::BLOD, color::DEFAULT};

#[test]
fn test_text_color() {
    let text_color: Color = Color::Default;
    let text_color_str: &String = &text_color.to_string();
    assert_eq!(text_color_str, DEFAULT);
}

#[test]
fn test_text_color_get_str() {
    let text_color_str: &str = &Color::Default.get_str(DisplayType::Text);
    let res_text_color_str: &String = &Color::Default.to_string();
    assert_eq!(text_color_str, res_text_color_str);
}

#[test]
fn test_text_color_blod() {
    let text_color_str: String = Color::Default.blod();
    let ans_text_color_str: String = format!("{}{}", Color::Default, BLOD);
    assert_eq!(text_color_str, ans_text_color_str);
}

#[test]
fn test_bg_color() {
    let bg_color: Color = Color::Default;
    let bg_color_str: &String = &bg_color.to_string();
    assert_eq!(bg_color_str, DEFAULT);
}

#[test]
fn test_bg_color_get_str() {
    let bg_color_str: &str = &Color::Default.get_str(DisplayType::Background);
    let ans_bg_color_str: &String = &Color::Default.to_string();
    assert_eq!(bg_color_str, ans_bg_color_str);
}

#[test]
fn test_bg_color_blod() {
    let bg_color_str: String = Color::Default.blod();
    let ans_bg_color_str: String = format!("{}{}", Color::Default, BLOD);
    assert_eq!(bg_color_str, ans_bg_color_str);
}
