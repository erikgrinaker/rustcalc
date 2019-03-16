extern crate rustcalc;

use std::f64;
use std::mem;

use rustcalc::error::Error;
use rustcalc::parser::Parser;

macro_rules! test_evaluate {
    ( $( $name:ident: ($input:expr, $expect:expr), )* ) => {
    $(
        #[test]
        fn $name() {
            let expect: Result<f64, Error> = $expect;
            let actual = Parser::new($input).parse().map(|expr| expr.evaluate());
            match expect {
                Ok(expect_value) => {
                    match actual {
                        Ok(value) => assert_eq!(expect_value, value),
                        Err(e) => assert!(false, "Error: {}", e),
                    }
                },
                Err(expect_error) => {
                    match actual {
                        Ok(value) => assert!(false, "Expected error, got {}", value),
                        Err(e) => assert_eq!(mem::discriminant(&expect_error), mem::discriminant(&e)),
                    }
                }
            }
        }
    )*
    }
}

test_evaluate! {
    empty:                  ("",            Err(Error::Parse(String::new()))),

    // Literals
    number:                 ("1",           Ok(1.0)),
    number_decimal:         ("3.14",        Ok(3.14)),
    number_decimal_multi:   ("3.14.15",     Err(Error::Parse(String::new()))),
    number_repeated:        ("1 2",         Err(Error::Parse(String::new()))),

    // Prefix operators
    assert:                 ("+1",          Ok(1.0)),
    assert_multi:           ("+++1",        Ok(1.0)),

    negate:                 ("-1",          Ok(-1.0)),
    negate_assert:          ("-+-+-1",      Ok(-1.0)),
    negate_multi:           ("---1",        Ok(-1.0)),

    // Postfix operators
    factorial:              ("5!",          Ok(120.0)),
    factorial_multi:        ("3!!",         Ok(720.0)),
    factorial_zero:         ("0!",          Ok(1.0)),

    // Infix operators
    infix_multi:            ("1 * / 2",     Err(Error::Parse(String::new()))),
    infix_bare:             ("*",           Err(Error::Parse(String::new()))),
    infix_pre:              ("* 2",         Err(Error::Parse(String::new()))),
    infix_post:             ("2 * ",        Err(Error::Parse(String::new()))),

    add:                    ("1 + 2",       Ok(3.0)),
    add_negative:           ("1 + -2",      Ok(-1.0)),
    add_precedence:         ("2 + 5 - 3",   Ok(4.0)),

    divide:                 ("6 / 2",       Ok(3.0)),
    divide_negative:        ("6 / -2",      Ok(-3.0)),
    divide_zero:            ("1 / 0",       Ok(f64::INFINITY)),

    exp:                    ("2 ^ 3",       Ok(8.0)),
    exp_fraction:           ("8 ^ (1/3)",   Ok(2.0)),
    exp_negative:           ("2 ^ -3",      Ok(0.125)),
    exp_zero:               ("2 ^ 0",       Ok(1.0)),
    exp_zero_zero:          ("0 ^ 0",       Ok(1.0)),

    modulo:                 ("5 % 2",       Ok(1.0)),
    modulo_divisible:       ("4 % 2",       Ok(0.0)),
    modulo_negative:        ("-5 % 2",      Ok(-1.0)),
    modulo_negative2:       ("5 % -2",      Ok(1.0)),

    multiply:               ("2 * 3",       Ok(6.0)),
    multiply_zero:          ("2 * 3",       Ok(6.0)),

    subtract:               ("1 - 2",       Ok(-1.0)),
    subtract_negative:      ("1 - -2",      Ok(3.0)),

    // Parenthesis
    paren_noclose:          ("(1 + 2",      Err(Error::Parse(String::new()))),
    paren_noopen:           ("1 + 2 )",     Err(Error::Parse(String::new()))),

    // Operator precedence and associativity
    assoc_add_sub:          ("7 - 4 + 2",   Ok(5.0)),
    prec_all:               ("2 ^ 3 * -4 / 2 + 4! % 2 - 3 % 4 ^ 2", Ok(-19.0)),
    prec_add_multiply:      ("2 * 3 + 1",   Ok(7.0)),
    prec_paren:             ("2 * (3 + 1)", Ok(8.0)),
}
