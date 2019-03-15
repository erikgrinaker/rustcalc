extern crate rustcalc;

use rustcalc::parser::Parser;
use std::f64;

macro_rules! test_evaluate {
    ( $( $name:ident: $value:expr, )* ) => {
    $(
        #[test]
        fn $name() {
            let (input, expect) = $value;
            assert_eq!(expect, Parser::new(input).parse().unwrap().evaluate())
        }
    )*
    }
}

macro_rules! test_error {
    ( $( $name:ident: $input:expr ),* ) => {
    $(
        #[test]
        fn $name() {
            assert!(Parser::new($input).parse().is_err(), "Expected error")
        }
    )*
    }
}

test_evaluate! {
    // Literals
    number:             ("1",           1.0),
    number_decimal:     ("3.14",        3.14),

    // Prefix operators
    assert:             ("+1",          1.0),
    assert_multi:       ("+++1",        1.0),

    negate:             ("-1",          -1.0),
    negate_assert:      ("-+-+-1",      -1.0),
    negate_multi:       ("---1",        -1.0),

    // Postfix operators
    factorial:          ("5!",          120.0),
    factorial_multi:    ("3!!",         720.0),
    factorial_zero:     ("0!",          1.0),

    // Infix operators
    add:                ("1 + 2",       3.0),
    add_negative:       ("1 + -2",      -1.0),

    divide:             ("6 / 2",       3.0),
    divide_negative:    ("6 / -2",      -3.0),
    divide_zero:        ("1 / 0",       f64::INFINITY),

    exp:                ("2 ^ 3",       8.0),
    exp_fraction:       ("8 ^ 1/3",     2.0),
    exp_negative:       ("2 ^ -3",      0.125),
    exp_zero:           ("2 ^ 0",       1.0),
    exp_zero_zero:      ("0 ^ 0",       1.0),

    modulo:             ("5 % 2",       1.0),
    modulo_divisible:   ("4 % 2",       0.0),
    modulo_negative:    ("-5 % 2",      -1.0),
    modulo_negative2:   ("5 % -2",      1.0),

    multiply:           ("2 * 3",       6.0),
    multiply_zero:      ("2 * 3",       6.0),

    subtract:           ("1 - 2",       -1.0),
    subtract_negative:  ("1 - -2",      3.0),
}

test_error! {
    empty:              "",
    infix_repeated:     "1 * / 2",
    infix_without_1:    "* 2",
    infix_without_2:    "2 * ",
    number_repeated:    "1 2",
    number_multidot:    "1.2.3",
    operator_bare:      "+"
}
