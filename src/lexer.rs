use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

use crate::error::Error;

// A lexer token
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// A literal number
    Number(String),
    /// A name, of a constant or function
    Ident(String),
    /// The addition symbol +
    Plus,
    /// The subtraction symbol -
    Minus,
    /// The multiplication symbol *
    Asterisk,
    /// The division symbol /
    Slash,
    /// The exponentiation symbol ^
    Caret,
    /// The square root symbol √
    SquareRoot,
    /// The modulo symbol %
    Percent,
    /// The factorial symbol !
    Exclamation,
    /// An opening parenthesis
    OpenParen,
    /// A closing parenthesis
    CloseParen,
    /// An expression separator ,
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

/// A lexer tokenizes an input string as an iterator
pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Result<Token, Error>> {
        self.scan().map(Ok).or_else(|| {
            self.iter.peek().map(|&c| Err(Error::Parse(format!("Unexpected character {}", c))))
        })
    }
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer for the given input string
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer { iter: input.chars().peekable() }
    }

    /// Consumes any whitespace characters
    fn consume_whitespace(&mut self) {
        self.next_while(|c| c.is_whitespace());
    }

    /// Grabs the next character if it matches the predicate function
    fn next_if<F>(&mut self, predicate: F) -> Option<char>
    where
        F: Fn(char) -> bool,
    {
        self.iter.peek().filter(|&c| predicate(*c))?;
        self.iter.next()
    }

    /// Grabs the next single-character token if the tokenizer function returns one
    fn next_if_token<F>(&mut self, tokenizer: F) -> Option<Token>
    where
        F: Fn(char) -> Option<Token>,
    {
        let token = self.iter.peek().and_then(|&c| tokenizer(c))?;
        self.iter.next();
        Some(token)
    }

    /// Grabs the next characters that match the predicate, as a string
    fn next_while<F>(&mut self, predicate: F) -> Option<String>
    where
        F: Fn(char) -> bool,
    {
        let mut value = String::new();
        while let Some(c) = self.next_if(&predicate) {
            value.push(c)
        }
        Some(value).filter(|v| !v.is_empty())
    }

    /// Scans the input for the next token if any, ignoring leading whitespace
    fn scan(&mut self) -> Option<Token> {
        self.consume_whitespace();
        None.or_else(|| self.scan_ident())
            .or_else(|| self.scan_number())
            .or_else(|| self.scan_operator())
            .or_else(|| self.scan_punctuation())
    }

    /// Scans the input for the next ident token, if any
    fn scan_ident(&mut self) -> Option<Token> {
        let mut name = self.next_if(|c| c.is_alphabetic())?.to_string();
        while let Some(c) = self.next_if(|c| c.is_alphanumeric() || c == '_') {
            name.push(c)
        }
        Some(Token::Ident(name))
    }

    /// Scans the input for the next number token, if any
    fn scan_number(&mut self) -> Option<Token> {
        let mut num = self.next_while(|c| c.is_digit(10))?;
        if let Some(sep) = self.next_if(|c| c == '.') {
            num.push(sep);
            while let Some(dec) = self.next_if(|c| c.is_digit(10)) {
                num.push(dec)
            }
        }
        if let Some(exp) = self.next_if(|c| c == 'e' || c == 'E') {
            num.push(exp);
            if let Some(sign) = self.next_if(|c| c == '+' || c == '-') {
                num.push(sign)
            }
            while let Some(c) = self.next_if(|c| c.is_digit(10)) {
                num.push(c)
            }
        }
        Some(Token::Number(num))
    }

    /// Scans the input for the next operator token, if any
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

    /// Scans the input for the next punctuation token, if any
    fn scan_punctuation(&mut self) -> Option<Token> {
        self.next_if_token(|c| match c {
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::CloseParen),
            ',' => Some(Token::Comma),
            _ => None,
        })
    }
}
