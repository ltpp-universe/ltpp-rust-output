/// Generates an ANSI escape sequence for foreground colors in 256 colors.
///
/// # Parameters
/// - `code`: The color code, which ranges from 0 to 255.
///
/// # Returns
/// - `String`: Returns a string containing the ANSI escape sequence for the foreground color.
pub fn color256_fg_color(code: u32) -> String {
    format!("\x1b[38;5;{}m", code)
}

/// Generates an ANSI escape sequence for background colors in 256 colors.
///
/// # Parameters
/// - `code`: The color code, which ranges from 0 to 255.
///
/// # Returns
/// - `String`: Returns a string containing the ANSI escape sequence for the background color.
pub fn color256_bg_color(code: u32) -> String {
    format!("\x1b[48;5;{}m", code)
}

/// Generates an ANSI escape sequence for true color foreground colors.
///
/// # Parameters
/// - `r`: The red component, which ranges from 0 to 255.
/// - `g`: The green component, which ranges from 0 to 255.
/// - `b`: The blue component, which ranges from 0 to 255.
///
/// # Returns
/// - `String`: Returns a string containing the ANSI escape sequence for the true color foreground color.
pub fn rgb_fg_color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

/// Generates an ANSI escape sequence for true color background colors.
///
/// # Parameters
/// - `r`: The red component, which ranges from 0 to 255.
/// - `g`: The green component, which ranges from 0 to 255.
/// - `b`: The blue component, which ranges from 0 to 255.
///
/// # Returns
/// - `String`: Returns a string containing the ANSI escape sequence for the true color background color.
pub fn rgb_bg_color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{};{};{}m", r, g, b)
}
