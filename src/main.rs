use crate::interpreter::{
    lexeme::get_tokens,
    interpret::eval,
    parsing::Parser
};

mod number;
mod interpreter;

const VERSION: &str = "1.2";

fn main() {
    println!("Calculator, version {}\n", VERSION);
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        let tokens = get_tokens(&args[2]);
        let mut parser = Parser::new(tokens.clone());
        println!("Input: {}", args[2]);
        match args[1].as_str() {
            "--calc" | "-c" => {
                    println!("Result: {}", eval(parser.parse()))
            },
            "--lexeme" | "-l" => println!("Result: {:#?}", tokens),
            "--tree" | "-t" => println!("Result: {:#?}", parser.parse()),
            _ => { /* ignore */ }
        }
    }
    else {
        shell();
    }
}

fn shell() {
    println!("Please input the expression in a line, or 'quit'");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim();
        match cmd {
            "quit" | "exit" => break,
            "" => { /* when empty */ continue; }
            _ => { /* ignore */ }
        }

        let tokens = get_tokens(cmd);
        let mut parser = Parser::new(tokens.clone());
        println!(": {}", eval(parser.parse()))
    }
}