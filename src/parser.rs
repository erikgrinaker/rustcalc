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
        use PrefixOperator::*;
        match self {
            Minus => Expression::Negate(operand.into()),
            Plus => operand,
            SquareRoot => Expression::SquareRoot(operand.into()),
        }
    }
}

impl Operator for PrefixOperator {
    fn from_token(token: &Token) -> Option<Self> {
        use PrefixOperator::*;
        match token {
            Token::Minus => Some(Minus),
            Token::Plus => Some(Plus),
            Token::SquareRoot => Some(SquareRoot),
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
        use InfixOperator::*;
        match self {
            Add => Expression::Add { lhs: lhs.into(), rhs: rhs.into() },
            Divide => Expression::Divide { lhs: lhs.into(), rhs: rhs.into() },
            Exponentiate => Expression::Exponentiate { lhs: lhs.into(), rhs: rhs.into() },
            Modulo => Expression::Modulo { lhs: lhs.into(), rhs: rhs.into() },
            Multiply => Expression::Multiply { lhs: lhs.into(), rhs: rhs.into() },
            Subtract => Expression::Subtract { lhs: lhs.into(), rhs: rhs.into() },
        }
    }
}

impl Operator for InfixOperator {
    fn from_token(token: &Token) -> Option<Self> {
        use InfixOperator::*;
        match token {
            Token::Plus => Some(Add),
            Token::Minus => Some(Subtract),
            Token::Asterisk => Some(Multiply),
            Token::Slash => Some(Divide),
            Token::Percent => Some(Modulo),
            Token::Caret => Some(Exponentiate),
            _ => None,
        }
    }

    fn associativity(&self) -> i8 {
        use InfixOperator::*;
        match self {
            Exponentiate => ASSOCIATES_RIGHT,
            _ => ASSOCIATES_LEFT,
        }
    }

    fn precedence(&self) -> i8 {
        use InfixOperator::*;
        match self {
            Add | Subtract => 1,
            Multiply | Divide | Modulo => 2,
            Exponentiate => 3,
        }
    }
}

enum PostfixOperator {
    Factorial,
}

impl PostfixOperator {
    fn build(&self, operand: Expression) -> Expression {
        use PostfixOperator::*;
        match self {
            Factorial => Expression::Factorial(operand.into()),
        }
    }
}

impl Operator for PostfixOperator {
    fn from_token(token: &Token) -> Option<Self> {
        use PostfixOperator::*;
        match token {
            Token::Exclamation => Some(Factorial),
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
        Parser { lexer: Lexer::new(input).peekable() }
    }

    /// Builds a constant expression node from a constant name
    fn build_constant(&mut self, name: String) -> Result<Expression, Error> {
        use Constant::*;
        match name.to_lowercase().as_str() {
            "e" => Ok(E.into()),
            "inf" => Ok(Infinity.into()),
            "nan" => Ok(NaN.into()),
            "pi" => Ok(Pi.into()),
            "π" => Ok(Pi.into()),
            _ => Err(Error::Parse(format!("Unknown constant {}", name))),
        }
    }

    fn build_function(&mut self, name: String, args: Vec<Expression>) -> Result<Expression, Error> {
        let count_args = |min, max| {
            if args.len() >= min && args.len() <= max {
                Ok(args.len())
            } else if min == max {
                Err(Error::Parse(format!("{}() takes {} args, received {}", name, min, args.len())))
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
                let decimals = if nargs == 1 { Expression::Number(0.0).into() } else { arg(1) };
                Ok(Expression::Round { value: arg(0), decimals: decimals })
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
        self.lexer
            .next()
            .map_or_else(|| Err(Error::Parse("Unexpected end of input".into())), |r| Ok(r?))
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

    /// Grabs the next lexer token, and returns it if it was expected
    /// or otherwise throws an error.
    fn next_must(&mut self, expect: Option<Token>) -> Result<Option<Token>, Error> {
        if let Some(t) = expect {
            let token = self.next()?;
            if token == t {
                Ok(Some(token))
            } else {
                Err(Error::Parse(format!("Expected token {}, found {}", t, token)))
            }
        } else if let Some(token) = self.peek()? {
            Err(Error::Parse(format!("Unexpected token {}", token)))
        } else {
            Ok(None)
        }
    }

    /// Peeks the next lexer token if any, but converts it from
    /// Option<Result<Token, Error>> to Result<Option<Token>, Error> which is
    /// more convenient to work with (the Iterator trait requires Option<T>).
    fn peek(&mut self) -> Result<Option<Token>, Error> {
        self.lexer.peek().cloned().map_or_else(|| Ok(None), |r| Ok(Some(r?)))
    }

    /// Parse parses the input string into an expression
    pub fn parse(&mut self) -> Result<Expression, Error> {
        let expr = self.parse_expression(1)?;
        self.next_must(None)?;
        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<Expression, Error> {
        match self.next()? {
            Token::Ident(n) => {
                if self.next_if(|t| *t == Token::OpenParen).is_some() {
                    let mut args = Vec::new();
                    while self.next_if(|t| *t == Token::CloseParen).is_none() {
                        if !args.is_empty() {
                            self.next_must(Some(Token::Comma))?;
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
                self.next_must(Some(Token::CloseParen))?;
                Ok(expr)
            }
            token => Err(Error::Parse(format!("Expected value, found {}", token))),
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
            lhs =
                infix.build(lhs, self.parse_expression(infix.precedence() + infix.associativity())?)
        }
        Ok(lhs)
    }
}
