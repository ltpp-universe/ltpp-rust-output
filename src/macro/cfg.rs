use crate::output_macro;
use crate::{
    color,
    output::{
        output::output,
        r#type::{Output, OutputBuilder},
    },
};

#[test]
fn test_proc_macro() {
    output_macro!(Output {
        text: "test_proc_macro",
        text_color: Some(color::r#type::Color::Default),
        text_bg_color: Some(color::r#type::Color::Red),
        show_time: Some(true),
        show_code_location: Some(true),
        time_text_color: Some(color::r#type::Color::Green),
        time_bg_color: Some(color::r#type::Color::Red),
        code_location_text_color: Some(color::r#type::Color::Blue),
        code_location_bg_color: Some(color::r#type::Color::Red),
        split: Some(" => "),
        split_color: Some(color::r#type::Color::Cyan),
        split_bg_color: Some(color::r#type::Color::Red),
        ..Default::default()
    });

    output_macro!(OutputBuilder::new()
        .set_text("test_output_builder")
        .set_text_color(color::r#type::Color::Cyan)
        .set_time_text_color(color::r#type::Color::Blue)
        .set_code_location_text_color(color::r#type::Color::Red)
        .set_text_blod(true)
        .set_time_text_blod(true)
        .set_code_location_text_blod(true)
        .set_show_time(true)
        .set_show_code_location(true)
        .build());

    output_macro!(
        Output {
            text: "test_proc_macro",
            text_color: Some(color::r#type::Color::Default),
            text_bg_color: Some(color::r#type::Color::Red),
            show_time: Some(true),
            show_code_location: Some(true),
            time_text_color: Some(color::r#type::Color::Green),
            time_bg_color: Some(color::r#type::Color::Red),
            code_location_text_color: Some(color::r#type::Color::Blue),
            code_location_bg_color: Some(color::r#type::Color::Red),
            split: Some(" => "),
            split_color: Some(color::r#type::Color::Cyan),
            split_bg_color: Some(color::r#type::Color::Red),
            ..Default::default()
        },
        OutputBuilder::new()
            .set_text("test_output_builder")
            .set_text_color(color::r#type::Color::Cyan)
            .set_time_text_color(color::r#type::Color::Blue)
            .set_code_location_text_color(color::r#type::Color::Red)
            .set_text_blod(true)
            .set_time_text_blod(true)
            .set_code_location_text_blod(true)
            .set_show_time(true)
            .set_show_code_location(true)
            .build()
    );
}
