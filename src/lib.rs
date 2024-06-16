//! colored_markup is a library for rendering HTML-like markup with ANSI escape codes using CSS-like stylesheets.
//!```rust
//! use colored_markup::{println_markup, StyleSheet};
//!
//! let style_sheet = StyleSheet::parse("red { foreground: bright-red; styles: underline }").unwrap();
//! println_markup!(&style_sheet, "The next word is <red>{}</red>", "red");
//! ```

mod markup;
pub mod styles;
pub mod stylesheet;
mod stylesheet_fmt;
mod stylesheet_parse;

use styles::*;
pub use stylesheet::*;

/// A macro to format a string with markup.
#[macro_export]
macro_rules! format_markup {
    ($stylesheet:expr, $($arg:tt)*) => {{
        let stylesheet: &StyleSheet = $stylesheet;
        let s = format!($($arg)*);
        stylesheet.render(&s).unwrap()
    }};
}

#[test]
fn test_format_markup() {
    let stylesheet = StyleSheet::parse("red { foreground: red }").unwrap();
    let result = format_markup!(&stylesheet, "Mode: <red>mode</red>");
    assert_eq!(result, "Mode: \u{1b}[31mmode\u{1b}[0m");
}

/// A macro to print a string with markup.
#[macro_export]
macro_rules! println_markup {
    ($stylesheet:expr, $($arg:tt)*) => {{
        println!("{}", colored_markup::format_markup!($stylesheet, $($arg)*));
    }};
}

/// A macro to print a string to stderr with markup.
#[macro_export]
macro_rules! eprintln_markup {
    ($stylesheet:expr, $($arg:tt)*) => {{
        eprintln!("{}", colored_markup::format_markup!($stylesheet, $($arg)*));
    }};
}

/// A trait for rendering markup.
pub trait Styled {
    /// Render the string with the given stylesheet.
    fn styled(&self, stylesheet: &StyleSheet) -> String;
}

impl Styled for str {
    fn styled(&self, stylesheet: &StyleSheet) -> String {
        stylesheet.render(self).unwrap()
    }
}

#[test]
fn test_styled() {
    let stylesheet: StyleSheet<'_> = StyleSheet::parse("red { foreground: red }").unwrap();
    let result = "Mode: <red>mode</red>".styled(&stylesheet);
    assert_eq!(result, "Mode: \u{1b}[31mmode\u{1b}[0m");
}

impl Styled for String {
    fn styled(&self, stylesheet: &StyleSheet) -> String {
        stylesheet.render(self).unwrap()
    }
}

#[test]
fn test_styled_2() {
    let stylesheet: StyleSheet<'_> = StyleSheet::parse("red { foreground: red }").unwrap();
    let result = "Mode: <red>mode</red>".to_string().styled(&stylesheet);
    assert_eq!(result, "Mode: \u{1b}[31mmode\u{1b}[0m");
}
