/// Output macro
///
/// [Official Documentation](https://docs.ltpp.vip/LTPP-RUST-OUTPUT/),
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
/// ```
///
/// ## Using the Constructor
///
/// ```rust
/// use ltpp_output::*;
/// ```
///
/// ## Multiple Inputs
///
/// ```rust
/// use ltpp_output::*;
/// ```
#[macro_export]
macro_rules! output_macro {
    ($($output:expr),*) => {
        $(
            $output.output();
        )*
    };
}
