# colored_markup

A rust library for parsing and rendering coloured markup with CSS style rules.

## Usage

```rust
use colored_markup::{println_markup, StyleSheet};

let style_sheet =
    StyleSheet::parse("red { foreground: bright-red; styles: underline }").unwrap();
println_markup!(&style_sheet, "The next word is <red>{}</red>", "red");
```

See [`examples`](examples) for more.

## License

MIT. See [`LICENSE.txt`](LICENSE.txt) for details.

## TODO

* RGB colour codes in CSS.
* Better CSS parsing error handling.
* Allow changing markup characters.
* CLI tool.
* Better documentation.
* Get rid of re-exports?
