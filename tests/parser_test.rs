extern crate rustcalc;

use std::f64;

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
                Ok(v) if v.is_nan() => assert!(actual.unwrap().is_nan(), "Expected NaN"),
                _ => assert_eq!(expect, actual),
            }
        }
    )*
    }
}

test_evaluate! {
    empty:                  ("",            Err(Error::Parse("Unexpected end of input".into()))),

    // Literals
    number:                 ("1",           Ok(1.0)),
    number_decimal:         ("3.14",        Ok(3.14)),
    number_decimal_nodec:   ("3.",          Ok(3.0)),
    number_decimal_comma:   ("3,14",        Err(Error::Parse("Unexpected character ,".into()))),
    number_decimal_multi:   ("3.14.15",     Err(Error::Parse("Unexpected character .".into()))),
    number_repeated:        ("1 2",         Err(Error::Parse("Unexpected token 2".into()))),

    // Prefix operators
    prefix_bare:            ("+",           Err(Error::Parse("Unexpected end of input".into()))),
    prefix_bare_multi:      ("-+",          Err(Error::Parse("Unexpected end of input".into()))),
    prefix_multi:           ("-+-+-1",      Ok(-1.0)),

    assert:                 ("+1",          Ok(1.0)),
    assert_decimal:         ("+3.14",       Ok(3.14)),

    negate:                 ("-1",          Ok(-1.0)),
    negate_decimal:         ("-3.14",       Ok(-3.14)),

    // Postfix operators
    factorial:              ("5!",          Ok(120.0)),
    factorial_multi:        ("3!!",         Ok(720.0)),
    factorial_zero:         ("0!",          Ok(1.0)),
    factorial_precedence:   ("2 ^ 3!",      Ok(64.0)),

    // Infix operators
    infix_multi:            ("1 * / 2",     Err(Error::Parse("Unexpected token /".into()))),
    infix_bare:             ("*",           Err(Error::Parse("Unexpected token *".into()))),
    infix_pre:              ("* 2",         Err(Error::Parse("Unexpected token *".into()))),
    infix_post:             ("2 *",         Err(Error::Parse("Unexpected end of input".into()))),

    add:                    ("1 + 2",       Ok(3.0)),
    add_decimals:           ("3.1 + 3.3",   Ok(6.4)),
    add_negative:           ("1 + -2",      Ok(-1.0)),
    add_assoc:              ("2 + 5 - 3",   Ok(4.0)),

    divide:                 ("6 / 2",       Ok(3.0)),
    divide_decimals:        ("6.594 / 3.14",Ok(2.1)),
    divide_fraction:        ("7 / 3",       Ok(2.3333333333333335)),
    divide_negative:        ("6 / -2",      Ok(-3.0)),
    divide_zero:            ("1 / 0",       Ok(f64::INFINITY)),
    divide_zero_negative:   ("-1 / 0",      Ok(f64::NEG_INFINITY)),
    divide_precedence_add:  ("5 + 6 / 3",   Ok(7.0)),
    divide_precedence_sub:  ("5 - 6 / 3",   Ok(3.0)),
    divide_precedence_mult: ("3 * 4 / 2",   Ok(6.0)),
    divide_precedence_mod:  ("5 % 3 / 2",   Ok(1.0)),

    exp:                    ("2 ^ 3",       Ok(8.0)),
    exp_decimals:           ("6.25 ^ 0.5",  Ok(2.5)),
    exp_fraction:           ("8 ^ (1/3)",   Ok(2.0)),
    exp_negative:           ("2 ^ -3",      Ok(0.125)),
    exp_zero:               ("2 ^ 0",       Ok(1.0)),
    exp_zero_zero:          ("0 ^ 0",       Ok(1.0)),
    exp_assoc:              ("2 ^ 3 ^ 2",   Ok(512.0)),
    exp_prec_multiply:      ("4 * 2 ^ 3",   Ok(32.0)),
    exp_prec_divide:        ("4 / 2 ^ 3",   Ok(0.5)),
    exp_prec_modulo:        ("5 % 2 ^ 3",   Ok(5.0)),

    modulo:                 ("5 % 3",       Ok(2.0)),
    modulo_divisible:       ("4 % 2",       Ok(0.0)),
    modulo_negative:        ("-5 % 3",      Ok(1.0)),
    modulo_negative2:       ("5 % -3",      Ok(-1.0)),
    modulo_decimals:        ("6.28 % 2.2",  Ok(1.88)),
    modulo_zero:            ("1 % 0",       Ok(f64::NAN)),
    modulo_assoc:           ("7 % 4 % 2",   Ok(1.0)),
    modulo_prec_add:        ("2 + 7 % 3",   Ok(3.0)),
    modulo_prec_subtract:   ("2 - 7 % 3",   Ok(1.0)),
    modulo_prec_divide:     ("6 / 2 % 3",   Ok(0.0)),
    modulo_prec_multiply:   ("5 * 2 % 3",   Ok(1.0)),

    multiply:               ("2 * 3",       Ok(6.0)),
    multiply_negative:      ("2 * -3",      Ok(-6.0)),
    multiply_zero:          ("2 * 0",       Ok(0.0)),
    multiply_decimal:       ("3.14 * 2.1",  Ok(6.594)),
    multiply_prec_add:      ("1 + 2 * 3",   Ok(7.0)),
    multiply_prec_subtract: ("1 - 2 * 3",   Ok(-5.0)),
    multiply_prec_divide:   ("4 / 2 * 3",   Ok(6.0)),
    multiply_prec_modulo:   ("7 % 4 * 2",   Ok(6.0)),

    subtract:               ("1 - 2",       Ok(-1.0)),
    subtract_negative:      ("1 - -2",      Ok(3.0)),
    subtract_decimal:       ("3.14 - 2.1",  Ok(1.04)),
    subtract_assoc:         ("5 - 2 + 4",   Ok(7.0)),

    // Parenthesis
    paren_precedence:       ("(2 + 3)!",    Ok(120.0)),
    paren_noclose:          ("(1 + 2",      Err(Error::Parse("Closing ) not found".into()))),
    paren_noopen:           ("1 + 2 )",     Err(Error::Parse("Unexpected token )".into()))),
}
