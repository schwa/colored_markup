use colored_markup::*;

fn main() {
    let style_sheet =
        StyleSheet::parse("red { foreground: bright-red; styles: underline }").unwrap();
    println_markup!(&style_sheet, "The next word is <red>red</red>");

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
