#![warn(clippy::all)]

mod error;
mod lexer;
mod parser;

use std::io;
use std::io::Write;

use error::Error;
use parser::Parser;

fn main() -> Result<(), Error> {
    while let Some(input) = prompt(">")? {
        match evaluate(&input) {
            Ok(Some(n)) => println!("{}", n),
            Err(e) => println!("Error: {}", e),
            Ok(None) => continue,
        }
    }
    println!(); // Newline after ^D
    Ok(())
}

fn evaluate(input: &str) -> Result<Option<f64>, Error> {
    if !input.is_empty() {
        Parser::new().parse(input).map(|expr| Some(expr.evaluate()))
    } else {
        Ok(None)
    }
}

fn prompt(prompt: &str) -> Result<Option<String>, Error> {
    print!("{} ", prompt);
    io::stdout().flush()?;

    let mut line = String::new();
    if io::stdin().read_line(&mut line).unwrap() > 0 {
        Ok(Some(line.trim().to_string()))
    } else {
        Ok(None)
    }
}