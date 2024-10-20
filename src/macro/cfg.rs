use crate::*;

#[test]
fn test_proc_macro() {
    output_macro!(Output {
        text: "test_proc_macro",
        text_color: Some(ColorType::default()),
        text_bg_color: Some(ColorType::Use(Color::Yellow)),
        show_time: Some(true),
        show_code_location: Some(true),
        time_text_color: Some(ColorType::Use(Color::Green)),
        time_bg_color: Some(ColorType::Use(Color::Yellow)),
        code_location_text_color: Some(ColorType::Use(Color::Blue)),
        code_location_bg_color: Some(ColorType::Use(Color::Yellow)),
        split: Some(" => "),
        split_color: Some(ColorType::Use(Color::Cyan)),
        split_bg_color: Some(ColorType::Use(Color::Yellow)),
        ..Default::default()
    });

    output_macro!(OutputBuilder::new()
        .set_text("test_output_builder")
        .set_text_color(ColorType::Use(Color::Cyan))
        .set_time_text_color(ColorType::Use(Color::Blue))
        .set_code_location_text_color(ColorType::Use(Color::Yellow))
        .set_text_blod(true)
        .set_time_text_blod(true)
        .set_code_location_text_blod(true)
        .set_show_time(true)
        .set_show_code_location(true)
        .build());

    output_macro!(
        Output {
            text: "test_proc_macro",
            text_color: Some(ColorType::Use(Color::Default)),
            text_bg_color: Some(ColorType::Use(Color::Yellow)),
            show_time: Some(true),
            show_code_location: Some(true),
            time_text_color: Some(ColorType::Use(Color::Green)),
            time_bg_color: Some(ColorType::Use(Color::Yellow)),
            code_location_text_color: Some(ColorType::Use(Color::Blue)),
            code_location_bg_color: Some(ColorType::Use(Color::Yellow)),
            split: Some(" => "),
            split_color: Some(ColorType::Use(Color::Cyan)),
            split_bg_color: Some(ColorType::Use(Color::Yellow)),
            ..Default::default()
        },
        OutputBuilder::new()
            .set_text("test_output_builder")
            .set_text_color(ColorType::Use(Color::Cyan))
            .set_time_text_color(ColorType::Use(Color::Blue))
            .set_code_location_text_color(ColorType::Use(Color::Yellow))
            .set_text_blod(true)
            .set_time_text_blod(true)
            .set_code_location_text_blod(true)
            .set_show_time(true)
            .set_show_code_location(true)
            .build()
    );
}
