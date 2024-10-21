use crate::Output;

/// A structure representing a list of outputs.
#[derive(Debug, Clone)]
pub struct OutputListBuilder<'a> {
    /// A list of output structures.
    pub(crate) output_list: Vec<Output<'a>>,
}
