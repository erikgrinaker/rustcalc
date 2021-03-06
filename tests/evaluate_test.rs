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
    constant_e:             ("e",           Ok(f64::consts::E)),
    constant_e_uppercase:   ("E",           Ok(f64::consts::E)),
    constant_pi:            ("pi",          Ok(f64::consts::PI)),
    constant_pi_utf8:       ("π",           Ok(f64::consts::PI)),
    constant_pi_utf8_cap:   ("Π",           Ok(f64::consts::PI)),
    constant_inf:           ("inf",         Ok(f64::INFINITY)),
    constant_nan:           ("nan",         Ok(f64::NAN)),
    constant_nan_mixedcase: ("NaN",         Ok(f64::NAN)),
    constant_unknown:       ("x",           Err(Error::Parse("Unknown constant x".into()))),
    constant_unknown_full:  ("a_LoNg_1",    Err(Error::Parse("Unknown constant a_LoNg_1".into()))),
    constant_unknown_hyphen:("a-constant",  Err(Error::Parse("Unknown constant a".into()))),
    constant_unknown_utf8:  ("銹",          Err(Error::Parse("Unknown constant 銹".into()))),
    constant_unknown_emoji: ("👋",          Err(Error::Parse("Unexpected character 👋".into()))),
    constant_unknown_num:   ("1pi",         Err(Error::Parse("Unexpected token pi".into()))),

    number:                 ("1",           Ok(1.0)),
    number_decimal:         ("3.14",        Ok(3.14)),
    number_decimal_nodec:   ("3.",          Ok(3.0)),
    number_decimal_comma:   ("3,14",        Err(Error::Parse("Unexpected token ,".into()))),
    number_decimal_multi:   ("3.14.15",     Err(Error::Parse("Unexpected character .".into()))),
    number_repeated:        ("1 2",         Err(Error::Parse("Unexpected token 2".into()))),
    number_sci:             ("3e2",         Ok(300.0)),
    number_sci_zero:        ("3e0",         Ok(3.0)),
    number_sci_capital:     ("3E2",         Ok(300.0)),
    number_sci_dec_base:    ("3.14e1",      Ok(31.4)),
    number_sci_dec_exp:     ("3e2.1",       Err(Error::Parse("Unexpected character .".into()))),
    number_sci_neg_base:    ("-3.14e1",     Ok(-31.4)),
    number_sci_neg_exp:     ("3.14e-2",     Ok(0.0314)),
    number_sci_no_exp:      ("3e",          Err(Error::Parse("invalid float literal".into()))),
    number_sci_exp_plus:    ("3.14e+2",     Ok(314.0)),
    number_sci_exp_signs:   ("3.14e--2",    Err(Error::Parse("invalid float literal".into()))),

    // Prefix operators
    prefix_bare:            ("+",           Err(Error::Parse("Unexpected end of input".into()))),
    prefix_bare_multi:      ("-+",          Err(Error::Parse("Unexpected end of input".into()))),
    prefix_multi:           ("-+-+-1",      Ok(-1.0)),

    identity:               ("+1",          Ok(1.0)),
    identity_decimal:       ("+3.14",       Ok(3.14)),
    identity_infinity:      ("+inf",        Ok(f64::INFINITY)),
    identity_nan:           ("+nan",        Ok(f64::NAN)),

    negate:                 ("-1",          Ok(-1.0)),
    negate_decimal:         ("-3.14",       Ok(-3.14)),
    negate_infinity:        ("-inf",        Ok(f64::NEG_INFINITY)),
    negate_nan:             ("-nan",        Ok(f64::NAN)),

    sqrt:                   ("√4",          Ok(2.0)),
    sqrt_negative:          ("√-4",         Ok(f64::NAN)),
    sqrt_decimal:           ("√4.84",       Ok(2.2)),
    sqrt_zero:              ("√0",          Ok(0.0)),
    sqrt_infinity:          ("√inf",        Ok(f64::INFINITY)),
    sqrt_infinity_neg:      ("√-inf",       Ok(f64::NAN)),
    sqrt_nan:               ("√nan",        Ok(f64::NAN)),

    // Postfix operators
    factorial:              ("5!",          Ok(120.0)),
    factorial_multi:        ("3!!",         Ok(720.0)),
    factorial_zero:         ("0!",          Ok(1.0)),
    factorial_decimal:      ("3.14!",       Ok(f64::NAN)),
    factorial_negative:     ("-1!",         Ok(f64::NAN)),
    factorial_precedence:   ("2 ^ 3!",      Ok(64.0)),
    factorial_infinity:     ("inf!",        Ok(f64::INFINITY)),
    factorial_infinity_neg: ("-inf!",       Ok(f64::NAN)),
    factorial_nan:          ("nan!",        Ok(f64::NAN)),

    // Infix operators
    infix_multi:            ("1 * / 2",     Err(Error::Parse("Expected value, found /".into()))),
    infix_bare:             ("*",           Err(Error::Parse("Expected value, found *".into()))),
    infix_pre:              ("* 2",         Err(Error::Parse("Expected value, found *".into()))),
    infix_post:             ("2 *",         Err(Error::Parse("Unexpected end of input".into()))),

    add:                    ("1 + 2",       Ok(3.0)),
    add_decimals:           ("3.1 + 3.3",   Ok(6.4)),
    add_negative:           ("1 + -2",      Ok(-1.0)),
    add_assoc:              ("2 + 5 - 3",   Ok(4.0)),
    add_inf_lhs:            ("inf + 1",     Ok(f64::INFINITY)),
    add_inf_rhs:            ("1 + inf",     Ok(f64::INFINITY)),
    add_inf_both:           ("inf + inf",   Ok(f64::INFINITY)),
    add_inf_opposite:       ("inf + -inf",  Ok(f64::NAN)),
    add_neginf_lhs:         ("-inf + 1",    Ok(f64::NEG_INFINITY)),
    add_neginf_rhs:         ("1 + -inf",    Ok(f64::NEG_INFINITY)),
    add_neginf_both:        ("-inf + -inf", Ok(f64::NEG_INFINITY)),
    add_nan_lhs:            ("nan + 1",     Ok(f64::NAN)),
    add_nan_rhs:            ("1 + nan",     Ok(f64::NAN)),
    add_nan_both:           ("nan + nan",   Ok(f64::NAN)),

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
    divide_inf_lhs:         ("inf / 2",     Ok(f64::INFINITY)),
    divide_inf_rhs:         ("2 / inf",     Ok(0.0)),
    divide_inf_both:        ("inf / inf",   Ok(f64::NAN)),
    divide_inf_opposite:    ("inf / -inf",  Ok(f64::NAN)),
    divide_neginf_lhs:      ("-inf / 2",    Ok(f64::NEG_INFINITY)),
    divide_neginf_rhs:      ("2 / -inf",    Ok(0.0)),
    divide_neginf_both:     ("-inf / -inf", Ok(f64::NAN)),
    divide_nan_lhs:         ("nan / 2",     Ok(f64::NAN)),
    divide_nan_rhs:         ("2 / nan",     Ok(f64::NAN)),
    divide_nan_both:        ("nan / nan",   Ok(f64::NAN)),

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
    exp_inf_lhs_0:          ("inf ^ 0",     Ok(1.0)),
    exp_inf_lhs_2:          ("inf ^ 2",     Ok(f64::INFINITY)),
    exp_inf_rhs:            ("2 ^ inf",     Ok(f64::INFINITY)),
    exp_inf_both:           ("inf ^ inf",   Ok(f64::INFINITY)),
    exp_inf_opposite:       ("inf ^ -inf",  Ok(0.0)),
    exp_neginf_lhs_0:       ("-inf ^ 0",    Ok(1.0)),
    exp_neginf_lhs_2:       ("-inf ^ 2",    Ok(f64::INFINITY)),
    exp_neginf_lhs_3:       ("-inf ^ 3",    Ok(f64::NEG_INFINITY)),
    exp_neginf_rhs:         ("2 ^ -inf",    Ok(0.0)),
    exp_neginf_both:        ("-inf ^ -inf", Ok(0.0)),
    exp_nan_lhs:            ("nan ^ 2",     Ok(f64::NAN)),
    exp_nan_rhs:            ("2 ^ nan",     Ok(f64::NAN)),
    exp_nan_both:           ("nan ^ nan",   Ok(f64::NAN)),

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
    modulo_inf_lhs:         ("inf % 1",     Ok(f64::NAN)),
    modulo_inf_rhs:         ("1 % inf",     Ok(f64::NAN)),
    modulo_inf_both:        ("inf % inf",   Ok(f64::NAN)),
    modulo_inf_opposite:    ("inf % -inf",  Ok(f64::NAN)),
    modulo_neginf_lhs:      ("-inf % 1",    Ok(f64::NAN)),
    modulo_neginf_rhs:      ("1 % -inf",    Ok(f64::NAN)),
    modulo_neginf_both:     ("-inf % -inf", Ok(f64::NAN)),
    modulo_nan_lhs:         ("nan % 1",     Ok(f64::NAN)),
    modulo_nan_rhs:         ("1 % nan",     Ok(f64::NAN)),
    modulo_nan_both:        ("nan % nan",   Ok(f64::NAN)),

    multiply:               ("2 * 3",       Ok(6.0)),
    multiply_negative:      ("2 * -3",      Ok(-6.0)),
    multiply_zero:          ("2 * 0",       Ok(0.0)),
    multiply_decimal:       ("3.14 * 2.1",  Ok(6.594)),
    multiply_prec_add:      ("1 + 2 * 3",   Ok(7.0)),
    multiply_prec_subtract: ("1 - 2 * 3",   Ok(-5.0)),
    multiply_prec_divide:   ("4 / 2 * 3",   Ok(6.0)),
    multiply_prec_modulo:   ("7 % 4 * 2",   Ok(6.0)),
    multiply_inf_lhs:       ("inf * 2",     Ok(f64::INFINITY)),
    multiply_inf_rhs:       ("2 * inf",     Ok(f64::INFINITY)),
    multiply_inf_both:      ("inf * inf",   Ok(f64::INFINITY)),
    multiply_inf_opposite:  ("inf * -inf",  Ok(f64::NEG_INFINITY)),
    multiply_neginf_lhs:    ("-inf * 2",    Ok(f64::NEG_INFINITY)),
    multiply_neginf_rhs:    ("2 * -inf",    Ok(f64::NEG_INFINITY)),
    multiply_neginf_both:   ("-inf * -inf", Ok(f64::INFINITY)),
    multiply_nan_lhs:       ("nan * 2",     Ok(f64::NAN)),
    multiply_nan_rhs:       ("2 * nan",     Ok(f64::NAN)),
    multiply_nan_both:      ("nan * nan",   Ok(f64::NAN)),

    subtract:               ("1 - 2",       Ok(-1.0)),
    subtract_negative:      ("1 - -2",      Ok(3.0)),
    subtract_decimal:       ("3.14 - 2.1",  Ok(1.04)),
    subtract_assoc:         ("5 - 2 + 4",   Ok(7.0)),
    subtract_inf_lhs:       ("inf - 1",     Ok(f64::INFINITY)),
    subtract_inf_rhs:       ("1 - inf",     Ok(f64::NEG_INFINITY)),
    subtract_inf_both:      ("inf - inf",   Ok(f64::NAN)),
    subtract_inf_opposite:  ("inf - -inf",  Ok(f64::INFINITY)),
    subtract_neginf_lhs:    ("-inf - 1",    Ok(f64::NEG_INFINITY)),
    subtract_neginf_rhs:    ("1 - -inf",    Ok(f64::INFINITY)),
    subtract_neginf_both:   ("-inf - -inf", Ok(f64::NAN)),
    subtract_nan_lhs:       ("nan - 1",     Ok(f64::NAN)),
    subtract_nan_rhs:       ("1 - nan",     Ok(f64::NAN)),
    subtract_nan_both:      ("nan - nan",   Ok(f64::NAN)),

    // Parenthesis
    paren_precedence:       ("(2 + 3)!",    Ok(120.0)),
    paren_noclose:          ("(1 + 2",      Err(Error::Parse("Unexpected end of input".into()))),
    paren_noopen:           ("1 + 2 )",     Err(Error::Parse("Unexpected token )".into()))),

    // Functions
    func_args:              ("sqrt(1)",                 Ok(1.0)),
    func_args_comma:        ("sqrt(,)",                 Err(Error::Parse("Expected value, found ,".into()))),
    func_args_trail_comma:  ("sqrt(1,)",                Err(Error::Parse("Expected value, found )".into()))),
    func_args_missing:      ("sqrt()",                  Err(Error::Parse("Missing argument for sqrt()".into()))),
    func_args_many:         ("sqrt(1, 2)",              Err(Error::Parse("Unexpected argument for sqrt()".into()))),
    func_args_tight:        ("sqrt(1,2)",               Err(Error::Parse("Unexpected argument for sqrt()".into()))),
    func_varargs_missing:   ("round()",                 Err(Error::Parse("Missing argument for round()".into()))),
    func_varargs_1:         ("round(1)",                Ok(1.0)),
    func_varargs_2:         ("round(1, 2)",             Ok(1.0)),
    func_varargs_many:      ("round(1, 2, 3)",          Err(Error::Parse("Unexpected argument for round()".into()))),
    func_space:             ("sqrt (1)",                Ok(1.0)),
    func_missing_close:     ("sqrt (1",                 Err(Error::Parse("Unexpected end of input".into()))),
    func_no_parens:         ("sqrt 1",                  Err(Error::Parse("Unknown constant sqrt".into()))),

    cos_zero:               ("round(cos(0), 2)",        Ok(1.0)),
    cos_1_2pi:              ("round(cos(1/2*pi), 2)",   Ok(0.0)),
    cos_pi:                 ("round(cos(pi), 2)",       Ok(-1.0)),
    cos_3_2pi:              ("round(cos(3/2*pi), 2)",   Ok(0.0)),
    cos_2pi:                ("round(cos(2*pi), 2)",     Ok(1.0)),
    cos_5_2pi:              ("round(cos(5/2*pi), 2)",   Ok(0.0)),
    cos_neg_1_2pi:          ("round(cos(-1/2*pi), 2)",  Ok(0.0)),
    cos_neg_pi:             ("round(cos(-pi), 2)",      Ok(-1.0)),
    cos_inf:                ("cos(inf)",                Ok(f64::NAN)),
    cos_neginf:             ("cos(-inf)",               Ok(f64::NAN)),
    cos_nan:                ("cos(nan)",                Ok(f64::NAN)),

    degrees:                ("degrees(pi)",             Ok(180.0)),
    degrees_2pi:            ("degrees(2*pi)",           Ok(360.0)),
    degrees_4pi:            ("degrees(4*pi)",           Ok(720.0)),
    degrees_zero:           ("degrees(0)",              Ok(0.0)),
    degrees_negative:       ("degrees(-pi)",            Ok(-180.0)),
    degrees_inf:            ("degrees(inf)",            Ok(f64::INFINITY)),
    degrees_neginf:         ("degrees(-inf)",           Ok(f64::NEG_INFINITY)),
    degrees_nan:            ("degrees(nan)",            Ok(f64::NAN)),

    radians:                ("radians(180)",            Ok(f64::consts::PI)),
    radians_360:            ("radians(360)",            Ok(2.0 * f64::consts::PI)),
    radians_720:            ("radians(720)",            Ok(4.0 * f64::consts::PI)),
    radians_zero:           ("radians(0)",              Ok(0.0)),
    radians_negative:       ("radians(-180)",           Ok(-f64::consts::PI)),
    radians_inf:            ("radians(inf)",            Ok(f64::INFINITY)),
    radians_neginf:         ("radians(-inf)",           Ok(f64::NEG_INFINITY)),
    radians_nan:            ("radians(nan)",            Ok(f64::NAN)),

    sin_zero:               ("round(sin(0), 2)",        Ok(0.0)),
    sin_1_2pi:              ("round(sin(1/2*pi), 2)",   Ok(1.0)),
    sin_pi:                 ("round(sin(pi), 2)",       Ok(0.0)),
    sin_3_2pi:              ("round(sin(3/2*pi), 2)",   Ok(-1.0)),
    sin_2pi:                ("round(sin(2*pi), 2)",     Ok(0.0)),
    sin_5_2pi:              ("round(sin(5/2*pi), 2)",   Ok(1.0)),
    sin_neg_1_2pi:          ("round(sin(-1/2*pi), 2)",  Ok(-1.0)),
    sin_neg_pi:             ("round(sin(-pi), 2)",      Ok(0.0)),
    sin_inf:                ("sin(inf)",                Ok(f64::NAN)),
    sin_neginf:             ("sin(-inf)",               Ok(f64::NAN)),
    sin_nan:                ("sin(nan)",                Ok(f64::NAN)),

    round:                  ("round(3.14)",             Ok(3.0)),
    round_0_5:              ("round(0.5)",              Ok(1.0)),
    round_minus_0_5:        ("round(-0.5)",             Ok(-1.0)),
    round_precision:        ("round(3.14, 1)",          Ok(3.1)),
    round_precision_zero:   ("round(3.14, 0)",          Ok(3.0)),
    round_precision_high:   ("round(3.14, 3)",          Ok(3.14)),
    round_precision_neg:    ("round(3.14, -1)",         Ok(f64::NAN)),
    round_precision_dec:    ("round(3.14, 1.1)",        Ok(f64::NAN)),
    round_precision_inf:    ("round(3.14, inf)",        Ok(f64::NAN)),
    round_precision_ninf:   ("round(3.14, -inf)",       Ok(f64::NAN)),
    round_precision_nan:    ("round(3.14, nan)",        Ok(f64::NAN)),
    round_inf:              ("round(inf)",              Ok(f64::INFINITY)),
    round_inf_inf:          ("round(inf, inf)",         Ok(f64::NAN)),
    round_neginf:           ("round(-inf)",             Ok(f64::NEG_INFINITY)),
    round_nan:              ("round(nan)",              Ok(f64::NAN)),

    sqrt_function:          ("sqrt(4)",             Ok(2.0)),

    tan_zero:               ("round(tan(0), 2)",        Ok(0.0)),
    tan_1_4pi:              ("round(tan(1/4*pi), 2)",   Ok(1.0)),
    tan_3_4pi:              ("round(tan(3/4*pi), 2)",   Ok(-1.0)),
    tan_pi:                 ("round(tan(pi), 2)",       Ok(0.0)),
    tan_5_4pi:              ("round(tan(5/4*pi), 2)",   Ok(1.0)),
    tan_neg_1_4pi:          ("round(tan(-1/4*pi), 2)",  Ok(-1.0)),
    tan_neg_3_4pi:          ("round(tan(-3/4*pi), 2)",  Ok(1.0)),
    tan_inf:                ("tan(inf)",                Ok(f64::NAN)),
    tan_neginf:             ("tan(-inf)",               Ok(f64::NAN)),
    tan_nan:                ("tan(nan)",                Ok(f64::NAN)),
}
