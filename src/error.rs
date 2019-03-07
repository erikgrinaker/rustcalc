#![warn(clippy::all)]

use std::fmt;
use std::num;

pub enum Error {
    ScanError(char),
    ParseError(String),
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Self {
        Error::ParseError(format!("invalid number: {}", e))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ScanError(c) => write!(f, "unexpected character '{}'", c),
            Error::ParseError(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}