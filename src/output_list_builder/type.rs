use crate::Output;

/// A structure representing a list of outputs.
#[derive(Debug, Clone)]
pub struct OutputList<'a> {
    /// A list of output structures.
    pub(crate) output_list: Vec<Output<'a>>,
}
