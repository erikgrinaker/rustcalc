#![warn(clippy::all)]

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

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        self.scan().map(Ok).or_else(||
            self.iter.peek().map(|&c| Err(Error::ScanError(c)))
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
        self.scan_while(is_whitespace);
    }

    fn scan(&mut self) -> Option<Token> {
        self.consume_whitespace();
        None
            .or_else(|| self.scan_number())
            .or_else(|| self.scan_operator())
            .or_else(|| self.scan_parens())
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
        match self.next_if(is_operator)? {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Asterisk),
            '/' => Some(Token::Slash),
            '^' => Some(Token::Caret),
            '%' => Some(Token::Percent),
            '!' => Some(Token::Exclamation),
            _ => None,
        }
    }

    fn scan_parens(&mut self) -> Option<Token> {
        match self.next_if(is_parens)? {
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            _ => None,
        }
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

fn is_decimal_separator(c: char) -> bool {
    c == '.'
}

fn is_number(c: char) -> bool {
    c.is_digit(10)
}

fn is_operator(c: char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '^' | '%' | '!' => true,
        _ => false,
    }
}

fn is_parens(c: char) -> bool {
    match c {
        '(' | ')' => true,
        _ => false,
    }
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lexer_test(input: &str, tokens: Vec<Token>) {
        let mut lexer = Lexer::new(input);
        for token in tokens {
            assert_eq!(lexer.next().unwrap().unwrap(), token);
        };
        assert_eq!(lexer.next().is_none(), true);
    }

    #[test]
    fn addition() {
        lexer_test("1 + 3.14", vec![
            Token::Number("1".into()),
            Token::Plus,
            Token::Number("3.14".into()),
        ])
    }

    #[test]
    fn decimal() {
        lexer_test("3.14", vec![
            Token::Number("3.14".into()),
        ]);
    }

    #[test]
    fn integer() {
        lexer_test("123", vec![
            Token::Number("123".into()),
        ]);
    }

    #[test]
    fn operators() {
        lexer_test("+-*/^%!", vec![
            Token::Plus,
            Token::Minus,
            Token::Asterisk,
            Token::Slash,
            Token::Caret,
            Token::Percent,
            Token::Exclamation,
        ]);
    }
}