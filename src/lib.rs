pub mod markup;
pub mod styles;
pub mod stylesheet;
pub mod stylesheet_parse;

pub use styles::*;
pub use stylesheet::*;

#[macro_export]
macro_rules! format_markup {
    ($stylesheet:expr, $($arg:tt)*) => {{
        let stylesheet: &StyleSheet = $stylesheet;
        let s = format!($($arg)*);
        stylesheet.render(&s).unwrap()
    }};
}

#[test]
fn test_format_markup() {
    let stylesheet = StyleSheet::parse("red { foreground: red }").unwrap();
    let result = format_markup!(&stylesheet, "Mode: <red>mode</red>");
    assert_eq!(result, "Mode: \u{1b}[31mmode\u{1b}[0m");
}

#[macro_export]
macro_rules! println_markup {
    ($stylesheet:expr, $($arg:tt)*) => {{
        println!("{}", format_markup!($stylesheet, $($arg)*));
    }};
}

#[macro_export]
macro_rules! eprintln_markup {
    ($stylesheet:expr, $($arg:tt)*) => {{
        eprintln!("{}", format_markup!($stylesheet, $($arg)*));
    }};
}

// let template = "Mode: <mode>{{mode}}</mode>
// Mean: <speed>{{mean}}</speed>/sec, Median: <speed>{{median}}</speed>/sec, Standard Deviation Ø: <speed>{{standard_deviation}}</speed>/sec
// Min: <speed>{{min}}</speed>/sec, Max: <speed>{{max}}</speed>/sec";
//         let context = context! {
//             mode => self.mode.to_string(),
//             mean => DataSize::from(self.statistics.mean).to_human_string(),
//             median => DataSize::from(self.statistics.median).to_human_string(),
//             standard_deviation => DataSize::from(self.statistics.standard_deviation).to_human_string(),
//             min => DataSize::from(self.statistics.min).to_human_string(),
//             max => DataSize::from(self.statistics.max).to_human_string(),
//         };

// fn render(template: &str, context: &minijinja::value::Value) -> anyhow::Result<()> {
//     let style_sheet = StyleSheet::parse(
//         "
//         info { foreground: yellow }
//         mode { foreground: red }
//         speed { foreground: cyan }
//         size { foreground: green }
//         num { foreground: yellow }
//         ",
//     )
//     .expect("Failed to parse stylesheet.");

//     let mut env = Environment::new();
//     env.add_template("template", template).unwrap();
//     let tmpl = env.get_template("template").unwrap();
//     let render = tmpl.render(context).unwrap();
//     println!("{}", style_sheet.render(&render)?);

//     Ok(())
// }
