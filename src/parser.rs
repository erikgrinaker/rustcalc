use std::iter::Peekable;

use crate::error::Error;
use crate::expression::{Constant, Expression};
use crate::lexer::{Lexer, Token};

const ASSOCIATES_LEFT: i8 = 1;
const ASSOCIATES_RIGHT: i8 = 0;

impl Token {
    // Returns the token's associativity
    fn associativity(&self) -> i8 {
        match self {
            Token::Caret => ASSOCIATES_RIGHT,
            _ => ASSOCIATES_LEFT,
        }
    }

    // Returns the token's precedence
    fn precedence(&self) -> i8 {
        match self {
            Token::Ident(..) => 0,
            Token::Number(..) => 0,
            Token::OpenParen => 0,
            Token::CloseParen => 0,
            Token::Comma => 0,
            Token::Plus => 1,
            Token::Minus => 1,
            Token::SquareRoot => 1,
            Token::Asterisk => 2,
            Token::Percent => 2,
            Token::Slash => 2,
            Token::Caret => 3,
            Token::Exclamation => 4,
        }
    }
}

/// Parses an input string into an expression, by precedence climbing
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser for the given input string
    pub fn new(input: &str) -> Parser {
        Parser {
            lexer: Lexer::new(input).peekable(),
        }
    }

    /// Builds a constant expression node from a constant name
    fn build_constant(&mut self, name: String) -> Result<Expression, Error> {
        match name.to_lowercase().as_str() {
            "e" => Ok(Constant::E.into()),
            "inf" => Ok(Constant::Infinity.into()),
            "nan" => Ok(Constant::NaN.into()),
            "pi" => Ok(Constant::Pi.into()),
            "π" => Ok(Constant::Pi.into()),
            _ => Err(Error::Parse(format!("Unknown constant {}", name))),
        }
    }

    fn build_function(&mut self, name: String, args: Vec<Expression>) -> Result<Expression, Error> {
        let count_args = |min, max| {
            if args.len() >= min && args.len() <= max {
                Ok(args.len())
            } else if min == max {
                Err(Error::Parse(format!(
                    "{}() takes {} args, received {}",
                    name,
                    min,
                    args.len()
                )))
            } else {
                Err(Error::Parse(format!(
                    "{}() takes {}-{} args, received {}",
                    name,
                    min,
                    max,
                    args.len()
                )))
            }
        };
        let arg = |n: usize| Box::new(args[n].clone());
        match name.to_lowercase().as_str() {
            "cos" => {
                count_args(1, 1)?;
                Ok(Expression::Cosine(arg(0)))
            }
            "degrees" => {
                count_args(1, 1)?;
                Ok(Expression::Degrees(arg(0)))
            }
            "radians" => {
                count_args(1, 1)?;
                Ok(Expression::Radians(arg(0)))
            }
            "round" => {
                let nargs = count_args(1, 2)?;
                let decimals = if nargs == 1 {
                    Box::new(Expression::Number(0.0))
                } else {
                    arg(1)
                };
                Ok(Expression::Round {
                    value: arg(0),
                    decimals: decimals,
                })
            }
            "sin" => {
                count_args(1, 1)?;
                Ok(Expression::Sine(arg(0)))
            }
            "sqrt" => {
                count_args(1, 1)?;
                Ok(Expression::SquareRoot(arg(0)))
            }
            "tan" => {
                count_args(1, 1)?;
                Ok(Expression::Tangent(arg(0)))
            }
            _ => Err(Error::Parse(format!("Unknown function {}", name))),
        }
    }

    /// Builds a number node from a number literal
    fn build_number(&mut self, literal: String) -> Result<Expression, Error> {
        Ok(literal.parse::<f64>()?.into())
    }

    /// Grabs the next lexer token if it satisfies the predicate function
    fn next_if<F>(&mut self, predicate: F) -> Option<Token>
    where
        F: Fn(&Token) -> bool,
    {
        self.lexer.peek().cloned()?.ok().filter(|t| predicate(t))?;
        self.lexer.next()?.ok()
    }

    /// Parse parses the input string into an expression
    pub fn parse(&mut self) -> Result<Expression, Error> {
        let expr = self.parse_expression(1)?;
        if let Some(result) = self.lexer.next() {
            Err(Error::Parse(format!("Unexpected token {}", result?)))
        } else {
            Ok(expr)
        }
    }

    fn parse_atom(&mut self) -> Result<Expression, Error> {
        let token = self
            .lexer
            .next()
            .ok_or_else(|| Error::Parse("Unexpected end of input".into()))??;
        match token {
            Token::Ident(n) => {
                if self.next_if(|t| *t == Token::OpenParen).is_some() {
                    let mut args = Vec::new();
                    while self.next_if(|t| *t == Token::CloseParen).is_none() {
                        if !args.is_empty() {
                            match self.lexer.next() {
                                Some(Ok(Token::Comma)) if !args.is_empty() => (),
                                Some(Ok(t)) => {
                                    return Err(Error::Parse(format!("Unexpected token {}", t)));
                                }
                                Some(Err(err)) => return Err(err),
                                None => return Err(Error::Parse("Unexpected end of input".into())),
                            }
                        }
                        args.push(self.parse_expression(1)?);
                    }
                    self.build_function(n.clone(), args)
                } else {
                    self.build_constant(n.clone())
                }
            }
            Token::Number(n) => self.build_number(n.clone()),
            Token::Minus => Ok(Expression::Negate(Box::new(self.parse_atom()?))),
            Token::Plus => Ok(self.parse_atom()?),
            Token::SquareRoot => Ok(Expression::SquareRoot(Box::new(self.parse_atom()?))),
            Token::OpenParen => {
                let expr = self.parse_expression(1)?; // 1 implies stop at )
                if self.next_if(|t| *t == Token::CloseParen).is_none() {
                    Err(Error::Parse("Closing ) not found".into()))
                } else {
                    Ok(expr)
                }
            }
            _ => Err(Error::Parse(format!("Unexpected token {}", token))),
        }
    }

    fn parse_expression(&mut self, min_prec: i8) -> Result<Expression, Error> {
        let mut lhs = self.parse_atom()?;
        while let Some(token) = self.next_if(|t| t.precedence() >= min_prec) {
            lhs = match token {
                Token::Asterisk => Expression::Multiply {
                    lhs: Box::new(lhs),
                    rhs: Box::new(
                        self.parse_expression(token.precedence() + token.associativity())?,
                    ),
                },
                Token::Caret => Expression::Exponentiate {
                    lhs: Box::new(lhs),
                    rhs: Box::new(
                        self.parse_expression(token.precedence() + token.associativity())?,
                    ),
                },
                Token::Exclamation => Expression::Factorial(Box::new(lhs)),
                Token::Minus => Expression::Subtract {
                    lhs: Box::new(lhs),
                    rhs: Box::new(
                        self.parse_expression(token.precedence() + token.associativity())?,
                    ),
                },
                Token::Percent => Expression::Modulo {
                    lhs: Box::new(lhs),
                    rhs: Box::new(
                        self.parse_expression(token.precedence() + token.associativity())?,
                    ),
                },
                Token::Plus => Expression::Add {
                    lhs: Box::new(lhs),
                    rhs: Box::new(
                        self.parse_expression(token.precedence() + token.associativity())?,
                    ),
                },
                Token::Slash => Expression::Divide {
                    lhs: Box::new(lhs),
                    rhs: Box::new(
                        self.parse_expression(token.precedence() + token.associativity())?,
                    ),
                },
                _ => break,
            };
        }
        Ok(lhs)
    }
}
