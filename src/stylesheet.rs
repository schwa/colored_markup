use anyhow::{anyhow, Ok, Result};
use colored::ColoredString;
use colored::Colorize;
use std::collections::HashMap;

use crate::markup::*;
use crate::styles::*;
use crate::stylesheet_parse::parse;

#[derive(Debug, PartialEq)]
pub struct StyleSheet<'a> {
    styles: HashMap<&'a str, Style>,
}

impl<'a> Default for StyleSheet<'a> {
    fn default() -> StyleSheet<'a> {
        let styles = vec![
            ("bold", Style::new(Some(vec![Styles::Bold]), None, None)),
            ("em", Style::new(Some(vec![Styles::Italic]), None, None)),
            (
                "strikethrough",
                Style::new(Some(vec![Styles::Strikethrough]), None, None),
            ),
        ];
        StyleSheet::new_internal(&styles)
    }
}

impl<'a> StyleSheet<'a> {
    pub fn new(
        styles: &[(
            &'a str,
            Vec<Styles>,
            Option<colored::Color>,
            Option<colored::Color>,
        )],
    ) -> StyleSheet<'a> {
        let styles = styles.iter().map(|(name, styles, foreground, background)| {
            let style = Style::new(Some(styles.clone()), *foreground, *background);
            (*name, style)
        });
        let styles = HashMap::from_iter(styles);
        StyleSheet { styles }
    }
}

impl<'a> StyleSheet<'a> {
    pub(crate) fn new_internal(styles: &[(&'a str, Style)]) -> StyleSheet<'a> {
        let styles = HashMap::from_iter(styles.iter().cloned());
        StyleSheet { styles }
    }
}

impl<'a> StyleSheet<'a> {
    pub fn parse(s: &'a str) -> Result<StyleSheet<'a>> {
        let rules = parse(s)?;
        Ok(StyleSheet::new_internal(&rules))
    }
}

#[test]
fn test_stylesheet() {
    let styles = vec![("alert", Style::new(None, Some(colored::Color::Red), None))];
    let expectation = StyleSheet::new_internal(&styles);
    assert_eq!(
        StyleSheet::parse("alert{foreground:red}").unwrap(),
        expectation
    );
}

impl<'a> StyleSheet<'a> {
    pub fn render(&self, markup: &str) -> Result<String> {
        let parts = parse_markup(markup)?;

        let mut style_stack: Vec<Style> = Vec::new();

        let mut colored_strings: Vec<colored::ColoredString> = Vec::new();

        for part in parts {
            match part {
                Part::Text(text) => {
                    let style = Style::resolve(&style_stack);
                    let mut text = ColoredString::from(text);

                    for style in style.styles {
                        text = style.apply(text);
                    }

                    if let Some(color) = style.foreground {
                        text = text.color(color);
                    }
                    if let Some(color) = style.background {
                        text = text.on_color(color);
                    }
                    colored_strings.push(text);
                }
                Part::OpenTag(tag) => {
                    if let Some(style) = self.styles.get(tag) {
                        style_stack.push(style.clone());
                    } else {
                        style_stack.push(Style::default());
                    }
                }
                Part::CloseTag(_) => {
                    style_stack
                        .pop()
                        .ok_or_else(|| anyhow!("Invalid template"))?; // TODO: error
                }
            }
        }

        let mut result = String::new();
        for colored_string in colored_strings {
            let f = format!("{}", colored_string);
            result.push_str(&f);
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_styles_template() {
        let template = StyleSheet {
            styles: HashMap::new(),
        };
        let result = template.render("Hello <bold>World</bold><em></em>!");
        assert_eq!(result.unwrap(), "Hello World!");
    }

    // TODO: Disable because this fails in github. Need to force color output.
    // #[test]
    // fn test_template() {
    //     let template = Template::default();
    //     let result = template.render("<em>EM <bold>BOLD</bold>EM</em>").unwrap();
    //     //println!("{}", result);
    //     assert_eq!(
    //         result,
    //         "\u{1b}[3mEM \u{1b}[0m\u{1b}[1;3mBOLD\u{1b}[0m\u{1b}[3mEM\u{1b}[0m"
    //     );
    // }
}
