use crate::text::r#type::Text;
use crate::time::time::get_now_time_format;
use crate::*;
use std::{borrow::Cow, panic};

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
///     show_code_location: Some(true),
///     time_text_color: Some(ColorType::Rgb(255, 255, 255)),
///     time_bg_color: Some(ColorType::Use(Color::Yellow)),
///     code_location_text_color: Some(ColorType::Color256(0xffffff)),
///     code_location_bg_color: Some(ColorType::Use(Color::Yellow)),
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
///         .set_code_location_text_color(ColorType::Use(Color::Yellow))
///         .set_text_blod(true)
///         .set_time_text_blod(true)
///         .set_code_location_text_blod(true)
///         .set_show_time(true)
///         .set_show_code_location(true)
///         .build()
/// );
/// ```
pub fn output(output: Output) {
    /// Text
    let text: &str = output.text.clone();
    let text_color: ColorType = output.text_color.clone().unwrap_or_default();
    let text_bg_color: ColorType = output.text_bg_color.clone().unwrap_or_default();
    let text_blod: bool = output.text_blod.clone().unwrap_or_default();
    /// Time
    let show_time: bool = output.show_time.clone().unwrap_or_default();
    let time_color: ColorType = output.time_text_color.clone().unwrap_or_default();
    let time_bg_color: ColorType = output.time_bg_color.clone().unwrap_or_default();
    let time_text_blod: bool = output.time_text_blod.clone().unwrap_or_default();
    /// Code Location
    let show_code_location: bool = output.show_code_location.clone().unwrap_or_default();
    let code_location_color: ColorType =
        output.code_location_text_color.clone().unwrap_or_default();
    let code_location_bg_color: ColorType =
        output.code_location_bg_color.clone().unwrap_or_default();
    let code_location_text_blod: bool = output.time_text_blod.clone().unwrap_or_default();
    /// Separator
    let split: &str = output.split.clone().unwrap_or_default();
    let split_color: ColorType = output.split_color.clone().unwrap_or_default();
    let split_bg_color: ColorType = output.split_bg_color.clone().unwrap_or_default();
    let split_text_blod: bool = output.time_text_blod.clone().unwrap_or_default();

    let mut output: String = String::new();

    // Add time
    if show_time {
        let time_str: &String = &format!("[{}]", get_now_time_format());
        let colored_time: &Cow<'_, str> = &Text {
            text: time_str,
            text_color: time_color,
            text_bg_color: time_bg_color,
            blod: time_text_blod,
        }
        .get_display_str_cow();
        output.push_str(colored_time);
    }

    // Add code location
    if show_code_location {
        let location: &panic::Location<'_> = panic::Location::caller();
        let code_location_str: &String = &format!("({}:{})", location.file(), location.line());
        let colored_time: &Cow<'_, str> = &Text {
            text: code_location_str,
            text_color: code_location_color,
            text_bg_color: code_location_bg_color,
            blod: code_location_text_blod,
        }
        .get_display_str_cow();
        output.push_str(colored_time);
    }

    /// Add separator
    let colored_time: &Cow<'_, str> = &Text {
        text: split,
        text_color: split_color,
        text_bg_color: split_bg_color,
        blod: split_text_blod,
    }
    .get_display_str_cow();
    output.push_str(colored_time);

    /// Add text
    let colored_text: &Cow<'_, str> = &Text {
        text,
        text_color,
        text_bg_color,
        blod: text_blod,
    }
    .get_display_str_cow();
    output.push_str(colored_text);

    println!("{}", output);
}
