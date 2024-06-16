use colored::Colorize;

/// A representation of a terminal color.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    TrueColor { r: u8, g: u8, b: u8 },
}

impl From<Color> for colored::Color {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => colored::Color::Black,
            Color::Red => colored::Color::Red,
            Color::Green => colored::Color::Green,
            Color::Yellow => colored::Color::Yellow,
            Color::Blue => colored::Color::Blue,
            Color::Magenta => colored::Color::Magenta,
            Color::Cyan => colored::Color::Cyan,
            Color::White => colored::Color::White,
            Color::BrightBlack => colored::Color::BrightBlack,
            Color::BrightRed => colored::Color::BrightRed,
            Color::BrightGreen => colored::Color::BrightGreen,
            Color::BrightYellow => colored::Color::BrightYellow,
            Color::BrightBlue => colored::Color::BrightBlue,
            Color::BrightMagenta => colored::Color::BrightMagenta,
            Color::BrightCyan => colored::Color::BrightCyan,
            Color::BrightWhite => colored::Color::BrightWhite,
            Color::TrueColor { r, g, b } => colored::Color::TrueColor { r, g, b },
        }
    }
}

use bitflags::bitflags;


/// A representation of a terminal style attribute.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct Attributes(u8);

bitflags! {
    impl Attributes: u8 {
        const Bold =          0b00000001;
        const Dimmed =        0b00000010;
        const Underline =     0b00000100;
        const Reversed =      0b00001000;
        const Italic =        0b00010000;
        const Blink =         0b00100000;
        const Hidden =        0b01000000;
        const Strikethrough = 0b10000000;
    }
}

impl Attributes {
    pub(crate) fn apply(self, s: colored::ColoredString) -> colored::ColoredString {
        match self {
            Attributes::Bold => s.bold(),
            Attributes::Dimmed => s.dimmed(),
            Attributes::Underline => s.underline(),
            Attributes::Reversed => s.reversed(),
            Attributes::Italic => s.italic(),
            Attributes::Blink => s.blink(),
            Attributes::Hidden => s.hidden(),
            Attributes::Strikethrough => s.strikethrough(),
            _ => s,
        }
    }
}

impl Attributes {
    // TODO: use correct from trait.
    pub(crate) fn from_vec(attributes: Vec<Attributes>) -> Attributes {
        let mut result = Attributes::default();
        for style in attributes {
            result |= style;
        }
        result
    }
}

#[test]
fn test_attributes() {
    assert_eq!(Attributes::Bold.apply("hello".into()), "hello".bold());
    assert_eq!(Attributes::Dimmed.apply("hello".into()), "hello".dimmed());
    assert_eq!(Attributes::Underline.apply("hello".into()), "hello".underline());
    assert_eq!(Attributes::Reversed.apply("hello".into()), "hello".reversed());
    assert_eq!(Attributes::Italic.apply("hello".into()), "hello".italic());
    assert_eq!(Attributes::Blink.apply("hello".into()), "hello".blink());
    assert_eq!(Attributes::Hidden.apply("hello".into()), "hello".hidden());
    assert_eq!(
        Attributes::Strikethrough.apply("hello".into()),
        "hello".strikethrough()
    );
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct Style {
    pub(crate) attributes: Attributes,
    pub(crate) foreground: Option<Color>,
    pub(crate) background: Option<Color>,
}

impl Style {
    pub(crate) fn new(
        attributes: Option<Attributes>,
        foreground: Option<Color>,
        background: Option<Color>,
    ) -> Style {
        Style {
            attributes: attributes.unwrap_or_default(),
            foreground,
            background,
        }
    }

    pub(crate) fn merge(&self, other: Style) -> Style {
        let mut attributes = self.attributes;
        attributes.extend(other.attributes);
        Style {
            attributes,
            foreground: other.foreground.or(self.foreground),
            background: other.background.or(self.background),
        }
    }

    pub(crate) fn resolve(stack: &Vec<Style>) -> Style {
        let mut attributes: Attributes = Attributes::default();
        let mut foreground: Option<Color> = None;
        let mut background: Option<Color> = None;

        for style in stack {
            attributes.extend(style.attributes.iter());
            if style.foreground.is_some() {
                foreground = style.foreground;
            }
            if style.background.is_some() {
                background = style.background;
            }
        }
        Style {
            attributes,
            foreground,
            background,
        }
    }
}
