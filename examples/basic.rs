use colored_markup::{
    println_markup,
    styles::{Attributes, Color},
    stylesheet::StyleSheet,
    Styled,
};

fn main() {
    let style_sheet =
        StyleSheet::parse("red { foreground: bright-red; styles: underline }").unwrap();

    println!("{}", "Red ones go <red>faster</red>".styled(&style_sheet));

    let style_sheet = StyleSheet::new(&[
        (
            "red",
            Attributes::Bold,
            Some(Color::Red),
            Some(Color::Yellow),
        ),
        ("green", Attributes::Underline, Some(Color::Green), None),
        ("blue", Attributes::Strikethrough, Some(Color::Blue), None),
    ]);
    println_markup!(
        &style_sheet,
        "<red>red</red>, <green>green</green>, <blue>blue</blue>"
    );
}
