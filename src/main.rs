#![warn(clippy::all)]

use std::io;
use std::io::Write;

fn main() {
    while let Some(expr) = prompt(">") {
        if expr.is_empty() {
            continue;
        }
        println!("{}", expr)
    }
    println!(); // Newline after ^D
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