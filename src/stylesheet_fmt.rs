use crate::*;
use std::fmt;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = match self {
            Color::Black => "black",
            Color::Red => "red",
            Color::Green => "green",
            Color::Yellow => "yellow",
            Color::Blue => "blue",
            Color::Magenta => "magenta",
            Color::Cyan => "cyan",
            Color::White => "white",
            Color::BrightBlack => "bright-black",
            Color::BrightRed => "bright-red",
            Color::BrightGreen => "bright-green",
            Color::BrightYellow => "bright-yellow",
            Color::BrightBlue => "bright-blue",
            Color::BrightMagenta => "bright-magenta",
            Color::BrightCyan => "bright-cyan",
            Color::BrightWhite => "bright-white",
            Color::TrueColor { r, g, b } => {
                return write!(f, "true-color: {}, {}, {}", r, g, b);
            }
        };
        write!(f, "{}", color)
    }
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut attributes = Vec::new();
        if self.contains(Attributes::Bold) {
            attributes.push("bold");
        }
        if self.contains(Attributes::Dimmed) {
            attributes.push("dimmed");
        }
        if self.contains(Attributes::Underline) {
            attributes.push("underline");
        }
        if self.contains(Attributes::Reversed) {
            attributes.push("reversed");
        }
        if self.contains(Attributes::Italic) {
            attributes.push("italic");
        }
        if self.contains(Attributes::Blink) {
            attributes.push("blink");
        }
        if self.contains(Attributes::Hidden) {
            attributes.push("hidden");
        }
        if self.contains(Attributes::Strikethrough) {
            attributes.push("strikethrough");
        }

        write!(f, "{}", attributes.join(" "))
    }
}

impl<'a> fmt::Display for StyleSheet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut style_str = String::new();
        for (name, style) in self.styles.iter() {
            style_str.push_str(&format!("{} {{ ", name));
            if let Some(foreground) = style.foreground {
                style_str.push_str(&format!("foreground: {}; ", foreground));
            }
            if let Some(background) = style.background {
                style_str.push_str(&format!("background: {}; ", background));
            }
            if !style.attributes.is_empty() {
                style_str.push_str("styles: ");
                style_str.push_str(&style.attributes.to_string());
                style_str.push_str("; ");
            }
            style_str.push_str("}\n");
        }
        write!(f, "{}", style_str)
    }
}
