#![warn(clippy::all)]

use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

pub enum Token {
    Number(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n)
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

    fn scan_number(&mut self) -> String {
        self.scan_while(|&c| is_number(c) )
    }

    fn scan_while<F>(&mut self, f: F) -> String where F: Fn(&char) -> bool {
        let mut value = String::new();
        while let Some(c) = self.iter.peek() {
            if f(c) {
                value.push(*c);
                self.iter.next();
            } else {
                break;
            }
        }
        value
    }

    fn scan_whitespace(&mut self) -> String {
        self.scan_while(|&c| is_whitespace(c) )
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.scan_whitespace();
        match self.iter.peek() {
            Some(&c) if is_number(c) => Some(Token::Number(self.scan_number())),
            None => None,
            _ => panic!("Parse error"),
        }
    }
}

fn is_number(c: char) -> bool {
    c.is_digit(10)
}

fn is_whitespace(c :char) -> bool {
    c.is_whitespace()
}
