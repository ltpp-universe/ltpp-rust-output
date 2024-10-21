use crate::{ColorType, Output};

use crate::output;

impl<'a> Default for Output<'a> {
    fn default() -> Self {
        Output {
            text: "",
            text_color: Some(ColorType::default()),
            text_bg_color: Some(ColorType::default()),
            show_time: Some(false),
            time_text_color: Some(ColorType::default()),
            time_bg_color: Some(ColorType::default()),
            split: Some(""),
            split_color: Some(ColorType::default()),
            split_bg_color: Some(ColorType::default()),
            text_blod: Some(false),
            time_text_blod: Some(false),
            split_text_blod: Some(false),
        }
    }
}

impl<'a> Output<'a> {
    /// Outputs
    ///
    /// # Returns
    /// - `()` : Nothing is returned.
    pub fn output(self) {
        output(self);
    }
}
