use std::iter::Peekable;

use crate::error::Error;
use crate::expression::{Constant, Expression};
use crate::lexer::{Lexer, Token};

const ASSOCIATES_LEFT: i8 = 1;
const ASSOCIATES_RIGHT: i8 = 0;

trait Operator: Sized {
    fn from_token(token: &Token) -> Option<Self>;
    fn associativity(&self) -> i8;
    fn precedence(&self) -> i8;
}

enum PrefixOperator {
    Minus,
    Plus,
    SquareRoot,
}

impl PrefixOperator {
    fn build(&self, operand: Expression) -> Expression {
        match self {
            PrefixOperator::Minus => Expression::Negate(operand.into()),
            PrefixOperator::Plus => operand,
            PrefixOperator::SquareRoot => Expression::SquareRoot(operand.into()),
        }
    }
}

impl Operator for PrefixOperator {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Minus => Some(PrefixOperator::Minus),
            Token::Plus => Some(PrefixOperator::Plus),
            Token::SquareRoot => Some(PrefixOperator::SquareRoot),
            _ => None,
        }
    }

    fn associativity(&self) -> i8 {
        ASSOCIATES_RIGHT
    }

    fn precedence(&self) -> i8 {
        5
    }
}

enum InfixOperator {
    Add,
    Divide,
    Exponentiate,
    Modulo,
    Multiply,
    Subtract,
}

impl InfixOperator {
    fn build(&self, lhs: Expression, rhs: Expression) -> Expression {
        match self {
            InfixOperator::Add => Expression::Add {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            InfixOperator::Divide => Expression::Divide {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            InfixOperator::Exponentiate => Expression::Exponentiate {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            InfixOperator::Modulo => Expression::Modulo {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            InfixOperator::Multiply => Expression::Multiply {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
            InfixOperator::Subtract => Expression::Subtract {
                lhs: lhs.into(),
                rhs: rhs.into(),
            },
        }
    }
}

impl Operator for InfixOperator {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Plus => Some(InfixOperator::Add),
            Token::Minus => Some(InfixOperator::Subtract),
            Token::Asterisk => Some(InfixOperator::Multiply),
            Token::Slash => Some(InfixOperator::Divide),
            Token::Percent => Some(InfixOperator::Modulo),
            Token::Caret => Some(InfixOperator::Exponentiate),
            _ => None,
        }
    }

    fn associativity(&self) -> i8 {
        match self {
            InfixOperator::Exponentiate => ASSOCIATES_RIGHT,
            _ => ASSOCIATES_LEFT,
        }
    }

    fn precedence(&self) -> i8 {
        match self {
            InfixOperator::Add | InfixOperator::Subtract => 1,
            InfixOperator::Multiply | InfixOperator::Divide | InfixOperator::Modulo => 2,
            InfixOperator::Exponentiate => 3,
        }
    }
}

enum PostfixOperator {
    Factorial,
}

impl PostfixOperator {
    fn build(&self, operand: Expression) -> Expression {
        match self {
            PostfixOperator::Factorial => Expression::Factorial(operand.into()),
        }
    }
}

impl Operator for PostfixOperator {
    fn from_token(token: &Token) -> Option<Self> {
        match token {
            Token::Exclamation => Some(PostfixOperator::Factorial),
            _ => None,
        }
    }

    fn associativity(&self) -> i8 {
        ASSOCIATES_LEFT
    }

    fn precedence(&self) -> i8 {
        4
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
        let arg = |n: usize| args[n].clone().into();
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
                    Expression::Number(0.0).into()
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

    /// Grabs the next lexer token, or throws an error if none is found.
    fn next(&mut self) -> Result<Token, Error> {
        self.lexer.next().map_or_else(
            || Err(Error::Parse("Unexpected end of input".into())),
            |r| Ok(r?),
        )
    }

    /// Grabs the next lexer token if it satisfies the predicate function
    fn next_if<F>(&mut self, predicate: F) -> Option<Token>
    where
        F: Fn(&Token) -> bool,
    {
        self.peek().unwrap_or(None).filter(|t| predicate(&t))?;
        self.next().ok()
    }

    /// Grabs the next operator if the operizer function returns one
    fn next_if_operator<F, T>(&mut self, operizer: F) -> Option<T>
    where
        F: Fn(&Token) -> Option<T>,
        T: Operator,
    {
        let operator = self.peek().unwrap_or(None).and_then(|t| operizer(&t))?;
        self.next().ok();
        Some(operator)
    }

    /// Peeks the next lexer token if any, but converts it from
    /// Option<Result<Token, Error>> to Result<Option<Token>, Error> which is
    /// more convenient to work with (the Iterator trait requires Option<T>).
    fn peek(&mut self) -> Result<Option<Token>, Error> {
        self.lexer
            .peek()
            .cloned()
            .map_or_else(|| Ok(None), |r| Ok(Some(r?)))
    }

    /// Parse parses the input string into an expression
    pub fn parse(&mut self) -> Result<Expression, Error> {
        let expr = self.parse_expression(1)?;
        if let Some(token) = self.peek()? {
            Err(Error::Parse(format!("Unexpected token {}", token)))
        } else {
            Ok(expr)
        }
    }

    fn parse_atom(&mut self) -> Result<Expression, Error> {
        let token = self.next()?;
        match token {
            Token::Ident(n) => {
                if self.next_if(|t| *t == Token::OpenParen).is_some() {
                    let mut args = Vec::new();
                    while self.next_if(|t| *t == Token::CloseParen).is_none() {
                        if !args.is_empty() {
                            let t = self.next()?;
                            if t != Token::Comma {
                                return Err(Error::Parse(format!("Unexpected token {}", t)));
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
        let mut lhs = if let Some(prefix) = self.next_if_operator(PrefixOperator::from_token) {
            prefix.build(self.parse_expression(prefix.precedence() + prefix.associativity())?)
        } else {
            self.parse_atom()?
        };
        while let Some(postfix) = self.next_if_operator(|t| {
            PostfixOperator::from_token(t).filter(|o| o.precedence() >= min_prec)
        }) {
            lhs = postfix.build(lhs)
        }
        while let Some(infix) = self.next_if_operator(|t| {
            InfixOperator::from_token(t).filter(|o| o.precedence() >= min_prec)
        }) {
            lhs = infix.build(
                lhs,
                self.parse_expression(infix.precedence() + infix.associativity())?,
            )
        }
        Ok(lhs)
    }
}
