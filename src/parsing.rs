use crate::lexeme::Token;

#[derive(Debug)]
pub enum Expression {
    Number(String),
    Unary { op: Token, expr: Box<Expression> },
    Binary { op: Token, left: Box<Expression>, right: Box<Expression> },
}

pub struct Parser {
    position: usize,
    token_stream: Vec<Token>,
    current_token: Token,
}

impl Parser {
    pub fn new(token_stream: Vec<Token>) -> Self {
        Self { position: 0, token_stream: token_stream.clone(), current_token: token_stream[0].clone() }
    }

    pub fn parse(&mut self) -> Expression { self.expr() }

    fn next_token(&mut self) {
        self.position += 1;
        if self.position < self.token_stream.len() {
            self.current_token = self.token_stream[self.position].clone();
        }
    }

    fn eat(&mut self, token: Token) {
        if self.current_token == token {
            self.next_token();
        }
        else {
            panic!("Wrong at eating")
        }
    }

    fn factor(&mut self) -> Expression {
        let token = self.current_token.clone();
        match token {
            Token::OpenParenthesis => {
                self.eat(Token::OpenParenthesis);
                let elem = self.expr();
                self.eat(Token::CloseParenthesis);
                elem
            },
            Token::Number(n) => {
                self.eat(Token::Number(String::new()));
                Expression::Number(n)
            },
            Token::Plus | Token::Minus => {
                self.eat(token.clone());
                Expression::Unary { op: token, expr: Box::new(self.factor()) }
            },
            _ => panic!("Wrong in factor")
        }
    }

    fn term(&mut self) -> Expression {
        let mut node = self.factor();
        while match self.current_token {  // condition
            Token::Multiply | Token::Divide => true,
            _ => false
        }
        {  // repeat
            let token = self.current_token.clone();
            self.eat(token.clone());
            node = Expression::Binary {
                op: token,
                left: Box::new(node),
                right: Box::new(self.factor())
            };
        }
        node
    }

    fn expr(&mut self) -> Expression {
        let mut node = self.term();
        while match self.current_token {  // condition
            Token::Plus | Token::Minus => true,
            _ => false
        }
        {  // repeat
            let token = self.current_token.clone();
            self.eat(token.clone());
            node = Expression::Binary {
                op: token,
                left: Box::new(node),
                right: Box::new(self.term())
            };
        }
        node
    }
}

#[test]
fn test() {
    use crate::lexeme;
    let examples: Vec<&str> = vec![
        "1",
        "1 * 2",
        "         1.14 \t\n +      5.1 * \t0.4",
        "1 / 2 * 3",
        "1 * 2 / 3",
        "-(114 * (5 + 1)) / 4",
        ];
    for example in examples {
        let ts = lexeme::get_tokens(example);
        let mut p = Parser::new(ts);
        println!("Text: {}\nExpr: {:#?}\n", example, p.expr());
    }
}