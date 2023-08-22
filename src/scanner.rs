use std::any::type_name;

use crate::error::LoxError;
use crate::token::Literal;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, Vec<LoxError>> {
        let mut lexical_errors: Vec<LoxError> = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => lexical_errors.push(e),
            };
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::from(""),
            None,
            self.line,
        ));
        match lexical_errors.len() {
            0 => Ok(&self.tokens),
            _ => Err(lexical_errors),
        }
    }

    pub fn scan_token(&mut self) -> Result<(), LoxError> {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => match self.next_char('=') {
                true => self.add_token(TokenType::BangEqual, None),
                false => self.add_token(TokenType::Bang, None),
            },
            '=' => match self.next_char('=') {
                true => self.add_token(TokenType::EqualEqual, None),
                false => self.add_token(TokenType::Equal, None),
            },
            '<' => match self.next_char('=') {
                true => self.add_token(TokenType::LessEqual, None),
                false => self.add_token(TokenType::Less, None),
            },
            '>' => match self.next_char('=') {
                true => self.add_token(TokenType::GreaterEqual, None),
                false => self.add_token(TokenType::Greater, None),
            },
            '/' => match self.next_char('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenType::Slash, None),
            },
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => match self.string() {
                Ok(_) => {}
                Err(e) => return Err(e),
            },
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' => self.identifier(),
            _ => {
                return Err(LoxError::error(
                    self.line,
                    "Unexpected character.".to_string(),
                    self.current,
                ))
            }
        };

        Ok(())
    }

    pub fn identifier(&mut self) {
        while self.is_alpha(self.peek()) || self.is_digit(self.peek()) {
            self.advance();
        }

        let text: &str = &self.source[self.start..self.current];
        let token_type: TokenType = match self.keywords(text) {
            Some(x) => x,
            None => TokenType::Identifier,
        };
        self.add_token(token_type, None)
    }

    pub fn keywords(&self, candidate: &str) -> Option<TokenType> {
        match candidate {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }

    pub fn is_alpha(&self, c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    }

    pub fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_max()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_token(
            TokenType::Number,
            Some(Literal::Num(
                self.source[self.start..self.current]
                    .parse::<f64>()
                    .unwrap(),
            )),
        );
    }

    pub fn is_digit(&self, c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    pub fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::error(
                self.line,
                "Unterminated string".to_string(),
                self.current,
            ));
        }

        self.advance();
        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(Literal::String(value)));

        Ok(())
    }

    pub fn next_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else if self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    pub fn peek_max(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len().try_into().unwrap()
    }

    pub fn advance(&mut self) -> char {
        let previous = self.current;
        self.current = self.current + 1;
        self.source
            .chars()
            .nth(previous.try_into().unwrap())
            .unwrap()
    }

    pub fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }
}
