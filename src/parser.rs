use crate::error::LoxError;
use crate::expr;
use crate::stmt;
use crate::token::Token;
use crate::token_type::TokenType;

use std::rc::Rc;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pub statements: Vec<Rc<dyn stmt::Stmt>>,
    pub errors: Vec<LoxError>,
    current: usize,
}

impl Parser<'_> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            statements: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while !self.is_at_end() {
            match self.declaration() {
                Ok(o) => self.statements.push(o),
                Err(e) => self.errors.push(e),
            }
        }
    }

    pub fn declaration(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        let result;
        if self.is_of(&[TokenType::Var]) {
            result = self.var_declaration();
        } else {
            result = self.statement();
        }

        match result {
            Ok(_) => {}
            Err(_) => {
                self.synchronize();
            }
        }
        result
    }

    fn var_declaration(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        let name = self
            .consume(TokenType::Identifier, "Expected variable name.".to_string())?
            .clone();
        let mut initializer: Rc<dyn expr::Expr> = Rc::new(expr::Literal {
            value: expr::LiteralKind::Nil,
        });

        if self.is_of(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.".to_string(),
        )?;

        Ok(Rc::new(stmt::Var { name, initializer }))
    }

    pub fn statement(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        if self.is_of(&[TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after value.".to_string(),
        )?;
        Ok(Rc::new(stmt::Print { expr: value }))
    }

    fn expression_statement(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after value.".to_string(),
        )?;
        Ok(Rc::new(stmt::Expression { expr }))
    }

    pub fn expression(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        let mut expr = self.comparison()?;

        while self.is_of(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Rc::new(expr::Binary {
                left: expr,
                operator,
                right,
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        let mut expr = self.term()?;

        while self.is_of(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Rc::new(expr::Binary {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        let mut expr = self.factor()?;

        while self.is_of(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Rc::new(expr::Binary {
                left: expr,
                operator,
                right,
            })
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        let mut expr = self.unary()?;

        while self.is_of(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Rc::new(expr::Binary {
                left: expr,
                operator: operator,
                right,
            })
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        if self.is_of(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Rc::new(expr::Unary {
                operator,
                expr: right,
            }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        if self.is_of(&[TokenType::False]) {
            return Ok(Rc::new(expr::Literal {
                value: expr::LiteralKind::False,
            }));
        }

        if self.is_of(&[TokenType::True]) {
            return Ok(Rc::new(expr::Literal {
                value: expr::LiteralKind::True,
            }));
        }

        if self.is_of(&[TokenType::Nil]) {
            return Ok(Rc::new(expr::Literal {
                value: expr::LiteralKind::Nil,
            }));
        }

        if self.is_of(&[TokenType::Number, TokenType::String]) {
            return Ok(Rc::new(expr::Literal {
                value: self.previous().literal().value,
            }));
        }

        if self.is_of(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            match self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            ) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
            return Ok(Rc::new(expr::Grouping { expr }));
        }
        if self.is_of(&[TokenType::Identifier]) {
            return Ok(Rc::new(expr::Variable {
                name: self.previous().clone(),
            }));
        }

        Err(LoxError::error(
            self.peek().line(),
            "Expected expression.".to_string(),
            self.peek().position().try_into().unwrap(),
        ))
    }

    // this is our match, match is a keyword
    //&[TokenType] is a slice of TokenTypes, a way to accept more than one
    //of the same typed arg in Rust
    fn is_of(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<&Token, LoxError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        Err(LoxError::error(
            self.peek().line(),
            message,
            self.peek().position().try_into().unwrap(),
        ))
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type() == token_type.clone()
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type() == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type() == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
            self.advance();
        }
    }
}
