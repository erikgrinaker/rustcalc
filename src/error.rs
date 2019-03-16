#![warn(clippy::all)]

use std::io;
use std::fmt;
use std::num;

pub enum Error {
    IOError(String),
    ParseError(String),
    ScanError(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError(s) => write!(f, "{}", s),
            Error::ScanError(c) => write!(f, "unexpected character '{}'", c),
            Error::IOError(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<&Error> for Error {
    fn from(e: &Error) -> Self {
        match e {
            Error::ParseError(s) => Error::ParseError(s.to_string()),
            Error::ScanError(c) => Error::ScanError(*c),
            Error::IOError(s) => Error::IOError(s.to_string()),
        }
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Self {
        Error::ParseError(format!("invalid number: {}", e))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IOError(format!("{}", e))
    }
}