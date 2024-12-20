use crate::{output, ColorType, Output};
/// OutputBuilder struct
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/),
///
/// # Code Example
///
/// ## Using the OutputBuilder
///
/// ### Using the output Function
///
/// ```rust
/// use color_output::*;
/// output(
///     OutputBuilder::new_from(Output::default())
///         .text("test_output_builder")
///         .text_color(ColorType::Color256(0xffffff))
///         .text_bg_color(ColorType::Color256(0xffffff))
///         .split_bg_color(ColorType::Color256(0xffffff))
///         .time_text_color(ColorType::Rgb(255, 200, 255))
///         .text_blod(true)
///         .time_text_blod(true)
///         .show_time(true)
///         .endl(true)
///         .build(),
/// );
/// ```
///
/// ### Using the output Method
///
/// ```rust
/// use color_output::*;
/// OutputBuilder::new()
///     .text("test_output_builder_output")
///     .text_bg_color(ColorType::Color256(0xffffff))
///     .text_color(ColorType::Color256(0xffffff))
///     .time_text_color(ColorType::Rgb(255, 200, 255))
///     .text_blod(true)
///     .time_text_blod(true)
///     .show_time(true)
///     .endl(true)
///     .build()
///     .output();
/// ```
pub struct OutputBuilder<'a> {
    pub output: Output<'a>,
}
