#![warn(clippy::all)]

use std::fmt;

pub enum Error {
    ScanError(char)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ScanError(c) => write!(f, "unexpected character '{}'", c),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}