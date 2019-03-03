#![warn(clippy::all)]

use std::io;
use std::io::Write;

fn main() {
    loop {
        let mut line = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut line) {
            Ok(n) => {
                if n == 0 {
                    println!();
                    break;
                }
                let result = line.trim();
                if !result.is_empty() {
                    println!("{}", result);
                }
            }
            Err(error) => {
                panic!(error);
            }
        }
    }
}
