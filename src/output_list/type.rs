use crate::Output;

/// A structure representing a list of outputlist.
#[derive(Debug, Clone)]
pub struct OutputList<'a>(pub Vec<Output<'a>>);
