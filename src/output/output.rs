use super::r#type::Output;
use crate::color::r#type::{Color, ColorDisplay};
use crate::text::r#type::Text;
use crate::{color::color::DEFAULT, time::time::get_now_time};
use std::{borrow::Cow, panic};

/// 输出
///
/// # 参数
/// - Output: 输出结构体
///
/// # 代码示例
///
/// ## 使用结构体
///
/// ### 使用 output 函数
///
/// ```rust
/// use ltpp_output::*;
/// output(Output {
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
///
/// ### 使用 output 函数
///
/// ```rust
/// use ltpp_output::*;
/// output(
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
pub fn output(output: Output) {
    /// 文字
    let text: &str = output.text.clone();
    let text_color: Color = output.text_color.clone().unwrap_or_default();
    let text_bg_color: Color = output.text_bg_color.clone().unwrap_or_default();
    let text_blod: bool = output.text_blod.clone().unwrap_or_default();
    /// 时间
    let show_time: bool = output.show_time.clone().unwrap_or_default();
    let time_color: Color = output.time_text_color.clone().unwrap_or_default();
    let time_bg_color: Color = output.time_bg_color.clone().unwrap_or_default();
    let time_text_blod: bool = output.time_text_blod.clone().unwrap_or_default();
    /// 代码位置
    let show_code_location: bool = output.show_code_location.clone().unwrap_or_default();
    let code_location_color: Color = output.code_location_text_color.clone().unwrap_or_default();
    let code_location_bg_color: Color = output.code_location_bg_color.clone().unwrap_or_default();
    let code_location_text_blod: bool = output.time_text_blod.clone().unwrap_or_default();
    /// 分隔符
    let split: &str = output.split.clone().unwrap_or_default();
    let split_color: Color = output.split_color.clone().unwrap_or_default();
    let split_bg_color: Color = output.split_bg_color.clone().unwrap_or_default();
    let split_text_blod: bool = output.time_text_blod.clone().unwrap_or_default();

    let mut output: String = String::new();

    // 添加时间
    if show_time {
        let time_str: &String = &format!("[{}]", get_now_time());
        let colored_time: &Cow<'_, str> = &Text {
            text: time_str,
            text_color: time_color,
            text_bg_color: time_bg_color,
            blod: time_text_blod,
        }
        .get_display_str_cow();
        output.push_str(colored_time);
    }

    // 添加代码位置
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

    /// 添加分隔符
    let colored_time: &Cow<'_, str> = &Text {
        text: split,
        text_color: split_color,
        text_bg_color: split_bg_color,
        blod: split_text_blod,
    }
    .get_display_str_cow();
    output.push_str(colored_time);

    /// 添加文字
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
