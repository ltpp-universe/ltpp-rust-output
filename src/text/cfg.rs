use crate::color::r#type::{BgColor, TextColor};

use super::r#type::Text;

#[test]
fn test_text() {
    let text_default: Text<'_> = Text::default();
    let text_default_str: &String = &text_default.get_display_str_cow().into_owned();
    let text: Text<'_> = Text {
        text: "",
        text_color: TextColor::default(),
        text_bg_color: BgColor::default(),
        blod: false,
    };
    let text_str: &String = &text.get_display_str_cow().into_owned();
    assert_eq!(text_default_str, text_str);
}
