#![warn(clippy::all)]

use std::iter::Peekable;

use super::error::Error;
use super::expression::Expression;
use super::lexer::{Lexer, Token};

fn associativity(token: &Token) -> i8 {
    match token {
        Token::Caret => 1,
        _ => 0,
    }
}

fn precedence(token: &Token) -> i8 {
    (match token {
        Token::Plus => 1,
        Token::Minus => 1,
        Token::Asterisk => 2,
        Token::Percent => 2,
        Token::Slash => 2,
        Token::Caret => 3,
        _ => 0,
    } + associativity(token))
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
        self.parse_expression(0)
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

    fn parse_expression(&mut self, min_prec: i8) -> Result<Expression, Error> {
        let mut lhs = self.parse_atom()?;
        while let Some(result) = self.lexer.peek() {
            let prec = result.as_ref().map(|token| precedence(&token))?;
            if prec < min_prec {
                break
            };
            let token = self.lexer.next().unwrap().unwrap(); // Safe since we peeked
            lhs = match token {
                Token::Asterisk => Expression::Multiply{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(prec)?),
                },
                Token::Caret => Expression::Exponentiate{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(prec)?),
                },
                Token::Exclamation => Expression::Factorial(Box::new(lhs)),
                Token::Minus => Expression::Subtract{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(prec)?),
                },
                Token::Percent => Expression::Modulo{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(prec)?),
                },
                Token::Plus => Expression::Add{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(prec)?),
                },
                Token::Slash => Expression::Divide{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(prec)?),
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