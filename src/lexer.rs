#![warn(clippy::all)]

use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

use super::error::Error;

pub enum Token {
    Number(String),
    Operator(char),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Operator(n) => write!(f, "{}", n),
        }
    }
}

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer{
            iter: input.chars().peekable(),
        }
    }

    fn consume_whitespace(&mut self) {
        self.scan_while(is_whitespace);
    }

    fn scan(&mut self) -> Option<Token> {
        self.consume_whitespace();
        match self.iter.peek() {
            Some(&c) if is_number(c) => self.scan_number(),
            Some(&c) if is_operator(c) => self.scan_operator(),
            _ => None,
        }
    }

    fn scan_number(&mut self) -> Option<Token> {
        let mut num = self.scan_while(is_number)?;
        if let Some(sep) = self.next_if(is_decimal_separator) {
            num.push(sep);
            if let Some(dec) = self.scan_while(is_number) {
                num.push_str(&dec)
            }
        }
        Some(Token::Number(num))
    }

    fn scan_operator(&mut self) -> Option<Token> {
        self.next_if(is_operator).map(Token::Operator)
    }

    fn scan_while<F>(&mut self, predicate: F) -> Option<String> where F: Fn(char) -> bool {
        let mut value = String::new();
        while let Some(c) = self.next_if(&predicate) {
            value.push(c)
        }
        Some(value).filter(|v| !v.is_empty())
    }

    fn next_if<F>(&mut self, predicate: F) -> Option<char> where F: Fn(char) -> bool {
        self.iter.peek().cloned().filter(|&c| predicate(c)).and_then(|_| self.iter.next())
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        self.scan().map(Ok).or_else(||
            self.iter.peek().map(|&c| Err(Error::ParseError(c)))
        )
    }
}

fn is_decimal_separator(c: char) -> bool {
    c == '.'
}

fn is_number(c: char) -> bool {
    c.is_digit(10)
}

fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' => true,
        _ => false,
    }
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}
