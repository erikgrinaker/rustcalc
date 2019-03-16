#![warn(clippy::all)]

#[derive(Debug)]
pub enum Expression {
    Add{lhs: Box<Expression>, rhs: Box<Expression>},
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
    pub fn evaluate(self) -> f64 {
        match self {
            Expression::Add{lhs, rhs} => lhs.evaluate() + rhs.evaluate(),
            Expression::Divide{lhs, rhs} => lhs.evaluate() / rhs.evaluate(),
            Expression::Exponentiate{lhs, rhs} => lhs.evaluate().powf(rhs.evaluate()),
            Expression::Factorial(n) => factorial(n.evaluate()),
            Expression::Modulo{lhs, rhs} => lhs.evaluate() % rhs.evaluate(),
            Expression::Multiply{lhs, rhs} => lhs.evaluate() * rhs.evaluate(),
            Expression::Negate(n) => -n.evaluate(),
            Expression::Number(n) => n,
            Expression::Subtract{lhs, rhs} => lhs.evaluate() - rhs.evaluate(),
        }
    }
}

fn factorial(n: f64) -> f64 {
    (1..=n.trunc() as i64).fold(1.0, |a,b| a * b as f64)
}
