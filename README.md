# rustcalc

Simple REPL for mathematical expressions. Mostly a toy project to learn Rust and parsers.

## Build

Rustcalc requires [Rust](https://www.rust-lang.org). To build a release binary and execute it, run:

```sh
$ cargo build --release
$ ./target/release/rustcalc
```

Alternatively, build and run it directly with `cargo run`, or test it with `cargo test`.

## Usage

Running the binary will start a REPL (read-evaluate-print-loop):

```
$ rustcalc
> 1 + 2 * 3.14
7.28

> 2 ^ (-1/2)
0.7071067811865476

> sin(pi / 4) * √2
1

> 1e100 / (1/0)
0
```

An expression can also be given as a command-line argument, e.g. `rustcalc "1 + 2 * 3"` will output `7` and exit. Debug output can be enabled with the `--debug` switch, see `--help` for more info.

## Expressions

Rustcalc supports simple mathematical expressions with the usual operations, and has a very basic type system entirely made up of 64-bit floating-point numbers.

### Numbers

Number literals can be expressed with digits and the decimal point `.`, as well as in scientific notation using `e` to signify powers of ten - for example `314`, `3.14`, and `3.14e2`.

Numerical values are always encoded as [64-bit IEEE 754](https://en.wikipedia.org/wiki/Double-precision_floating-point_format#IEEE_754_double-precision_binary_floating-point_format:_binary64) `binary64` double-precision floating point numbers. These have a magnitude of roughly 10<sup>-307</sup> to 10<sup>308</sup>, and can represent 15 significant figures with exact precision - beyond this, significant figures are rounded to 53-bit precision.

Arithmetic using integer values is exact up to 53 bits, while arithmetic using decimal values may be inexact due to their machine representation. The special values infinity (`inf`) and not-a-number (`NaN`) are fully supported, and are typically returned for invalid or undefined operations such as division by zero and numeric overflow.

### Constants

The following case-insensitive constants are supported:

* `pi`, `π`: 3.141592653589793.
* `e`: 2.718281828459045.
* `inf`: the IEEE 754 infinity value.
* `nan`: the IEEE 754 not-a-number value.

### Prefix Operators

* `+`: the identity operation, e.g. `+2` yields `2`.
* `-`: negation, e.g. `-(1 + 2)` yields `-3`.
* `√`: square root, e.g. `√4` yields `2`.

### Postfix Operators

* `!`: factorial, e.g. `5!` yields `120`.

### Infix Operators

* `+`: addition, e.g. `2 + 3` yields `5`.
* `-`: subtraction, e.g. `2 - 3` yields `-1`.
* `*`: multiplication, e.g. `2 * 3` yields `6`.
* `/`: division, e.g. `4 / 2` yields `2`.
* `%`: modulo, e.g. `7 % 4` yields `3`. Has sign of dividend and magnitude less than divisor.
* `^`: exponentiation, e.g. `2 ^ 3` yields `8`.

### Operator Precedence

Operator precedence and associativity is listed below, and can be overridden by grouping expressions in parentheses, e.g. `(1 + 2) * 3` yields `9`.

| Operators     | Prec | Assoc |
| ------------- | :--: | :---: |
| `!`           | 4    | left  |
| `^`           | 3    | right |
| `*`, `/`, `%` | 2    | left  |
| `+`, `-`      | 1    | left  |
| `√`           | 1    | right |

### Functions

Functions are expressed as `name(a, b)`, where arguments must be numerical values. They return a single number, or `NaN` on error.

* `round(n, [d])`: rounds `n` to the number of decimals given by `d` (default 0), e.g. `round(3.14)` yields `3` and `round(3.14, 1)` yields `3.1`.
* `sqrt(n)`: returns the square root of the given number, e.g. `sqrt(4)` yields `2`.

* `radians(d)`: converts the angle `d` in degrees to radians e.g. `radians(180)` yields ~`3.14`.
* `degrees(r)`: converts the angle `r` in radians to degrees, e.g. `degrees(pi)` yields `180`.
* `sin(r)`: returns the sine of the given angle in radians, e.g. `sin(pi/2)` yields `1`.
* `cos(r)`: returns the cosine of the given angle in radians, e.g. `cos(pi)` yields `-1`.
* `tan(r)`: returns the tangent of the given angle in radians, e.g. `tan(pi/4)` yields `1`.
