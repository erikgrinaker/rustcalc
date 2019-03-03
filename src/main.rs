#![warn(clippy::all)]

use std::io;
use std::io::Write;

fn main() {
    loop {
        let mut line = String::new();

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();

        let result = line.trim();
        println!("{}", result);
    }
}
