/// Output macro
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
/// ```rust
/// use ltpp_output::*;
/// output_macro!(Output {
///     text: "test_proc_macro",
///     text_color: Some(ColorType::default()),
///     text_bg_color: Some(ColorType::Use(Color::Yellow)),
///     show_time: Some(true),
///     time_text_color: Some(ColorType::Use(Color::Green)),
///     time_bg_color: Some(ColorType::Color256(0xffffff)),
///     split: Some(" => "),
///     split_color: Some(ColorType::Use(Color::Cyan)),
///     split_bg_color: Some(ColorType::Use(Color::Yellow)),
///     ..Default::default()
/// });
/// ```
///
/// ## Using the Constructor
///
/// ```rust
/// use ltpp_output::*;
/// output_macro!(OutputBuilder::new()
///     .set_text("test_output_builder")
///     .set_text_color(ColorType::Color256(0xffffff))
///     .set_time_text_color(ColorType::Rgb(255, 200, 255))
///     .set_text_blod(true)
///     .set_time_text_blod(true)
///     .set_show_time(true)
///     .build());
/// ```
///
/// ## Multiple Inputs
///
/// ```rust
/// use ltpp_output::*;
/// output_macro!(
///     Output {
///         text: "test_proc_macro",
///         text_color: Some(ColorType::default()),
///         text_bg_color: Some(ColorType::Use(Color::Yellow)),
///         show_time: Some(true),
///         time_text_color: Some(ColorType::Use(Color::Green)),
///         time_bg_color: Some(ColorType::Color256(0xffffff)),
///         split: Some(" => "),
///         split_color: Some(ColorType::Use(Color::Cyan)),
///         split_bg_color: Some(ColorType::Use(Color::Yellow)),
///         ..Default::default()
///     },
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
#[macro_export]
macro_rules! output_macro {
    ($($output:expr),*) => {
        $(
            $output.output();
        )*
    };
}
