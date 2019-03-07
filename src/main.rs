#![warn(clippy::all)]
extern crate rustyline;

mod error;
mod lexer;
mod parser;

use rustyline::Editor;
use rustyline::error::ReadlineError;

use error::Error;
use parser::Parser;

fn main() -> Result<(), Error> {
    let mut rl = Editor::<()>::new();
    while let Some(input) = prompt(&mut rl, "> ")? {
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

fn prompt(rl: &mut Editor<()>, prompt: &str) -> Result<Option<String>, Error> {
    match rl.readline(prompt) {
        Ok(input) => {
            rl.add_history_entry(input.as_ref());
            Ok(Some(input))
        },
        Err(ReadlineError::Eof) => Ok(None),
        Err(ReadlineError::Interrupted) => Ok(None),
        Err(err) => Err(Error::IOError(format!("{}", err))),
    }
}