const END: char = '\0';

#[derive(Debug)]
#[derive(Clone, Copy)]
pub enum Token {
    Number(f64),
    Plus, Minus, Multiply, Divide,
    LeftParenthesis, RightParenthesis,
    Fin
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(_), Self::Number(_)) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[test]
fn test_token_partial_eq() {
    assert_eq!(Token::Number(0.0), Token::Number(0.0));
    assert!(Token::Number(114.514) == Token::Number(233.3));
    assert_ne!(Token::Number(0.0), Token::Fin);
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Tokenizer {
    text: String,
    position: usize,
    ch: char
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        let first_char = *input.as_bytes().first().unwrap() as char;
        Self {
            text: input.to_string(), position: 0, ch: first_char
        }
    }

    fn advance(&mut self) {
        self.position += 1;
        let current_chars = self.text.as_bytes();
        if self.is_end() {
            self.ch = END  // I think it's not a good design
        }
        else {
            self.ch = current_chars[self.position] as char;
        }
    }

    fn is_end(&self) -> bool {
        if self.position >= self.text.len() { true } else { false }
    }

    pub(crate) fn next_token(&mut self) -> Token {
        while self.ch != END {
            match self.ch {
                ' ' | '\t' | '\n' => {
                    self.advance();
                    continue;
                },
                '(' => {
                    self.advance();
                    return Token::LeftParenthesis;
                },
                ')' => {
                    self.advance();
                    return Token::RightParenthesis;
                },
                c if is_digit(&c) => {
                    return Token::Number(self.read_number());
                },
                '*' => {
                    self.advance();
                    return Token::Multiply;
                },
                '/' => {
                    self.advance();
                    return Token::Divide;
                },
                '+' => {
                    self.advance();
                    return Token::Plus;
                },
                '-' => {
                    self.advance();
                    return Token::Minus;
                },
                _ => panic!("Wrong in tokenizing")
            }
        }
        Token::Fin
    }

    fn read_number(&mut self) -> f64 {
        let mut result = String::new();
        while self.ch != END && is_digit(&self.ch) {
            result += &self.ch.to_string();
            self.advance();
        }
        result.parse::<f64>().unwrap()
    }
}

fn is_digit(ch: &char) -> bool {
    match ch {
        '0'..='9' | '.' => true,
        _ => false
    }
}

pub fn get_tokens(input: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(input);
    let mut tokens: Vec<Token> = vec![];
    loop {  // repeat
        let next_token = tokenizer.next_token();
        if next_token == Token::Fin { break; }  // until
        tokens.push(next_token);
    }
    tokens
}

#[test]
fn test() {
    let tokens = get_tokens("114 + .514 - (19.19 * (-8)) / 10");
    // let tokens = get_tokens("-.1");
    println!("{:#?}", tokens);
}