use super::output::output;
use crate::color::r#type::Color;

/// 输出结构体
/// # 代码示例
///
/// ## 使用结构体
///
/// ### 使用 output 方法
///
/// ```rust
/// use ltpp_output::*;
/// Output {
///     text: "test_output_struct_output",
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
/// }
/// .output();
/// ```
///
/// ## 使用构造器
///
/// ### 使用 output 方法
///
/// ```rust
/// use ltpp_output::*;
/// OutputBuilder::new()
///     .set_text("test_output_builder_output")
///     .set_text_color(Color::Cyan)
///     .set_time_text_color(Color::Blue)
///     .set_code_location_text_color(Color::Red)
///     .set_text_blod(true)
///     .set_time_text_blod(true)
///     .set_code_location_text_blod(true)
///     .set_show_time(true)
///     .set_show_code_location(true)
///     .build()
///     .output();
/// ```
///
/// ### 使用 output 方法
///
/// ```rust
/// use ltpp_output::*;
/// OutputBuilder::new()
///     .set_text("test_output_builder_output")
///     .set_text_color(Color::Cyan)
///     .set_time_text_color(Color::Blue)
///     .set_code_location_text_color(Color::Red)
///     .set_text_blod(true)
///     .set_time_text_blod(true)
///     .set_code_location_text_blod(true)
///     .set_show_time(true)
///     .set_show_code_location(true)
///     .output();
/// ```
#[derive(Debug, Clone)]
pub struct Output<'a> {
    /// 输出文字
    pub text: &'a str,
    /// 输出文字颜色
    pub text_color: Option<Color>,
    /// 输出文字背景颜色
    pub text_bg_color: Option<Color>,
    /// 文字加粗
    pub text_blod: Option<bool>,
    /// 是否显示时间
    pub show_time: Option<bool>,
    /// 时间文字颜色
    pub time_text_color: Option<Color>,
    /// 时间背景颜色
    pub time_bg_color: Option<Color>,
    /// 时间文字加粗
    pub time_text_blod: Option<bool>,
    /// 是否显示代码位置
    pub show_code_location: Option<bool>,
    /// 代码位置文字颜色
    pub code_location_text_color: Option<Color>,
    /// 代码位置背景颜色
    pub code_location_bg_color: Option<Color>,
    /// 代码位置文字加粗
    pub code_location_text_blod: Option<bool>,
    /// 分隔符
    pub split: Option<&'a str>,
    /// 分隔符文字颜色
    pub split_color: Option<Color>,
    /// 分隔符背景颜色
    pub split_bg_color: Option<Color>,
    /// 分隔符文字加粗
    pub split_text_blod: Option<bool>,
}

impl<'a> Default for Output<'a> {
    fn default() -> Self {
        Output {
            text: "",
            text_color: Some(Color::Default),
            text_bg_color: Some(Color::Default),
            show_time: Some(true),
            show_code_location: Some(true),
            time_text_color: Some(Color::Default),
            time_bg_color: Some(Color::Default),
            code_location_text_color: Some(Color::Default),
            code_location_bg_color: Some(Color::Default),
            split: Some(""),
            split_color: Some(Color::Default),
            split_bg_color: Some(Color::Default),
            text_blod: Some(true),
            time_text_blod: Some(true),
            code_location_text_blod: Some(true),
            split_text_blod: Some(true),
        }
    }
}

impl<'a> Output<'a> {
    pub fn output(self) {
        output(self);
    }
}

pub struct OutputBuilder<'a> {
    output: Output<'a>,
}

impl<'a> OutputBuilder<'a> {
    pub fn new() -> Self {
        Self {
            output: Output::default(),
        }
    }

    pub fn set_text(&mut self, text: &'a str) -> &mut Self {
        self.output.text = text;
        self
    }

    pub fn set_text_color(&mut self, text_color: Color) -> &mut Self {
        self.output.text_color = Some(text_color);
        self
    }

    pub fn set_text_bg_color(&mut self, text_bg_color: Color) -> &mut Self {
        self.output.text_bg_color = Some(text_bg_color);
        self
    }

    pub fn set_text_blod(&mut self, text_blod: bool) -> &mut Self {
        self.output.text_blod = Some(text_blod);
        self
    }

    pub fn set_show_time(&mut self, show_time: bool) -> &mut Self {
        self.output.show_time = Some(show_time);
        self
    }

    pub fn set_time_text_color(&mut self, time_text_color: Color) -> &mut Self {
        self.output.time_text_color = Some(time_text_color);
        self
    }

    pub fn set_time_bg_color(&mut self, time_bg_color: Color) -> &mut Self {
        self.output.time_bg_color = Some(time_bg_color);
        self
    }

    pub fn set_time_text_blod(&mut self, time_text_blod: bool) -> &mut Self {
        self.output.time_text_blod = Some(time_text_blod);
        self
    }

    pub fn set_show_code_location(&mut self, show_code_location: bool) -> &mut Self {
        self.output.show_code_location = Some(show_code_location);
        self
    }

    pub fn set_code_location_text_color(&mut self, code_location_text_color: Color) -> &mut Self {
        self.output.code_location_text_color = Some(code_location_text_color);
        self
    }

    pub fn set_code_location_bg_color(&mut self, code_location_bg_color: Color) -> &mut Self {
        self.output.code_location_bg_color = Some(code_location_bg_color);
        self
    }

    pub fn set_code_location_text_blod(&mut self, code_location_text_blod: bool) -> &mut Self {
        self.output.code_location_text_blod = Some(code_location_text_blod);
        self
    }

    pub fn set_split(&mut self, split: &'a str) -> &mut Self {
        self.output.split = Some(split);
        self
    }

    pub fn set_split_color(&mut self, split_color: Color) -> &mut Self {
        self.output.split_color = Some(split_color);
        self
    }

    pub fn set_split_bg_color(&mut self, split_bg_color: Color) -> &mut Self {
        self.output.split_bg_color = Some(split_bg_color);
        self
    }

    pub fn set_split_text_blod(&mut self, split_text_blod: bool) -> &mut Self {
        self.output.split_text_blod = Some(split_text_blod);
        self
    }

    pub fn build(&self) -> Output {
        self.output.clone()
    }

    pub fn output(&self) {
        output(self.output.clone());
    }
}
