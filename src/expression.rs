use std::f64;

/// Mathematical constants
#[derive(Clone, Debug)]
pub enum Constant {
    /// The base of the natural logarithm
    E,
    /// The IEEE 754 special value infinity
    Infinity,
    /// The IEEE 754 special value not-a-number (NaN)
    NaN,
    /// The ratio of a circle's circumference to its diameter
    Pi,
}

impl From<&Constant> for f64 {
    fn from(c: &Constant) -> Self {
        match c {
            Constant::E => f64::consts::E,
            Constant::Infinity => f64::INFINITY,
            Constant::NaN => f64::NAN,
            Constant::Pi => f64::consts::PI,
        }
    }
}

/// A mathematical operation or entity that evaluates to a f64
#[derive(Clone, Debug)]
pub enum Expression {
    /// Adds two terms
    Add {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// A named mathematical constant
    Constant(Constant),

    /// Returns the cosine of the argument angle in radians
    Cosine(Box<Expression>),

    /// Converts the argument from radians to degrees
    Degrees(Box<Expression>),

    /// Divides two values
    Divide {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// Raises the LHS value to the power of the RHS
    Exponentiate {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// Returns the factorial value of the argument
    Factorial(Box<Expression>),

    /// Returns the modulo of the arguments, with the sign of the RHS and
    /// magnitude less than the LHS
    Modulo {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// Multiplies the LHS by the RHS
    Multiply {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// Negates the argument
    Negate(Box<Expression>),

    /// Represents a numerical value
    Number(f64),

    /// Converts the argument from degrees to radians
    Radians(Box<Expression>),

    /// Rounds a value to a given number of decimals. Returns NaN for negative or
    /// fractional decimals.
    Round {
        value: Box<Expression>,
        decimals: Box<Expression>,
    },

    /// Returns the sine of the argument angle in radians
    Sine(Box<Expression>),

    /// Takes the square root of the argument
    SquareRoot(Box<Expression>),

    /// Subtracts the RHS from the LHS
    Subtract {
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },

    /// Returns the tangent of the argument angle in radians
    Tangent(Box<Expression>),
}

impl From<Constant> for Expression {
    fn from(c: Constant) -> Self {
        Expression::Constant(c)
    }
}

impl From<f64> for Expression {
    fn from(n: f64) -> Self {
        Expression::Number(n)
    }
}

impl From<&Expression> for f64 {
    fn from(expr: &Expression) -> Self {
        expr.evaluate()
    }
}

impl Expression {
    /// Evaluates the expression to a f64. Returns f64::NAN or f64::INFINITY on error.
    pub fn evaluate(&self) -> f64 {
        match self {
            Expression::Add { lhs, rhs } => lhs.evaluate() + rhs.evaluate(),
            Expression::Constant(c) => c.into(),
            Expression::Cosine(expr) => expr.evaluate().cos(),
            Expression::Degrees(expr) => expr.evaluate().to_degrees(),
            Expression::Divide { lhs, rhs } => lhs.evaluate() / rhs.evaluate(),
            Expression::Exponentiate { lhs, rhs } => lhs.evaluate().powf(rhs.evaluate()),
            Expression::Factorial(expr) => {
                let n = expr.evaluate();
                if n == f64::INFINITY {
                    n
                } else if n < 0.0 || n.fract() != 0.0 {
                    f64::NAN
                } else {
                    (1..=n.trunc() as i64).fold(1.0, |a, b| a * b as f64)
                }
            }
            Expression::Modulo { lhs, rhs } => {
                // The % operator in Rust is remainder, not modulo
                let l = lhs.evaluate();
                let r = rhs.evaluate();
                ((l % r) + r) % r
            }
            Expression::Multiply { lhs, rhs } => lhs.evaluate() * rhs.evaluate(),
            Expression::Negate(expr) => -expr.evaluate(),
            Expression::Number(n) => *n,
            Expression::Radians(expr) => expr.evaluate().to_radians(),
            Expression::Round { value, decimals } => {
                let n = value.evaluate();
                let d = decimals.evaluate();
                if d < 0.0 || d.fract() != 0.0 {
                    return f64::NAN;
                };
                let scale = 10_f64.powf(d);
                (scale * n).round() / scale
            }
            Expression::Sine(expr) => expr.evaluate().sin(),
            Expression::SquareRoot(expr) => expr.evaluate().sqrt(),
            Expression::Subtract { lhs, rhs } => lhs.evaluate() - rhs.evaluate(),
            Expression::Tangent(expr) => expr.evaluate().tan(),
        }
    }
}
