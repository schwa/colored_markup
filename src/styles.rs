use colored::Colorize;

pub type Color = colored::Color;

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

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct Style {
    pub(crate) styles: Vec<Styles>, // TODO: Hashset
    pub(crate) foreground: Option<colored::Color>,
    pub(crate) background: Option<colored::Color>,
}

impl Style {
    pub(crate) fn new(
        styles: Option<Vec<Styles>>,
        foreground: Option<colored::Color>,
        background: Option<colored::Color>,
    ) -> Style {
        Style {
            styles: styles.unwrap_or(Vec::new()),
            foreground,
            background,
        }
    }

    pub(crate) fn empty() -> Style {
        Style {
            styles: Vec::new(),
            foreground: None,
            background: None,
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
        let mut foreground: Option<colored::Color> = None;
        let mut background: Option<colored::Color> = None;

        for style in stack {
            styles.extend(style.styles.iter());
            if style.foreground.is_some() {
                foreground = style.foreground;
            }
            if style.background.is_some() {
                background = style.background;
            }
        }

        // styles = styles
        //     .iter()
        //     .unique_by(|s| s.to_string())
        //     .cloned()
        //     .collect();

        Style {
            styles,
            foreground,
            background,
        }
    }
}
