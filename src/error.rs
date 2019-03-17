use rustyline::error::ReadlineError;

use std::fmt;
use std::io;
use std::num;

#[derive(Clone, PartialEq)]
pub enum Error {
    IO(String),
    Parse(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(s) | Error::Parse(s) => write!(f, "{}", s),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err.to_string())
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(err: num::ParseFloatError) -> Self {
        Error::Parse(err.to_string())
    }
}

impl From<ReadlineError> for Error {
    fn from(err: ReadlineError) -> Self {
        Error::IO(err.to_string())
    }
}
