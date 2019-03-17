#![warn(clippy::all)]

use std::iter::Peekable;

use super::error::Error;
use super::expression::{Constant, Expression};
use super::lexer::{Lexer, Token};

const ASSOCIATES_LEFT: i8 = 1;
const ASSOCIATES_RIGHT: i8 = 0;

impl Token {
    fn associativity(&self) -> i8 {
        match self {
            Token::Caret => ASSOCIATES_RIGHT,
            _ => ASSOCIATES_LEFT,
        }
    }

    fn precedence(&self) -> i8 {
        match self {
            Token::Ident(..) => 0,
            Token::Number(..) => 0,
            Token::OpenParen => 0,
            Token::CloseParen => 0,
            Token::Plus => 1,
            Token::Minus => 1,
            Token::Asterisk => 2,
            Token::Percent => 2,
            Token::Slash => 2,
            Token::Caret => 3,
            Token::Exclamation => 4,
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

    fn next_if<F>(&mut self, predicate: F) -> Option<Token> where F: Fn(&Token) -> bool {
        let token = match self.lexer.peek()? {
            Ok(t) => t,
            Err(_) => return None,
        };
        if predicate(token) {
            Some(self.lexer.next().unwrap().unwrap())
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Result<Expression, Error> {
        let expr = self.parse_expression(1)?;
        if let Some(result) = self.lexer.next() {
            Err(Error::Parse(format!("Unexpected token {}", result?)))
        } else {
            Ok(expr)
        }
    }

    fn parse_atom(&mut self) -> Result<Expression, Error> {
        let token = self.lexer.next().ok_or_else(||
            Error::Parse("Unexpected end of input".into()))??;
        match token {
            Token::Ident(n) => self.parse_constant(n.clone()),
            Token::Number(n) => self.parse_number(n.clone()),
            Token::Minus => Ok(Expression::Negate(Box::new(self.parse_atom()?))),
            Token::Plus => Ok(self.parse_atom()?),
            Token::OpenParen => {
                let expr = self.parse_expression(1)?; // 1 implies stop at )
                if self.next_if(|t| *t == Token::CloseParen).is_none() {
                    Err(Error::Parse("Closing ) not found".into()))
                } else {
                    Ok(expr)
                }
            },
            _ => Err(Error::Parse(format!("Unexpected token {}", token))),
        }
    }

    fn parse_expression(&mut self, min_prec: i8) -> Result<Expression, Error> {
        let mut lhs = self.parse_atom()?;
        while let Some(token) = self.next_if(|t| t.precedence() >= min_prec) {
            lhs = match token {
                Token::Asterisk => Expression::Multiply{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(token.precedence() + token.associativity())?),
                },
                Token::Caret => Expression::Exponentiate{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(token.precedence() + token.associativity())?),
                },
                Token::Exclamation => Expression::Factorial(Box::new(lhs)),
                Token::Minus => Expression::Subtract{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(token.precedence() + token.associativity())?),
                },
                Token::Percent => Expression::Modulo{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(token.precedence() + token.associativity())?),
                },
                Token::Plus => Expression::Add{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(token.precedence() + token.associativity())?),
                },
                Token::Slash => Expression::Divide{
                    lhs: Box::new(lhs),
                    rhs: Box::new(self.parse_expression(token.precedence() + token.associativity())?),
                },
                _ => break,
            };
        };
        Ok(lhs)
    }

    fn parse_constant(&mut self, name: String) -> Result<Expression, Error> {
        Ok(Expression::Constant(
            match name.to_lowercase().as_str() {
                "e" => Constant::E,
                "inf" => Constant::Infinity,
                "nan" => Constant::NaN,
                "pi" => Constant::Pi,
                _ => return Err(Error::Parse(format!("Unknown constant {}", name))),
            }
        ))
    }

    fn parse_number(&mut self, n: String) -> Result<Expression, Error> {
        Ok(Expression::Number(n.parse::<f64>()?))
    }
}