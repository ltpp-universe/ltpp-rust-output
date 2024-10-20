#![allow(warnings)]
mod color;
mod r#macro;
mod output;
mod text;
mod time;
pub use color::r#type::*;
pub use output::{output::*, r#type::*};
pub use r#macro::proc_macro;
