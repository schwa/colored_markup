use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub(crate) enum Part<'a> {
    OpenTag(&'a str),
    CloseTag(&'a str),
    Text(&'a str),
}

pub(crate) struct Markup<'a> {
    pub(crate) parts: Vec<Part<'a>>,
}

impl<'a> Markup<'a> {
    pub(crate) fn parse(string: &'a str) -> Result<Self> {
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

        Ok(Markup { parts })
    }

    #[cfg(test)]
    pub(crate) fn is_valid(&self) -> bool {
        let mut tags: Vec<&'a str> = Vec::new();
        for part in &self.parts {
            match part {
                Part::OpenTag(tag) => {
                    tags.push(tag);
                }
                Part::CloseTag(tag) => {
                    if tags.is_empty() {
                        return false;
                    }
                    let top = tags.pop().unwrap();
                    if top != *tag {
                        return false;
                    }
                }
                Part::Text(_) => {}
            }
        }

        true
    }
}

#[test]
fn test_parse_markup() {
    let markup = Markup::parse("Hello <bold>World</bold><em></em>!").unwrap();
    assert!(markup.is_valid());
    let parts = markup.parts;
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

#[test]
fn test_negative() {
    assert!(!Markup::parse("</oops>").unwrap().is_valid());
    assert!(!Markup::parse("<foo></bar>").unwrap().is_valid());
}
