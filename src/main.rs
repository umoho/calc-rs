use crate::{lexeme::get_tokens, interpret::{eval_f64, eval_decimal}};

mod interpret;
mod parsing;
mod lexeme;
mod decimal;

fn main() {
    println!("Calculator, version 0.1\n");
    let args: Vec<String> = std::env::args().collect();
    let mut backend = Backend::Decimal;
    for arg in args.clone() {
        match arg.as_str() {
            "--backend=f64" | "--backend:f64" => {
                backend = Backend::Float64;
            },
            _ => {}
        }
    }
    if args.len() > 2 {
        let tokens = get_tokens(&args[2]);
        let mut parser = parsing::Parser::new(tokens.clone());
        println!("Input: {}", args[2]);
        match args[1].as_str() {
            "--calc" | "-c" => {
                match backend {
                    Backend::Decimal => {
                        println!("Result: {}", eval_decimal(parser.parse()))
                    },
                    Backend::Float64 => {
                        println!("Result: {}", eval_f64(parser.parse()));
                    },
                }
            },
            "--lexeme" | "-l" => println!("Result: {:#?}", tokens),
            "--tree" | "-t" => println!("Result: {:#?}", parser.parse()),
            _ => { /* ignore */ }
        }
    }
    else {
        shell(backend);
    }
}

fn shell(backend: Backend) {
    println!("Please input the expression in a line, or 'quit' or 'exit'");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim();
        match cmd {
            "quit" | "exit" => break,
            _ => { /* ignore */ }
        }

        let tokens = get_tokens(cmd);
        let mut parser = parsing::Parser::new(tokens.clone());
        match backend {
            Backend::Decimal => {
                println!(": {}", eval_decimal(parser.parse()))
            },
            Backend::Float64 => {
                println!(": {}", eval_f64(parser.parse()));
            },
        }
    }
}

enum Backend {
    Decimal, Float64
}