use colored_markup::{
    println_markup,
    styles::{Color, Styles},
    stylesheet::StyleSheet,
    Styled,
};

fn main() {
    let style_sheet =
        StyleSheet::parse("red { foreground: bright-red; styles: underline }").unwrap();

    println!("{}", "Red ones go <red>faster</red>".styled(&style_sheet));

    println_markup!(&style_sheet, "The next word is <red>{}</red>", "red");

    let style_sheet = StyleSheet::new(&[
        (
            "red",
            vec![Styles::Bold],
            Some(Color::Red),
            Some(Color::Yellow),
        ),
        ("green", vec![Styles::Underline], Some(Color::Green), None),
        ("blue", vec![Styles::Strikethrough], Some(Color::Blue), None),
    ]);
    println_markup!(
        &style_sheet,
        "<red>red</red>, <green>green</green>, <blue>blue</blue>"
    );
}
