use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

use super::error::Error;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(String),
    Ident(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Caret,
    SquareRoot,
    Percent,
    Exclamation,
    OpenParen,
    CloseParen,
    Comma,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Token::Number(n) => n,
            Token::Ident(s) => s,
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::Caret => "^",
            Token::SquareRoot => "√",
            Token::Percent => "%",
            Token::Exclamation => "!",
            Token::OpenParen => "(",
            Token::CloseParen => ")",
            Token::Comma => ",",
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
            self.iter.peek().map(|&c| Err(Error::Parse(format!("Unexpected character {}", c))))
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
            .or_else(|| self.scan_ident())
            .or_else(|| self.scan_number())
            .or_else(|| self.scan_operator())
            .or_else(|| self.scan_punctuation())
    }

    fn scan_ident(&mut self) -> Option<Token> {
        let mut name = self.next_if(|c| c.is_alphabetic())?.to_string();
        if let Some(rest) = self.next_while(|c| c.is_alphanumeric() || c == '_') {
            name.push_str(rest.as_str())
        }
        Some(Token::Ident(name))
    }

    fn scan_number(&mut self) -> Option<Token> {
        let mut num = self.next_while(|c| c.is_digit(10))?;
        if let Some(sep) = self.next_if(|c| c == '.') {
            num.push(sep);
            if let Some(dec) = self.next_while(|c| c.is_digit(10)) {
                num.push_str(&dec)
            }
        }
        if let Some(exp) = self.next_if(|c| c == 'e' || c == 'E') {
            num.push(exp);
            if let Some(sign) = self.next_if(|c| c == '-' || c == '+') {
                num.push(sign)
            }
            if let Some(n) = self.next_while(|c| c.is_digit(10)) {
                num.push_str(&n)
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
            '√' => Some(Token::SquareRoot),
            '%' => Some(Token::Percent),
            '!' => Some(Token::Exclamation),
            _ => None,
        })
    }

    fn scan_punctuation(&mut self) -> Option<Token> {
        self.next_if_token(|c| match c {
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            ',' => Some(Token::Comma),
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
