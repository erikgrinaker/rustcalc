extern crate clap;
extern crate rustyline;

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use rustyline::error::ReadlineError;
use rustyline::Editor;

use super::error::Error;
use super::parser::Parser;

/// The main CLI application
pub struct CLI {
    debug: bool,
    prompt: Editor<()>,
}

impl Default for CLI {
    fn default() -> Self {
        Self::new()
    }
}

impl CLI {
    /// Creates a new CLI application
    pub fn new() -> Self {
        Self {
            debug: false,
            prompt: Editor::<()>::new(),
        }
    }

    /// Parses and evaluates the input expression, returning the numerical result
    fn evaluate(&mut self, input: &str) -> Result<Option<f64>, Error> {
        if !input.is_empty() {
            let expr = Parser::new(input).parse()?;
            if self.debug {
                println!("{:#?}", expr);
            }
            Ok(Some(expr.evaluate()))
        } else {
            Ok(None)
        }
    }

    /// Prompts the user for an input expression and returns it
    fn prompt(&mut self) -> Result<Option<String>, Error> {
        match self.prompt.readline("> ") {
            Ok(input) => {
                self.prompt.add_history_entry(input.as_ref());
                Ok(Some(input))
            }
            Err(ReadlineError::Eof) => Ok(None),
            Err(ReadlineError::Interrupted) => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    /// Runs the CLI application
    pub fn run(&mut self) -> Result<(), Error> {
        let opts = app_from_crate!()
            .arg(
                Arg::with_name("debug")
                    .short("d")
                    .long("debug")
                    .help("Enables debug output"),
            )
            .arg(Arg::with_name("expr").index(1))
            .get_matches();
        self.debug = opts.is_present("debug");

        if let Some(input) = opts.value_of("expr") {
            if let Some(result) = self.evaluate(&input)? {
                println!("{}", result)
            };
            return Ok(());
        }

        while let Some(input) = self.prompt()? {
            match self.evaluate(&input) {
                Ok(Some(result)) => println!("{}", result),
                Err(err) => println!("Error: {}", err),
                Ok(None) => {}
            }
        }
        Ok(())
    }
}
