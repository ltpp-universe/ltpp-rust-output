use crate::{
    color,
    output::{
        output::output,
        r#type::{Output, OutputBuilder},
    },
};

#[test]
fn test_output_struct() {
    output(Output {
        text: "test_output_struct",
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

    Output {
        text: "test_output_struct_output",
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
    }
    .output();
}

#[test]
fn test_output_builder() {
    output(
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
            .build(),
    );

    OutputBuilder::new()
        .set_text("test_output_builder_output")
        .set_text_color(color::r#type::Color::Cyan)
        .set_time_text_color(color::r#type::Color::Blue)
        .set_code_location_text_color(color::r#type::Color::Red)
        .set_text_blod(true)
        .set_time_text_blod(true)
        .set_code_location_text_blod(true)
        .set_show_time(true)
        .set_show_code_location(true)
        .build()
        .output();

    OutputBuilder::new()
        .set_text("test_output_builder_output")
        .set_text_color(color::r#type::Color::Cyan)
        .set_time_text_color(color::r#type::Color::Blue)
        .set_code_location_text_color(color::r#type::Color::Red)
        .set_text_blod(true)
        .set_time_text_blod(true)
        .set_code_location_text_blod(true)
        .set_show_time(true)
        .set_show_code_location(true)
        .output();
}
