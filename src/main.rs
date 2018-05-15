extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "ident.pest"]
struct IdentParser;

fn main() {
    let pairs = IdentParser::parse(Rule::expression, "123+4*5+1*2*3").unwrap_or_else(|e| panic!("{}", e));
    println!("{}", pairs);

    for pair in pairs {

        for inner_pair in pair.into_inner() {
            let inner_span = inner_pair.clone().into_span();
            match inner_pair.as_rule() {
                Rule::number => println!("Number:   {}", inner_span.as_str()),
                Rule::digit_non_zero => println!("Digit:   {}", inner_span.as_str()),
                Rule::digit => println!("Digit-nz:   {}", inner_span.as_str()),
                Rule::addsub_exp => println!("AddSub:   {}", inner_span.as_str()),
                Rule::expression => println!("Expression:   {}", inner_span.as_str()),
                _ => {
                    println!("Unreachable:   {}", inner_span.as_str());
                    unreachable!()
                }
            };
        }
    }
}
