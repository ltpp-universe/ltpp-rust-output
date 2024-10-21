use crate::*;
use output_list_builder::r#type::OutputListBuilder;

#[test]
fn test_new_from_output_list_builder() {
    OutputListBuilder::new_from(vec![Output::default()])
        .add(Output::default())
        .add(Output {
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
        })
        .run();
}
