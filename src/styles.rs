use colored::Colorize;

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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Styles {
    Bold,
    Dimmed,
    Underline,
    Reversed,
    Italic,
    Blink,
    Hidden,
    Strikethrough,
}

impl Styles {
    pub(crate) fn apply(&self, s: colored::ColoredString) -> colored::ColoredString {
        match self {
            Styles::Bold => s.bold(),
            Styles::Dimmed => s.dimmed(),
            Styles::Underline => s.underline(),
            Styles::Reversed => s.reversed(),
            Styles::Italic => s.italic(),
            Styles::Blink => s.blink(),
            Styles::Hidden => s.hidden(),
            Styles::Strikethrough => s.strikethrough(),
        }
    }
}

#[test]
fn test_styles() {
    assert_eq!(Styles::Bold.apply("hello".into()), "hello".bold());
    assert_eq!(Styles::Dimmed.apply("hello".into()), "hello".dimmed());
    assert_eq!(Styles::Underline.apply("hello".into()), "hello".underline());
    assert_eq!(Styles::Reversed.apply("hello".into()), "hello".reversed());
    assert_eq!(Styles::Italic.apply("hello".into()), "hello".italic());
    assert_eq!(Styles::Blink.apply("hello".into()), "hello".blink());
    assert_eq!(Styles::Hidden.apply("hello".into()), "hello".hidden());
    assert_eq!(
        Styles::Strikethrough.apply("hello".into()),
        "hello".strikethrough()
    );
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct Style {
    pub(crate) styles: Vec<Styles>, // TODO: Hashset
    pub(crate) foreground: Option<Color>,
    pub(crate) background: Option<Color>,
}

impl Style {
    pub(crate) fn new(
        styles: Option<Vec<Styles>>,
        foreground: Option<Color>,
        background: Option<Color>,
    ) -> Style {
        Style {
            styles: styles.unwrap_or(Vec::new()),
            foreground,
            background,
        }
    }

    pub(crate) fn merge(&self, other: Style) -> Style {
        let mut styles = self.styles.clone();
        styles.extend(other.styles);
        Style {
            styles,
            foreground: other.foreground.or(self.foreground),
            background: other.background.or(self.background),
        }
    }

    pub(crate) fn resolve(stack: &Vec<Style>) -> Style {
        let mut styles: Vec<Styles> = Vec::new();
        let mut foreground: Option<Color> = None;
        let mut background: Option<Color> = None;

        for style in stack {
            styles.extend(style.styles.iter());
            if style.foreground.is_some() {
                foreground = style.foreground;
            }
            if style.background.is_some() {
                background = style.background;
            }
        }
        Style {
            styles,
            foreground,
            background,
        }
    }
}
