use crate::{parsing::Expression, lexeme::Token, decimal::Decimal};

pub fn eval_f64(expr: Expression) -> f64 {
    match expr {
        Expression::Number(n) => {
            n
        },
        Expression::Unary { op, expr } => {
            match op {
                Token::Plus => eval_f64(*expr),
                Token::Minus => -eval_f64(*expr),
                _ => panic!("Wrong of signed number")
            }
        },
        Expression::Binary { op, left, right } => {
            match op {
                Token::Plus => eval_f64(*left) + eval_f64(*right),
                Token::Minus => eval_f64(*left) - eval_f64(*right),
                Token::Multiply => eval_f64(*left) * eval_f64(*right),
                Token::Divide => eval_f64(*left) / eval_f64(*right),
                _ => panic!("Wrong on calculating")
            }
        },
    }
}

#[test]
fn test_eval_f64() {
    use crate::lexeme;
    use crate::parsing::Parser;
    let examples: Vec<&str> = vec![
        "1",
        ".1 * .3 * 10.",
        "(.1 + .1 + .1) * 1000",  // decimal error
        ".1) + (.1",  // incorrect result
        ];
    for example in examples {
        let ts = lexeme::get_tokens(example);
        let mut p = Parser::new(ts);
        let result = eval_f64(p.parse());
        println!("Result: {}", result);
    }
}

pub fn eval_decimal(expr: Expression) -> Decimal {
    match expr {
        Expression::Number(n) => {
            Decimal::from(n)
        },
        Expression::Unary { op, expr } => {
            match op {
                Token::Plus => eval_decimal(*expr),
                Token::Minus => -eval_decimal(*expr),
                _ => panic!("Wrong of signed number")
            }
        },
        Expression::Binary { op, left, right } => {
            match op {
                Token::Plus => eval_decimal(*left) + eval_decimal(*right),
                Token::Minus => eval_decimal(*left) - eval_decimal(*right),
                Token::Multiply => eval_decimal(*left) * eval_decimal(*right),
                Token::Divide => eval_decimal(*left) / eval_decimal(*right),
                _ => panic!("Wrong on calculating")
            }
        },
    }
}