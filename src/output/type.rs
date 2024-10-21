use super::output::output;
use crate::*;

/// Output struct
///
/// [Official Documentation](https://docs.ltpp.vip/LTPP-RUST-OUTPUT/)
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
/// ### Using the output Method
///
/// ```rust
/// use ltpp_output::*;
/// Output {
///     text: "test_output_struct_output",
///     text_color: Some(ColorType::Use(Color::Default)),
///     text_bg_color: Some(ColorType::Color256(0x000000)),
///     show_time: Some(true),
///     time_text_color: Some(ColorType::Rgb(255, 255, 255)),
///     time_bg_color: Some(ColorType::Use(Color::Yellow)),
///     split: Some(" => "),
///     split_color: Some(ColorType::Use(Color::Cyan)),
///     split_bg_color: Some(ColorType::Use(Color::Yellow)),
///     ..Default::default()
/// }
/// .output();
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
///         .build(),
/// );
/// ```
///
/// ### Using the output Method
///
/// ```rust
/// use ltpp_output::*;
/// OutputBuilder::new()
///     .set_text("test_output_builder_output")
///     .set_text("test_output_builder")
///     .set_text_color(ColorType::Color256(0xffffff))
///     .set_time_text_color(ColorType::Rgb(255, 200, 255))
///     .set_text_blod(true)
///     .set_time_text_blod(true)
///     .set_show_time(true)
///     .build()
///     .output();
/// ```
#[derive(Debug, Clone)]
pub struct Output<'a> {
    /// text
    pub text: &'a str,
    /// text color
    pub text_color: Option<ColorType>,
    /// Text background color
    pub text_bg_color: Option<ColorType>,
    /// Bold text
    pub text_blod: Option<bool>,
    /// Whether to show time
    pub show_time: Option<bool>,
    /// Time text color
    pub time_text_color: Option<ColorType>,
    /// Time background color
    pub time_bg_color: Option<ColorType>,
    /// Time text bold
    pub time_text_blod: Option<bool>,
    /// Separator
    pub split: Option<&'a str>,
    /// Separator text color
    pub split_color: Option<ColorType>,
    /// Separator background color
    pub split_bg_color: Option<ColorType>,
    /// Separator text bold
    pub split_text_blod: Option<bool>,
}
