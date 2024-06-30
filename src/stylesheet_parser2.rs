
use chumsky::prelude::*;

#[derive(Debug)]
struct Rule<'a> {
    selector: Selector<'a>,
    properties: Vec<Property<'a>>,
}

#[derive(Debug)]
struct Property<'a> {
    name: Identifier<'a>,
    values: Vec<Value<'a>>,
}

#[derive(Debug)]
struct Selector<'a> {
    name: &'a str,
}


#[derive(Debug)]
struct Identifier<'a> {
    name: &'a str,
}

#[derive(Debug)]
struct Value<'a> {
    value: &'a str,
}

// https://github.com/zesterer/chumsky/tree/0.9
// https://github.com/zesterer/chumsky/blob/0.9/examples/foo.rs
// https://docs.rs/chumsky/1.0.0-alpha.7/chumsky/index.html

// selector { idenrfieri: value; idenrfier: value value; }

fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Rule<'a>>> {
    let identifier = text::ascii::ident()
    .padded()
    .map(|s| Identifier { name: s });

    let selector = text::ascii::ident()
        .padded()
        .map(|s| Selector { name: s });

    let value = text::ascii::ident().padded().map(|s| Value { value: s });

    let values = value.repeated().at_least(1).collect();

    let property = identifier
        .clone()
        .then_ignore(just(':').padded())
        .then(values)
        .map(|p| Property {
            name: p.0,
            values: p.1,
        });

    let properties = property.repeated().collect();

    let rule = selector
        .clone()
        .then_ignore(just('{').padded())
        .then(properties)
        .then_ignore(just('}').padded())
        .map(|r| Rule {
            selector: r.0,
            properties: r.1,
        });

    let rules = rule.repeated().collect();

    return rules;
}

// fn main() {
//     let src = "
//     selector { }
//     selector { property: value }
//     selector { property: value1 value2 }
//     ";

//     match parser().parse(src).into_result() {
//         Ok(ast) => println!("{:#?}", ast),
//         Err(errs) => errs.into_iter().for_each(|e| println!("{:?}", e)),
//     };

// }
