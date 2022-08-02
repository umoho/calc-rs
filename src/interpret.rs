use crate::{parsing::Expression, lexeme::Token};

pub fn eval(expr: Expression) -> f64 {
    match expr {
        Expression::Number(n) => {
            n
        },
        Expression::Unary { op, expr } => {
            match op {
                Token::Plus => eval(*expr),
                Token::Minus => -eval(*expr),
                _ => panic!("Wrong of signed number")
            }
        },
        Expression::Binary { op, left, right } => {
            match op {
                Token::Plus => eval(*left) + eval(*right),
                Token::Minus => eval(*left) - eval(*right),
                Token::Multiply => eval(*left) * eval(*right),
                Token::Divide => eval(*left) / eval(*right),
                _ => panic!("Wrong of calculating")
            }
        },
    }
}

#[test]
fn test_eval() {
    use crate::lexeme;
    use crate::parsing::Parser;
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
        let result = eval(p.parse());
        println!("Result: {}", result);
    }
}