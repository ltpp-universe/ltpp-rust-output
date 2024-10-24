use super::r#type::Text;
use crate::*;

#[test]
fn test_text() {
    let text_default: Text<'_> = Text::default();
    let text_default_str: &String = &text_default.get_display_str_cow().into_owned();
    let text: Text<'_> = Text {
        text: "",
        text_color: ColorType::default(),
        text_bg_color: ColorType::default(),
        blod: false,
        endl: false,
    };
    let text_str: &String = &text.get_display_str_cow().into_owned();
    assert_eq!(text_default_str, text_str);
}
