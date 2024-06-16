use anyhow::{anyhow, Result};
use clap::Parser;
use colored::control::set_override;
use colored_markup::StyleSheet;
use std::path::PathBuf;

/// TODO
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Css-like style sheet.
    #[arg(long)]
    styles: Option<String>,

    /// Path to a css-like style sheet file.
    #[arg(long)]
    styles_path: Option<PathBuf>,

    /// Do not output a newline.
    #[arg(short = 'n', long, default_value_t = false)]
    no_newline: bool,

    /// Do not separate arguments with spaces.
    #[arg(short = 's', long, default_value_t = false)]
    no_spaces: bool,

    /// Force color output.
    #[arg(short, long, default_value_t = false)]
    force_color: bool,

    /// Print the style sheet instead of input text..
    #[arg(long, default_value_t = false)]
    print_stylesheet: bool,

    text: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let styles: String;
    if args.styles.is_some() && args.styles_path.is_some() {
        return Err(anyhow!("Cannot specify both --styles and --styles-path"));
    } else if let Some(styles_path) = args.styles_path {
        styles = std::fs::read_to_string(styles_path)?;
    } else if let Some(styles_) = args.styles {
        styles = styles_;
    } else {
        let default_stylesheet = "
        blue { foreground: blue }
        cyan { foreground: cyan }
        green { foreground: green }
        magenta { foreground: magenta }
        red { foreground: red }
        white { foreground: white }
        yellow { foreground: yellow }

        bright-blue { foreground: bright-blue }
        bright-cyan { foreground: bright-cyan }
        bright-green { foreground: bright-green }
        bright-magenta { foreground: bright-magenta }
        bright-red { foreground: bright-red }
        bright-white { foreground: bright-white }
        bright-yellow { foreground: bright-yellow }

        bg-blue { background: blue }
        bg-cyan { background: cyan }
        bg-green { background: green }
        bg-magenta { background: magenta }
        bg-red { background: red }
        bg-white { background: white }
        bg-yellow { background: yellow }

        bright-bg-blue { background: bright-blue }
        bright-bg-cyan { background: bright-cyan }
        bright-bg-green { background: bright-green }
        bright-bg-magenta { background: bright-magenta }
        bright-bg-red { background: bright-red }
        bright-bg-white { background: bright-white }
        bright-bg-yellow { background: bright-yellow }

        bold { styles: bold }
        dimmed { styles: dimmed }
        underline { styles: underline }
        reversed { styles: reversed }
        italic { styles: italic }
        blink { styles: blink }
        ";

        styles = default_stylesheet.to_string();
    }

    if args.force_color {
        set_override(true);
    }

    let style_sheet = StyleSheet::parse(&styles)?;

    if args.print_stylesheet {
        if !args.text.is_empty() {
            return Err(anyhow!("Cannot specify text with --print-stylesheet"));
        }
        print!("{}", style_sheet);
        return Ok(());
    }

    let text = args.text.join(if args.no_spaces { "" } else { " " });
    let output = style_sheet.render(&text)?;
    print!("{}{}", output, if args.no_newline { "" } else { "\n" });

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use assert_cmd::Command;

//     #[test]
//     fn test_cli() {
//         Command::new("cargo").args(["build"]).assert();
//         let mut cmd = Command::cargo_bin(assert_cmd::crate_name!()).unwrap();
//         let output = cmd
//             .args(["--force-color", "Hello <red>world</red>"])
//             .unwrap();

//         let stdout = std::str::from_utf8(&output.stdout).unwrap();
//         assert!(stdout == "Hello \u{1b}[4;91mworld\u{1b}[0m\n");
//     }
// }
