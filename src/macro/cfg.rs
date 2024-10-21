use crate::*;

#[test]
fn test_proc_macro() {
    output_macro!(Output {
        text: "test_proc_macro",
        text_color: Some(ColorType::default()),
        text_bg_color: Some(ColorType::Use(Color::Yellow)),
        show_time: Some(true),
        time_text_color: Some(ColorType::Use(Color::Green)),
        time_bg_color: Some(ColorType::Color256(0xffffff)),
        split: Some(" => "),
        split_color: Some(ColorType::Use(Color::Cyan)),
        split_bg_color: Some(ColorType::Use(Color::Yellow)),
        ..Default::default()
    });

    output_macro!(OutputBuilder::new()
        .set_text("test_output_builder")
        .set_text_color(ColorType::Use(Color::Cyan))
        .set_time_text_color(ColorType::Use(Color::Blue))
        .set_text_blod(true)
        .set_time_text_blod(true)
        .set_show_time(true)
        .build());

    output_macro!(
        Output {
            text: "test_proc_macro",
            text_color: Some(ColorType::default()),
            text_bg_color: Some(ColorType::Use(Color::Yellow)),
            show_time: Some(true),
            time_text_color: Some(ColorType::Use(Color::Green)),
            time_bg_color: Some(ColorType::Color256(0xffffff)),
            split: Some(" => "),
            split_color: Some(ColorType::Use(Color::Cyan)),
            split_bg_color: Some(ColorType::Use(Color::Yellow)),
            ..Default::default()
        },
        OutputBuilder::new()
            .set_text("test_output_builder")
            .set_text_color(ColorType::Color256(0xffffff))
            .set_time_text_color(ColorType::Rgb(255, 200, 255))
            .set_text_blod(true)
            .set_time_text_blod(true)
            .set_show_time(true)
            .build()
    );
}
