#[macro_export]
/// 输出宏
///
/// # 参数
/// - Output: 输出结构体
///
/// # 代码示例
///
/// ## 使用结构体
///
/// ```rust
/// use ltpp_output::*;
/// output_macro!(Output {
///     text: "test_output_struct",
///     text_color: Some(Color::Default),
///     text_bg_color: Some(Color::Red),
///     show_time: Some(true),
///     show_code_location: Some(true),
///     time_text_color: Some(Color::Green),
///     time_bg_color: Some(Color::Red),
///     code_location_text_color: Some(Color::Blue),
///     code_location_bg_color: Some(Color::Red),
///     split: Some(" => "),
///     split_color: Some(Color::Cyan),
///     split_bg_color: Some(Color::Red),
///     ..Default::default()
/// });
/// ```
///
/// ## 使用构造器
/// use ltpp_output::*;
/// ```rust
/// output_macro!(
///     OutputBuilder::new()
///         .set_text("test_output_builder")
///         .set_text_color(Color::Cyan)
///         .set_time_text_color(Color::Blue)
///         .set_code_location_text_color(Color::Red)
///         .set_text_blod(true)
///         .set_time_text_blod(true)
///         .set_code_location_text_blod(true)
///         .set_show_time(true)
///         .set_show_code_location(true)
///         .build(),
/// );
/// ```
macro_rules! output_macro {
    ($($output:expr),*) => {
        $(
            $output.output();
        )*
    };
}
