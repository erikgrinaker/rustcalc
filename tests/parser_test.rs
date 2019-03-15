extern crate rustcalc;

use rustcalc::parser::Parser;

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(input: &str, result: f64) {
        assert_eq!(result, Parser::new(input).parse().unwrap().evaluate())
    }

    #[test]
    fn test_precedence() {
        eval("3 * 2 + 1", 7.0)
    }

    #[test]
    fn test_infix() {
        eval("3 + 2 - 1", 4.0)
    }

    #[test]
    fn test_prefix() {
        eval("-- +-3.14", -3.14)
    }
}