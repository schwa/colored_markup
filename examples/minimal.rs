use colored_markup::{println_markup, StyleSheet};

fn main() {
    let style_sheet =
        StyleSheet::parse("red { foreground: bright-red; styles: underline }").unwrap();
    println_markup!(&style_sheet, "The next word is <red>{}</red>", "red");
}
