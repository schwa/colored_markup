use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub(crate) enum Part<'a> {
    OpenTag(&'a str),
    CloseTag(&'a str),
    Text(&'a str),
}

pub(crate) fn parse_markup(string: &str) -> Result<Vec<Part>> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(
            r"(?x)
            (?P<tag><
                (?:(?P<open>[a-z]+)|/(?P<close>[a-z]+))
            >)"
        )
        .unwrap();
    }
    let mut parts: Vec<Part> = Vec::new();
    let mut current_index: usize = 0;
    while let Some(captures) = REGEX.captures_at(string, current_index) {
        if let Some(tag) = captures.name("tag") {
            let text = &string[current_index..tag.start()];
            if !text.is_empty() {
                parts.push(Part::Text(text));
            }
            current_index = tag.end();
            if let Some(open) = captures.name("open") {
                parts.push(Part::OpenTag(open.as_str()));
            } else if let Some(close) = captures.name("close") {
                parts.push(Part::CloseTag(close.as_str()));
            }
        }
    }
    let text = &string[current_index..];
    if !text.is_empty() {
        parts.push(Part::Text(text));
    }
    Ok(parts)
}

#[test]
fn test_parse_markup() {
    let parts = parse_markup("Hello <bold>World</bold><em></em>!").unwrap();
    let expectation = vec![
        Part::Text("Hello "),
        Part::OpenTag("bold"),
        Part::Text("World"),
        Part::CloseTag("bold"),
        Part::OpenTag("em"),
        Part::CloseTag("em"),
        Part::Text("!"),
    ];
    assert_eq!(parts, expectation);
}
