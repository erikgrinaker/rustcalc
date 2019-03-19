use std::iter::Peekable;

use crate::error::Error;
use crate::expression::{Constant, Expression};
use crate::lexer::{Lexer, Token};

const ASSOC_LEFT: u8 = 1;
const ASSOC_RIGHT: u8 = 0;

/// An operator represents a token that operates on surrounding values
trait Operator: Sized {
    /// Creates an operator from a token, if appropriate
    fn from(token: &Token) -> Option<Self>;

    /// Returns the associativity of the operator
    fn assoc(&self) -> u8;

    /// Returns the precedence of the operator
    fn prec(&self) -> u8;
}

// Prefix operators, e.g. -(1 + 2)
enum PrefixOperator {
    Minus,
    Plus,
    SquareRoot,
}

impl PrefixOperator {
    // Builds an expression node for the prefix operator
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
    fn from(token: &Token) -> Option<Self> {
        use PrefixOperator::*;
        match token {
            Token::Minus => Some(Minus),
            Token::Plus => Some(Plus),
            Token::SquareRoot => Some(SquareRoot),
            _ => None,
        }
    }

    fn assoc(&self) -> u8 {
        ASSOC_RIGHT
    }

    fn prec(&self) -> u8 {
        5
    }
}

/// Infix operators, e.g. 1 + 2
enum InfixOperator {
    Add,
    Divide,
    Exponentiate,
    Modulo,
    Multiply,
    Subtract,
}

impl InfixOperator {
    // Builds an expression node for the infix operator
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
    fn from(token: &Token) -> Option<Self> {
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

    fn assoc(&self) -> u8 {
        use InfixOperator::*;
        match self {
            Exponentiate => ASSOC_RIGHT,
            _ => ASSOC_LEFT,
        }
    }

    fn prec(&self) -> u8 {
        use InfixOperator::*;
        match self {
            Add | Subtract => 1,
            Multiply | Divide | Modulo => 2,
            Exponentiate => 3,
        }
    }
}

/// Postfix operators, e.g. 5!
enum PostfixOperator {
    Factorial,
}

impl PostfixOperator {
    // Builds an expression node for the postfix operator
    fn build(&self, operand: Expression) -> Expression {
        use PostfixOperator::*;
        match self {
            Factorial => Expression::Factorial(operand.into()),
        }
    }
}

impl Operator for PostfixOperator {
    fn from(token: &Token) -> Option<Self> {
        use PostfixOperator::*;
        match token {
            Token::Exclamation => Some(Factorial),
            _ => None,
        }
    }

    fn assoc(&self) -> u8 {
        ASSOC_LEFT
    }

    fn prec(&self) -> u8 {
        4
    }
}

/// Parses an input string into an expression
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser for the given input string
    pub fn new(input: &str) -> Parser {
        Parser { lexer: Lexer::new(input).peekable() }
    }

    /// Builds an expression node from a constant name
    fn build_constant(&self, name: String) -> Result<Expression, Error> {
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

    // Builds an expression node from a function call
    fn build_function(&self, name: String, mut args: Vec<Expression>) -> Result<Expression, Error> {
        args.reverse();
        let mut arg = || {
            args.pop()
                .map(|e| e.clone().into())
                .ok_or_else(|| Error::Parse(format!("Missing argument for {}()", name)))
        };
        let expr = match name.to_lowercase().as_str() {
            "cos" => Expression::Cosine(arg()?),
            "degrees" => Expression::Degrees(arg()?),
            "radians" => Expression::Radians(arg()?),
            "round" => {
                Expression::Round { value: arg()?, decimals: arg().unwrap_or_else(|_| 0.0.into()) }
            }
            "sin" => Expression::Sine(arg()?),
            "sqrt" => Expression::SquareRoot(arg()?),
            "tan" => Expression::Tangent(arg()?),
            _ => return Err(Error::Parse(format!("Unknown function {}", name))),
        };
        if args.is_empty() {
            Ok(expr)
        } else {
            Err(Error::Parse(format!("Unexpected argument for {}()", name)))
        }
    }

    /// Builds a number node from a number literal
    fn build_number(&self, literal: String) -> Result<Expression, Error> {
        Ok(literal.parse::<f64>()?.into())
    }

    /// Grabs the next lexer token, or throws an error if none is found.
    fn next(&mut self) -> Result<Token, Error> {
        self.lexer
            .next()
            .map_or_else(|| Err(Error::Parse("Unexpected end of input".into())), |r| Ok(r?))
    }

    /// Grabs the next lexer token, and returns it if it was expected or
    /// otherwise throws an error.
    fn next_expect(&mut self, expect: Option<Token>) -> Result<Option<Token>, Error> {
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

    /// Grabs the next lexer token if it satisfies the predicate function
    fn next_if<F: Fn(&Token) -> bool>(&mut self, predicate: F) -> Option<Token> {
        self.peek().unwrap_or(None).filter(|t| predicate(&t))?;
        self.next().ok()
    }

    /// Grabs the next operator token if it satisfies the type and precedence
    fn next_if_operator<T: Operator>(&mut self, min_prec: u8) -> Option<T> {
        let operator = self
            .peek()
            .unwrap_or(None)
            .and_then(|t| T::from(&t))
            .filter(|o| o.prec() >= min_prec)?;
        self.next().ok();
        Some(operator)
    }

    /// Peeks the next lexer token if any, but converts it from
    /// Option<Result<Token, Error>> to Result<Option<Token>, Error> which is
    /// more convenient to work with (the Iterator trait requires Option<T>).
    fn peek(&mut self) -> Result<Option<Token>, Error> {
        self.lexer.peek().cloned().map_or_else(|| Ok(None), |r| Ok(Some(r?)))
    }

    /// Parses the input string into an expression
    pub fn parse(&mut self) -> Result<Expression, Error> {
        let expr = self.parse_expression(0)?;
        self.next_expect(None)?;
        Ok(expr)
    }

    /// Parses an atom, i.e. a number, constant, function, or parenthesis
    fn parse_atom(&mut self) -> Result<Expression, Error> {
        match self.next()? {
            Token::Ident(n) => {
                if self.next_if(|t| *t == Token::OpenParen).is_some() {
                    let mut args = Vec::new();
                    while self.next_if(|t| *t == Token::CloseParen).is_none() {
                        if !args.is_empty() {
                            self.next_expect(Some(Token::Comma))?;
                        }
                        args.push(self.parse_expression(0)?);
                    }
                    self.build_function(n.clone(), args)
                } else {
                    self.build_constant(n.clone())
                }
            }
            Token::Number(n) => self.build_number(n.clone()),
            Token::OpenParen => {
                let expr = self.parse_expression(0)?;
                self.next_expect(Some(Token::CloseParen))?;
                Ok(expr)
            }
            t => Err(Error::Parse(format!("Expected value, found {}", t))),
        }
    }

    /// Parses an expression consisting of at least one atom operated on by any
    /// number of operators. Uses precedence climbing.
    fn parse_expression(&mut self, min_prec: u8) -> Result<Expression, Error> {
        let mut lhs = if let Some(prefix) = self.next_if_operator::<PrefixOperator>(min_prec) {
            prefix.build(self.parse_expression(prefix.prec() + prefix.assoc())?)
        } else {
            self.parse_atom()?
        };
        while let Some(postfix) = self.next_if_operator::<PostfixOperator>(min_prec) {
            lhs = postfix.build(lhs)
        }
        while let Some(infix) = self.next_if_operator::<InfixOperator>(min_prec) {
            lhs = infix.build(lhs, self.parse_expression(infix.prec() + infix.assoc())?)
        }
        Ok(lhs)
    }
}
