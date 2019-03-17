use std::f64;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Expression {
    Add {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Constant(Constant),
    Cosine(Box<Expression>),
    Degrees(Box<Expression>),
    Divide {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Exponentiate {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Factorial(Box<Expression>),
    Modulo {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Multiply {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Negate(Box<Expression>),
    Number(f64),
    Radians(Box<Expression>),
    Round {
        value: Box<Expression>,
        decimals: Box<Expression>,
    },
    Sine(Box<Expression>),
    SquareRoot(Box<Expression>),
    Subtract {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Tangent(Box<Expression>),
}

impl Expression {
    pub fn evaluate(&self) -> f64 {
        match self {
            Expression::Add { lhs, rhs } => lhs.evaluate() + rhs.evaluate(),
            Expression::Constant(c) => c.value(),
            Expression::Cosine(n) => n.evaluate().cos(),
            Expression::Degrees(n) => n.evaluate().to_degrees(),
            Expression::Divide { lhs, rhs } => lhs.evaluate() / rhs.evaluate(),
            Expression::Exponentiate { lhs, rhs } => lhs.evaluate().powf(rhs.evaluate()),
            Expression::Factorial(n) => {
                let num = n.evaluate();
                if num == f64::INFINITY {
                    num
                } else if num < 0.0 || num.fract() != 0.0 {
                    f64::NAN
                } else {
                    (1..=num.trunc() as i64).fold(1.0, |a, b| a * b as f64)
                }
            }
            Expression::Modulo { lhs, rhs } => {
                // The % operator in Rust is remainder, not modulo
                let l = lhs.evaluate();
                let r = rhs.evaluate();
                ((l % r) + r) % r
            }
            Expression::Multiply { lhs, rhs } => lhs.evaluate() * rhs.evaluate(),
            Expression::Negate(n) => -n.evaluate(),
            Expression::Number(n) => *n,
            Expression::Radians(n) => n.evaluate().to_radians(),
            Expression::Round { value, decimals } => {
                let n = value.evaluate();
                let d = decimals.evaluate();
                if d < 0.0 || d.fract() != 0.0 {
                    return f64::NAN;
                };
                let scale = 10_f64.powf(d);
                (scale * n).round() / scale
            }
            Expression::Sine(n) => n.evaluate().sin(),
            Expression::SquareRoot(n) => n.evaluate().sqrt(),
            Expression::Subtract { lhs, rhs } => lhs.evaluate() - rhs.evaluate(),
            Expression::Tangent(n) => n.evaluate().tan(),
        }
    }
}
