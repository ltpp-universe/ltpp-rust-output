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

impl<'a> Default for Output<'a> {
    fn default() -> Self {
        Output {
            text: "",
            text_color: Some(ColorType::default()),
            text_bg_color: Some(ColorType::default()),
            show_time: Some(true),
            time_text_color: Some(ColorType::default()),
            time_bg_color: Some(ColorType::default()),
            split: Some(""),
            split_color: Some(ColorType::default()),
            split_bg_color: Some(ColorType::default()),
            text_blod: Some(true),
            time_text_blod: Some(true),
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
    /// Creates the struct
    ///
    /// # Returns
    /// - `Self`: Output
    pub fn new() -> Self {
        Self {
            output: Output::default(),
        }
    }

    /// Sets the text.
    ///
    /// # Parameters
    /// - `text`: The text to be set.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_text(&mut self, text: &'a str) -> &mut Self {
        self.output.text = text;
        self
    }

    /// Sets the text color.
    ///
    /// # Parameters
    /// - `text_color`: The color to be set for the text.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_text_color(&mut self, text_color: ColorType) -> &mut Self {
        self.output.text_color = Some(text_color);
        self
    }

    /// Sets the background color for the text.
    ///
    /// # Parameters
    /// - `text_bg_color`: The background color to be set for the text.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_text_bg_color(&mut self, text_bg_color: ColorType) -> &mut Self {
        self.output.text_bg_color = Some(text_bg_color);
        self
    }

    /// Sets whether the text should be bold.
    ///
    /// # Parameters
    /// - `text_blod`: A boolean indicating whether the text should be bold.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_text_blod(&mut self, text_blod: bool) -> &mut Self {
        self.output.text_blod = Some(text_blod);
        self
    }

    /// Sets whether to show the time.
    ///
    /// # Parameters
    /// - `show_time`: A boolean indicating whether to display the time.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_show_time(&mut self, show_time: bool) -> &mut Self {
        self.output.show_time = Some(show_time);
        self
    }

    /// Sets the time text color.
    ///
    /// # Parameters
    /// - `time_text_color`: The color to be set for the time text.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_time_text_color(&mut self, time_text_color: ColorType) -> &mut Self {
        self.output.time_text_color = Some(time_text_color);
        self
    }

    /// Sets the background color for the time text.
    ///
    /// # Parameters
    /// - `time_bg_color`: The background color to be set for the time text.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_time_bg_color(&mut self, time_bg_color: ColorType) -> &mut Self {
        self.output.time_bg_color = Some(time_bg_color);
        self
    }

    /// Sets whether the time text should be bold.
    ///
    /// # Parameters
    /// - `time_text_blod`: A boolean indicating whether the time text should be bold.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_time_text_blod(&mut self, time_text_blod: bool) -> &mut Self {
        self.output.time_text_blod = Some(time_text_blod);
        self
    }

    /// Sets the separator.
    ///
    /// # Parameters
    /// - `split`: The separator string to be set.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_split(&mut self, split: &'a str) -> &mut Self {
        self.output.split = Some(split);
        self
    }

    /// Sets the separator color.
    ///
    /// # Parameters
    /// - `split_color`: The color to be set for the separator.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_split_color(&mut self, split_color: ColorType) -> &mut Self {
        self.output.split_color = Some(split_color);
        self
    }

    /// Sets the background color for the separator.
    ///
    /// # Parameters
    /// - `split_bg_color`: The background color to be set for the separator.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_split_bg_color(&mut self, split_bg_color: ColorType) -> &mut Self {
        self.output.split_bg_color = Some(split_bg_color);
        self
    }

    /// Sets whether the separator text should be bold.
    ///
    /// # Parameters
    /// - `split_text_blod`: A boolean indicating whether the separator text should be bold.
    ///
    /// # Returns
    /// - `&mut Self`: A mutable reference to the struct for method chaining.
    pub fn set_split_text_blod(&mut self, split_text_blod: bool) -> &mut Self {
        self.output.split_text_blod = Some(split_text_blod);
        self
    }

    /// Builds and returns the Output struct.
    ///
    /// # Returns
    /// - `Output`: The constructed Output struct.
    pub fn build(&self) -> Output {
        self.output.clone()
    }

    /// Outputs the current state of the Output struct.
    ///
    /// # Returns
    /// - `()` : Nothing is returned.
    pub fn output(&self) {
        output(self.output.clone());
    }
}
