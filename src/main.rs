extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::*;

use std::io;

#[derive(Parser)]
#[grammar = "ident.pest"]
struct IdentParser;

fn main() {
    let mut input = String::new();

    loop{
        match io::stdin().read_line(&mut input){
            Ok(_) => {
                let pairs = IdentParser::parse(Rule::expression, &input).unwrap_or_else(|e| panic!("{}", e));
                println!("{}", pairs);
                for pair in pairs {
                    println!("result {}", calc(&pair));
                }

            } 

            Err(err) => {
                println!("error: {}", err);
                break;
            }
        }
    }
}

fn  calc <'a>(p:&Pair<'a, Rule>) -> i64{
    
    match p.as_rule() {
        Rule::number => {
            p.as_str().parse::<i64>().unwrap()
        }
        Rule::addsub_exp => {
            let mut current_value = 0;
            //1 add 2 sub
            let mut state = 0;

            for i in p.clone().into_inner(){

                match i.as_rule() {
                    Rule::plus => state = 1,
                    Rule::sub => state=2,
                    _ => {
                        let right = calc(&i);
                        match state {
                            0 => current_value = right,
                            1 => current_value += right,
                            2 => current_value -= right,
                            _ => panic!("Undefined")
                        }
                    }
                }
            }
            // }
            current_value
        }
        Rule::muldiv_exp => {
            let mut current_value = 0;
            //1 add 2 sub
            let mut state = 0;

            for i in p.clone().into_inner(){

                match i.as_rule() {
                    Rule::mul => state = 1,
                    Rule::div => state = 2,
                    _ => {
                        let right = calc(&i);
                        match state {
                            0 => current_value = right,
                            1 => current_value *= right,
                            2 => current_value /= right,
                            _ => panic!("Undefined")
                        }
                    }
                }
            }
            current_value
        }
        Rule::expression=>{
            for i in p.clone().into_inner(){
                return calc(&i)
            }
            0
        }
        _ => panic!("Undefined behavior {}",p.as_str())

    }
}