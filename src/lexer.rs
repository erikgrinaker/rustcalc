#![warn(clippy::all)]

use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

use super::error::Error;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    Percent,
    Exclamation,
    OpenParen,
    CloseParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Token::Number(n) => n,
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::Caret => "^",
            Token::Percent => "%",
            Token::Exclamation => "!",
            Token::OpenParen => "(",
            Token::CloseParen => ")",
        })
    }
}

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        self.scan().map(Ok).or_else(||
            self.iter.peek().map(|&c| Err(Error::Parse(format!("unexpected character {}", c))))
        )
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer{
            iter: input.chars().peekable(),
        }
    }

    fn consume_whitespace(&mut self) {
        self.next_while(|c| c.is_whitespace());
    }

    fn scan(&mut self) -> Option<Token> {
        self.consume_whitespace();
        None
            .or_else(|| self.scan_number())
            .or_else(|| self.scan_operator())
            .or_else(|| self.scan_parens())
    }

    fn scan_number(&mut self) -> Option<Token> {
        let mut num = self.next_while(|c| c.is_digit(10))?;
        if let Some(sep) = self.next_if(|c| c == '.') {
            num.push(sep);
            if let Some(dec) = self.next_while(|c| c.is_digit(10)) {
                num.push_str(&dec)
            }
        }
        Some(Token::Number(num))
    }

    fn scan_operator(&mut self) -> Option<Token> {
        self.next_if_token(|c| match c {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            '^' => Some(Token::Caret),
            '%' => Some(Token::Percent),
            '!' => Some(Token::Exclamation),
            _ => None,
        })
    }

    fn scan_parens(&mut self) -> Option<Token> {
        self.next_if_token(|c| match c {
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            _ => None,
        })
    }

    fn next_if<F>(&mut self, predicate: F) -> Option<char> where F: Fn(char) -> bool {
        self.iter.peek().cloned().filter(|&c| predicate(c)).and_then(|_| self.iter.next())
    }

    fn next_if_token<F>(&mut self, predicate: F) -> Option<Token> where F: Fn(char) -> Option<Token> {
        let token = self.iter.peek().and_then(|&c| predicate(c))?;
        self.iter.next();
        Some(token)
    }

    fn next_while<F>(&mut self, predicate: F) -> Option<String> where F: Fn(char) -> bool {
        let mut value = String::new();
        while let Some(c) = self.next_if(&predicate) {
            value.push(c)
        }
        Some(value).filter(|v| !v.is_empty())
    }
}
