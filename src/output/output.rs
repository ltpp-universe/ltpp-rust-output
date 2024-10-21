use task::r#type::Task;

use crate::text::r#type::Text;
use crate::time::time::get_now_time_format;
use crate::*;
use std::borrow::Cow;

/// Output
///
/// [Official Documentation](https://docs.ltpp.vip/LTPP-RUST-OUTPUT/)
///
/// # Parameters
/// - `Output`: The output struct
///
/// # Code Example
///
/// ## Using the Struct
///
/// ### Using the output Function
///
/// ```rust
/// use ltpp_output::*;
/// output(Output {
///     text: "test_output_struct",
///     text_color: Some(ColorType::Use(Color::Default)),
///     text_bg_color: Some(ColorType::Color256(0x000000)),
///     show_time: Some(true),
///     time_text_color: Some(ColorType::Rgb(255, 255, 255)),
///     time_bg_color: Some(ColorType::Use(Color::Yellow)),
///     split: Some(" => "),
///     split_color: Some(ColorType::Use(Color::Cyan)),
///     split_bg_color: Some(ColorType::Use(Color::Yellow)),
///     ..Default::default()
/// });
/// ```
///
/// ## Using the Constructor
///
/// ### Using the output Function
///
/// ```rust
/// use ltpp_output::*;
/// output(
///     OutputBuilder::new()
///         .set_text("test_output_builder")
///         .set_text_color(ColorType::Color256(0xffffff))
///         .set_time_text_color(ColorType::Rgb(255, 200, 255))
///         .set_text_blod(true)
///         .set_time_text_blod(true)
///         .set_show_time(true)
///         .build()
/// );
/// ```
pub fn output(output: Output) {
    // Text
    let text: &str = output.text;
    let text_color: ColorType = output.text_color.clone().unwrap_or_default();
    let text_bg_color: ColorType = output.text_bg_color.clone().unwrap_or_default();
    let text_blod: bool = output.text_blod.clone().unwrap_or_default();
    // Time
    let show_time: bool = output.show_time.clone().unwrap_or_default();
    let time_color: ColorType = output.time_text_color.clone().unwrap_or_default();
    let time_bg_color: ColorType = output.time_bg_color.clone().unwrap_or_default();
    let time_text_blod: bool = output.time_text_blod.clone().unwrap_or_default();
    // Separator
    let split: &str = output.split.clone().unwrap_or_default();
    let split_color: ColorType = output.split_color.clone().unwrap_or_default();
    let split_bg_color: ColorType = output.split_bg_color.clone().unwrap_or_default();
    let split_text_blod: bool = output.time_text_blod.clone().unwrap_or_default();

    let mut output: String = String::new();

    let mut task_list: Task<'_> = Task::new();

    // Add time
    let time_str: &String = if show_time {
        &format!("[{}]", get_now_time_format())
    } else {
        &String::new()
    };
    task_list.add(Text {
        text: time_str,
        text_color: time_color,
        text_bg_color: time_bg_color,
        blod: time_text_blod,
    });

    // Add separator
    task_list.add(Text {
        text: split,
        text_color: split_color,
        text_bg_color: split_bg_color,
        blod: split_text_blod,
    });

    // Add text
    task_list.add(Text {
        text,
        text_color,
        text_bg_color,
        blod: text_blod,
    });

    // run
    task_list.run_all();
}
