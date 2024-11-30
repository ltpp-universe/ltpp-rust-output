use std::vec;

use crate::*;

#[test]
fn test_output_list_struct() {
    OutputList(vec![
        Output {
            text: "test_output_list_struct_1",
            text_color: ColorType::Use(Color::Default),
            text_bg_color: ColorType::Color256(0x000000),
            show_time: true,
            time_text_color: ColorType::Rgb(255, 255, 255),
            time_bg_color: ColorType::Use(Color::Yellow),
            split: " => ",
            split_color: ColorType::Use(Color::Cyan),
            split_bg_color: ColorType::Use(Color::Yellow),
            endl: false,
            ..Default::default()
        },
        Output {
            text: "test_output_struct_output_2",
            text_color: ColorType::Use(Color::Default),
            text_bg_color: ColorType::Use(Color::Blue),
            show_time: true,
            time_text_color: ColorType::Rgb(255, 255, 255),
            time_bg_color: ColorType::Use(Color::Yellow),
            split: " => ",
            split_color: ColorType::Use(Color::Cyan),
            split_bg_color: ColorType::Use(Color::Yellow),
            endl: true,
            ..Default::default()
        },
    ])
    .output();
}