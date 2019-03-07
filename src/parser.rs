#![warn(clippy::all)]

use super::error::Error;
use super::lexer::{Lexer, Token};

pub enum Expression {
    Number(f64),
}

impl Expression {
    pub fn evaluate(self) -> f64 {
        match self {
            Expression::Number(n) => n,
        }
    }
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser{}
    }

    pub fn parse(self, expr: &str) -> Result<Expression, Error> {
        for result in Lexer::new(expr) {
            return match result? {
                Token::Number(n) => self.parse_number(n),
                _ => Err(Error::ParseError("not implemented".to_string())),
            };
        }
        Err(Error::ParseError("No input".to_string()))
    }

    fn parse_number(self, n: String) -> Result<Expression, Error> {
        Ok(Expression::Number(n.parse::<f64>()?))
    }
}