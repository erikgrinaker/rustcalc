#![warn(clippy::all)]
extern crate rustyline;

mod error;
mod lexer;
mod parser;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use error::Error;
use parser::Parser;

fn main() -> Result<(), Error> {
    CLI::new().run()
}

struct CLI {
    prompt: Editor<()>,
}

impl CLI {
    fn new() -> CLI {
        CLI {
            prompt: Editor::<()>::new(),
        }
    }

    fn evaluate(&mut self, input: &str) -> Result<Option<f64>, Error> {
        if !input.is_empty() {
            Parser::new(input).parse().map(|expr| Some(expr.evaluate()))
        } else {
            Ok(None)
        }
    }

    fn prompt(&mut self) -> Result<Option<String>, Error> {
        match self.prompt.readline("> ") {
            Ok(input) => {
                self.prompt.add_history_entry(input.as_ref());
                Ok(Some(input))
            }
            Err(ReadlineError::Eof) => Ok(None),
            Err(ReadlineError::Interrupted) => Ok(None),
            Err(err) => Err(Error::IOError(format!("{}", err))),
        }
    }

    fn run(&mut self) -> Result<(), Error> {
        while let Some(input) = self.prompt()? {
            match self.evaluate(&input) {
                Ok(Some(n)) => println!("{}", n),
                Err(e) => println!("Error: {}", e),
                Ok(None) => continue,
            }
        }
        Ok(())
    }
}
