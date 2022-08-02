use crate::{lexeme::get_tokens, interpret::eval};

mod interpret;
mod parsing;
mod lexeme;

fn main() {
    println!("Calculator, version 0.1\n");
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        let tokens = get_tokens(&args[2]);
        let mut parser = parsing::Parser::new(tokens.clone());
        println!("Input: {}", args[2]);
        match args[1].as_str() {
            "--calc" | "-c" => println!("Result: {}", eval(parser.parse())),
            "--lexeme" | "-l" => println!("Result: {:#?}", tokens),
            "--tree" | "-t" => println!("Result: {:#?}", parser.parse()),
            _ => { /* ignore */ }
        }
    }
}