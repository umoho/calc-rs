use crate::{
    interpreter::{
        parsing::Expression, lexeme::Token
    },
    number::decimal::Decimal
};

pub fn eval(expr: Expression) -> Decimal {
    match expr {
        Expression::Number(n) => {
            Decimal::from(n)
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
                Token::Power => eval(*left).pow(&eval(*right)),
                _ => panic!("Wrong on calculating")
            }
        },
    }
}