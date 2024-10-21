use crate::Output;

/// OutputListBuilder struct
///
/// [Official Documentation](https://docs.ltpp.vip/LTPP-RUST-OUTPUT/),
///
/// # Code Example
///
/// ## Using the OutputBuilder
///
/// ```rust
/// use ltpp_output::*;
/// OutputListBuilder::new_from(vec![Output::default()])
///     .add(Output::default())
///     .add(Output {
///         text: "test_new_from_output_list_builder",
///         text_color: ColorType::Use(Color::Default),
///         text_bg_color: ColorType::Color256(0x000000),
///         show_time: true,
///         time_text_color: ColorType::Rgb(255, 255, 255),
///         time_bg_color: ColorType::Use(Color::Yellow),
///         split: " => ",
///         split_color: ColorType::Use(Color::Cyan),
///         split_bg_color: ColorType::Use(Color::Yellow),
///         ..Default::default()
///     })
///     .run();
/// ```
#[derive(Debug, Clone)]
pub struct OutputListBuilder<'a> {
    /// A list of output structures.
    pub(crate) output_list: Vec<Output<'a>>,
}
