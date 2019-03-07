#![warn(clippy::all)]

mod error;
mod lexer;

use std::io;
use std::io::Write;

use error::Error;
use lexer::Lexer;

fn main() -> Result<(), Error> {
    'main: while let Some(expr) = prompt(">") {
        for result in Lexer::new(&expr) {
            match result {
                Ok(token) => println!("{}", token),
                Err(e) => {
                    println!("Error: {}", e);
                    continue 'main;
                },
            }
        }
    }
    println!(); // Newline after ^D
    Ok(())
}

fn prompt(prompt: &str) -> Option<String> {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();

    let mut line = String::new();
    if io::stdin().read_line(&mut line).unwrap() > 0 {
        Some(line.trim().to_string())
    } else {
        None
    }
}