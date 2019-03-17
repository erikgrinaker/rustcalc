#![warn(clippy::all)]

use std::f64;

#[derive(Debug)]
pub enum Constant {
    E,
    Infinity,
    NaN,
    Pi,
}

impl Constant {
    fn value(&self) -> f64 {
        match self {
            Constant::E => f64::consts::E,
            Constant::Infinity => f64::INFINITY,
            Constant::NaN => f64::NAN,
            Constant::Pi => f64::consts::PI,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Add{lhs: Box<Expression>, rhs: Box<Expression>},
    Constant(Constant),
    Divide{lhs: Box<Expression>, rhs: Box<Expression>},
    Exponentiate{lhs: Box<Expression>, rhs: Box<Expression>},
    Factorial(Box<Expression>),
    Modulo{lhs: Box<Expression>, rhs: Box<Expression>},
    Multiply{lhs: Box<Expression>, rhs: Box<Expression>},
    Negate(Box<Expression>),
    Number(f64),
    Subtract{lhs: Box<Expression>, rhs: Box<Expression>},
}

impl Expression {
    pub fn evaluate(&self) -> f64 {
        match self {
            Expression::Add{lhs, rhs} => lhs.evaluate() + rhs.evaluate(),
            Expression::Constant(c) => c.value(),
            Expression::Divide{lhs, rhs} => lhs.evaluate() / rhs.evaluate(),
            Expression::Exponentiate{lhs, rhs} => lhs.evaluate().powf(rhs.evaluate()),
            Expression::Factorial(n) => {
                let num = n.evaluate();
                if num == f64::INFINITY {
                    num
                } else if num < 0.0 || num.fract() != 0.0 {
                    f64::NAN
                } else {
                    (1..=num.trunc() as i64).fold(1.0, |a,b| a * b as f64)
                }
            },
            Expression::Modulo{lhs, rhs} => {
                // The % operator in Rust is remainder, not modulo
                let l = lhs.evaluate();
                let r = rhs.evaluate();
                ((l % r) + r) % r
            },
            Expression::Multiply{lhs, rhs} => lhs.evaluate() * rhs.evaluate(),
            Expression::Negate(n) => -n.evaluate(),
            Expression::Number(n) => *n,
            Expression::Subtract{lhs, rhs} => lhs.evaluate() - rhs.evaluate(),
        }
    }
}
