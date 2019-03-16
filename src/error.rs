#![warn(clippy::all)]

use std::io;
use std::fmt;
use std::num;

pub enum Error {
    IO(String),
    Parse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IO(s) => write!(f, "{}", s),
            Error::Parse(s) => write!(f, "{}", s),
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
            Error::IO(s) => Error::IO(s.to_string()),
            Error::Parse(s) => Error::Parse(s.to_string()),
        }
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(e: num::ParseFloatError) -> Self {
        Error::Parse(format!("invalid number: {}", e))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(format!("{}", e))
    }
}