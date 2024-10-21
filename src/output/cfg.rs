use crate::*;

#[test]
fn test_output_struct() {
    output(Output {
        text: "test_output_struct",
        text_color: Some(ColorType::Use(Color::Default)),
        text_bg_color: Some(ColorType::Color256(0x000000)),
        show_time: Some(true),
        time_text_color: Some(ColorType::Rgb(255, 255, 255)),
        time_bg_color: Some(ColorType::Use(Color::Yellow)),
        split: Some(" => "),
        split_color: Some(ColorType::Use(Color::Cyan)),
        split_bg_color: Some(ColorType::Use(Color::Yellow)),
        ..Default::default()
    });

    Output {
        text: "test_output_struct_output",
        text_color: Some(ColorType::Use(Color::Default)),
        text_bg_color: Some(ColorType::Use(Color::Blue)),
        show_time: Some(true),
        time_text_color: Some(ColorType::Rgb(255, 255, 255)),
        time_bg_color: Some(ColorType::Use(Color::Yellow)),
        split: Some(" => "),
        split_color: Some(ColorType::Use(Color::Cyan)),
        split_bg_color: Some(ColorType::Use(Color::Yellow)),
        ..Default::default()
    }
    .output();
}
