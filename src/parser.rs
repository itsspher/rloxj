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

    fn declaration(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        let result;
        if self.is_of(&[TokenType::Fun]) {
            result = self.function("function".to_string());
        } else if self.is_of(&[TokenType::Var]) {
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

    fn function(&mut self, kind: String) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        let message = format!("Expected {} name.", kind);
        let name = self.consume(TokenType::Identifier, message)?.clone();

        let message = format!("Expected '(' after {} name.", kind);
        self.consume(TokenType::LeftParen, message)?;
        let mut parameters: Vec<Token> = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(LoxError::error(
                        self.peek().line(),
                        "Can't have more than 255 parameters.".to_string(),
                        self.peek().position(),
                    ));
                }
                parameters.push(
                    self.consume(TokenType::Identifier, "Expected parameter name".to_string())?
                        .clone(),
                );
                if !self.is_of(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(
            TokenType::RightParen,
            "Expected ')' after parameters.".to_string(),
        )?;

        let message = format!("Expected '{{' before {} body.", kind);
        self.consume(TokenType::LeftBrace, message)?;
        let body = match self.block()?.kind() {
            stmt::Kind::Block(s) => s,
            _ => {
                return Err(LoxError::error(
                    self.peek().line(),
                    "Body of function somehow not a block??".to_string(),
                    self.peek().position(),
                ));
            }
        };
        Ok(Rc::new(stmt::Function {
            name,
            params: parameters,
            body,
        }))
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

    fn statement(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        if self.is_of(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.is_of(&[TokenType::LeftBrace]) {
            return self.block();
        }
        if self.is_of(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.is_of(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.is_of(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.is_of(&[TokenType::Return]) {
            return self.return_statement();
        }
        self.expression_statement()
    }

    fn return_statement(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        let keyword = self.previous().clone();
        let mut value: Option<Rc<dyn expr::Expr>> = None;
        if !self.check(&TokenType::Semicolon) {
            value = Some(self.expression()?);
        }
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after return value.".to_string(),
        )?;
        Ok(Rc::new(stmt::Return { keyword, value }))
    }

    fn for_statement(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        self.consume(
            TokenType::LeftParen,
            "Expected '(' after 'for'.".to_string(),
        )?;
        let mut initializer_null = false;
        let initializer: Rc<dyn stmt::Stmt>;
        if self.is_of(&[TokenType::Semicolon]) {
            initializer_null = true;
            initializer = Rc::new(stmt::Expression {
                expr: Rc::new(expr::Literal {
                    value: expr::LiteralKind::Nil,
                }),
            });
        } else if self.is_of(&[TokenType::Var]) {
            initializer = self.var_declaration()?;
        } else {
            initializer = self.expression_statement()?;
        }

        let condition = match !self.check(&TokenType::Semicolon) {
            true => self.expression()?,
            false => Rc::new(expr::Literal {
                value: expr::LiteralKind::True,
            }),
        };
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after loop condition.".to_string(),
        )?;

        let mut increment_null = false;
        let increment = match !self.check(&TokenType::RightParen) {
            true => self.expression()?,
            false => {
                increment_null = true;
                Rc::new(expr::Literal {
                    value: expr::LiteralKind::Nil,
                })
            }
        };
        self.consume(
            TokenType::RightParen,
            "Expected ')' after 'for' clause.".to_string(),
        )?;

        let mut body = self.statement()?;

        if !increment_null {
            body = Rc::new(stmt::Block {
                statements: vec![body, Rc::new(stmt::Expression { expr: increment })],
                function_block: false,
            })
        }

        body = Rc::new(stmt::While { condition, body });

        if !initializer_null {
            body = Rc::new(stmt::Block {
                statements: vec![initializer, body],
                function_block: false,
            })
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        self.consume(
            TokenType::LeftParen,
            "Expected '(' after 'while'".to_string(),
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expected ')' after condition".to_string(),
        )?;
        let body = self.statement()?;

        Ok(Rc::new(stmt::While { condition, body }))
    }

    fn if_statement(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'.".to_string())?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Expected ')' after condition.".to_string(),
        )?;

        let then_branch = self.statement()?;
        let mut else_branch: Rc<dyn stmt::Stmt> = Rc::new(stmt::Expression {
            expr: Rc::new(expr::Literal {
                value: expr::LiteralKind::Nil,
            }),
        });
        if self.is_of(&[TokenType::Else]) {
            else_branch = self.statement()?;
        };

        Ok(Rc::new(stmt::If {
            condition,
            then_branch,
            else_branch,
        }))
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

    fn block(&mut self) -> Result<Rc<dyn stmt::Stmt>, LoxError> {
        let mut statements: Vec<Rc<dyn stmt::Stmt>> = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(
            TokenType::RightBrace,
            "Expected '}' after block.".to_string(),
        )?;
        Ok(Rc::new(stmt::Block {
            statements,
            function_block: false,
        }))
    }

    fn expression(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        let expr = self.or()?;
        if self.is_of(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match expr.kind() {
                expr::Kind::Variable(name) => return Ok(Rc::new(expr::Assign { name, value })),
                _ => {
                    return Err(LoxError::error(
                        equals.line(),
                        "Invalid assignment target.".to_string(),
                        equals.position().try_into().unwrap(),
                    ))
                }
            };
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        let mut expr = self.and()?;
        while self.is_of(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Rc::new(expr::Logical {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        let mut expr = self.equality()?;
        while self.is_of(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Rc::new(expr::Logical {
                left: expr,
                operator,
                right,
            });
        }
        Ok(expr)
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
                operator,
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
        self.call()
    }

    fn call(&mut self) -> Result<Rc<dyn expr::Expr>, LoxError> {
        let mut expr = self.primary()?;

        loop {
            if self.is_of(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Rc<dyn expr::Expr>) -> Result<Rc<dyn expr::Expr>, LoxError> {
        let mut arguments: Vec<Rc<dyn expr::Expr>> = Vec::new();
        if !self.check(&TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(LoxError::error(
                        self.peek().line(),
                        "Can't have more than 255 arguments.".to_string(),
                        self.peek().position().try_into().unwrap(),
                    ));
                }
                arguments.push(self.expression()?);
                if !self.is_of(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(
            TokenType::RightParen,
            "Expected ')' after arguments.".to_string(),
        )?;

        Ok(Rc::new(expr::Call {
            callee,
            paren: paren.clone(),
            arguments,
        }))
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
        let message = format!("Expected expression at token {}.", self.peek().lexeme());
        Err(LoxError::error(
            self.peek().line(),
            message,
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
