use crate::{
    color,
    output::{
        output::colored_output,
        r#type::{Output, OutputBuilder},
    },
};

#[test]
fn test_output_struct() {
    colored_output(Output {
        text: "test_output_struct",
        text_color: Some(color::r#type::TextColor::Default),
        text_bg_color: Some(color::r#type::BgColor::Red),
        show_time: Some(true),
        show_code_location: Some(true),
        time_text_color: Some(color::r#type::TextColor::Green),
        time_bg_color: Some(color::r#type::BgColor::Red),
        code_location_text_color: Some(color::r#type::TextColor::Blue),
        code_location_bg_color: Some(color::r#type::BgColor::Red),
        split: Some(" => "),
        split_color: Some(color::r#type::TextColor::Cyan),
        split_bg_color: Some(color::r#type::BgColor::Red),
        ..Default::default()
    });

    Output {
        text: "test_output_struct_output",
        text_color: Some(color::r#type::TextColor::Default),
        text_bg_color: Some(color::r#type::BgColor::Red),
        show_time: Some(true),
        show_code_location: Some(true),
        time_text_color: Some(color::r#type::TextColor::Green),
        time_bg_color: Some(color::r#type::BgColor::Red),
        code_location_text_color: Some(color::r#type::TextColor::Blue),
        code_location_bg_color: Some(color::r#type::BgColor::Red),
        split: Some(" => "),
        split_color: Some(color::r#type::TextColor::Cyan),
        split_bg_color: Some(color::r#type::BgColor::Red),
        ..Default::default()
    }
    .output();
}

#[test]
fn test_output_builder() {
    colored_output(
        OutputBuilder::new()
            .set_text("test_output_builder")
            .set_text_color(color::r#type::TextColor::Cyan)
            .set_time_text_color(color::r#type::TextColor::Blue)
            .set_code_location_text_color(color::r#type::TextColor::Red)
            .set_text_blod(true)
            .set_time_text_blod(true)
            .set_code_location_text_blod(true)
            .set_show_time(true)
            .set_show_code_location(true)
            .build(),
    );

    OutputBuilder::new()
        .set_text("test_output_builder_output")
        .set_text_color(color::r#type::TextColor::Cyan)
        .set_time_text_color(color::r#type::TextColor::Blue)
        .set_code_location_text_color(color::r#type::TextColor::Red)
        .set_text_blod(true)
        .set_time_text_blod(true)
        .set_code_location_text_blod(true)
        .set_show_time(true)
        .set_show_code_location(true)
        .build()
        .output();

    OutputBuilder::new()
        .set_text("test_output_builder_output")
        .set_text_color(color::r#type::TextColor::Cyan)
        .set_time_text_color(color::r#type::TextColor::Blue)
        .set_code_location_text_color(color::r#type::TextColor::Red)
        .set_text_blod(true)
        .set_time_text_blod(true)
        .set_code_location_text_blod(true)
        .set_show_time(true)
        .set_show_code_location(true)
        .output();
}
