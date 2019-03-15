#![warn(clippy::all)]

use std::iter::Peekable;

use super::error::Error;
use super::expression::Expression;
use super::lexer::{Lexer, Token};

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
        let mut lhs = self.parse_atom()?;
        while let Some(result) = self.lexer.next() {
            let token = result?;
            lhs = match token {
                Token::Asterisk => Expression::Multiply{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression()?),
                },
                Token::Caret => Expression::Exponentiate{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression()?),
                },
                Token::Exclamation => Expression::Factorial(Box::new(lhs)),
                Token::Minus => Expression::Subtract{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression()?),
                },
                Token::Percent => Expression::Modulo{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression()?),
                },
                Token::Plus => Expression::Add{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression()?),
                },
                Token::Slash => Expression::Divide{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression()?),
                },
                _ => return Err(Error::ParseError(format!("Unexpected token {}", token))),
            };
        }
        Ok(lhs)
    }

    fn parse_number(&mut self, n: String) -> Result<Expression, Error> {
        Ok(Expression::Number(n.parse::<f64>()?))
    }
}