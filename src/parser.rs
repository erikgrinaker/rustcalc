#![warn(clippy::all)]

use std::iter::Peekable;

use super::error::Error;
use super::lexer::{Lexer, Token};

pub enum Expression {
    Number(f64),
    Negate(Box<Expression>),
}

impl Expression {
    pub fn evaluate(self) -> f64 {
        match self {
            Expression::Negate(rhs) => - rhs.evaluate(),
            Expression::Number(n) => n,
        }
    }
}

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

// This uses the precedence climbing parser algorithm
impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        Parser{
            lexer: Lexer::new(input).peekable()
        }
    }

    pub fn parse(&mut self) -> Result<Expression, Error> {
        self.parse_expression()
    }

    fn parse_atom(&mut self) -> Result<Expression, Error> {
        let token = self.lexer.next().ok_or_else(||
            Error::ParseError("Unexpected end of input".to_string()))??;
        match token {
            Token::Number(n) => self.parse_number(n.to_string()),
            Token::Minus => Ok(Expression::Negate(Box::new(self.parse_atom()?))),
            Token::Plus => Ok(self.parse_atom()?),
            _ => Err(Error::ParseError(format!("Unexpected token {}", token))),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_atom()
    }

    fn parse_number(&mut self, n: String) -> Result<Expression, Error> {
        Ok(Expression::Number(n.parse::<f64>()?))
    }
}