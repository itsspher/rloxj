use crate::error::LoxError;
use crate::lox_object::LoxObject;
use crate::token::Token;
use crate::token_type::TokenType;
use std::rc::Rc;

pub trait Expr {
    fn kind(&self) -> Kind;
    fn display(&self) -> String;
    fn eval(&self) -> Result<LoxObject, LoxError>;
}

pub enum Kind {
    Literal,
    Unary,
    Binary,
    Grouping,
    NoOp,
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    String(String),
    Num(f64),
    True,
    False,
    Nil,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: LiteralKind,
}

impl Expr for Literal {
    fn kind(&self) -> Kind {
        Kind::Literal
    }

    fn display(&self) -> String {
        match &self.value {
            LiteralKind::String(s) => s.clone(),
            LiteralKind::Num(n) => n.to_string(),
            LiteralKind::True => "true".to_string(),
            LiteralKind::False => "false".to_string(),
            LiteralKind::Nil => "nil".to_string(),
        }
    }

    fn eval(&self) -> Result<LoxObject, LoxError> {
        match self.value {
            LiteralKind::String(s) => Ok(LoxObject::String(s)),
            LiteralKind::Num(n) => Ok(LoxObject::Number(n)),
            LiteralKind::True => Ok(LoxObject::Bool(true)),
            LiteralKind::False => Ok(LoxObject::Bool(false)),
            LiteralKind::Nil => Ok(LoxObject::Nil),
        }
    }
}

pub struct Unary {
    pub operator: Token,
    pub expr: Rc<dyn Expr>,
}

impl Expr for Unary {
    fn kind(&self) -> Kind {
        Kind::Unary
    }

    fn display(&self) -> String {
        let mut result: Vec<&str> = Vec::new();
        result.push("(");
        let binding = &self.operator.lexeme();
        result.push(binding);
        let binding = &self.expr.display();
        result.push(binding);
        result.push(")");
        result.into_iter().collect::<String>()
    }
    fn eval(&self) -> Result<LoxObject, LoxError> {
        let expr = self.expr.eval()?;
        match self.operator.token_type() {
            TokenType::Minus => {
                is_num_operand(self.operator, expr)?;
                match expr {
                    LoxObject::Number(n) => Ok(LoxObject::Number(-n)),
                    _ => unreachable!(),
                }
            }
            TokenType::Bang => match expr {
                LoxObject::Bool(b) => Ok(LoxObject::Bool(!b)),
                LoxObject::Nil => Ok(LoxObject::Bool(true)),
                _ => Err(LoxError::error(
                    self.operator.line(),
                    "Cannot convert expression to truthy/falsy.".to_string(),
                    self.operator.position().try_into().unwrap(),
                )),
            },
            _ => unreachable!(),
        }
    }
}

pub struct Binary {
    pub left: Rc<dyn Expr>,
    pub operator: Token,
    pub right: Rc<dyn Expr>,
}

impl Expr for Binary {
    fn kind(&self) -> Kind {
        Kind::Binary
    }

    fn display(&self) -> String {
        let mut result: Vec<&str> = Vec::new();
        result.push("(");
        let binding = &self.left.display();
        result.push(binding);
        let binding = &self.operator.lexeme();
        result.push(binding.as_str());
        let binding = &self.right.display();
        result.push(binding);
        result.push(")");
        result.into_iter().collect::<String>()
    }
    fn eval(&self) -> Result<LoxObject, LoxError> {
        let left = self.left.eval()?;
        let right = self.right.eval()?;
        match self.operator.token_type() {
            TokenType::Minus => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Number(a - b)),
                _ => throw_num_operands_error(&self.operator),
            },
            TokenType::Slash => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Number(a / b)),
                _ => throw_num_operands_error(&self.operator),
            },
            TokenType::Star => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Number(a * b)),
                _ => throw_num_operands_error(&self.operator),
            },
            TokenType::Plus => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Number(a + b)),
                (LoxObject::String(a), LoxObject::String(b)) => Ok(LoxObject::String(a + &b)),
                _ => throw_num_operands_error(&self.operator),
            },
            _ => unreachable!(),
        }
    }
}

fn is_num_operand(operator: Token, expr: LoxObject) -> Result<(), LoxError> {
    match expr {
        LoxObject::Number(n) => Ok(()),
        _ => Err(LoxError::error(
            operator.line(),
            "Operand must be number.".to_string(),
            operator.position().try_into().unwrap(),
        )),
    }
}

fn throw_num_operands_error(operator: &Token) -> Result<LoxObject, LoxError> {
    Err(LoxError::error(
        operator.line(),
        "Operands must both be numbers.".to_string(),
        operator.position().try_into().unwrap(),
    ))
}

pub struct Grouping {
    pub expr: Rc<dyn Expr>,
}

impl Expr for Grouping {
    fn kind(&self) -> Kind {
        Kind::Grouping
    }

    fn display(&self) -> String {
        let mut result: Vec<&str> = Vec::new();
        result.push("(");
        result.push("group ");
        let binding = &self.expr.display();
        result.push(binding);
        result.push(")");
        result.into_iter().collect::<String>()
    }
    fn eval(&self) -> Result<LoxObject, LoxError> {
        self.expr.eval()
    }
}

pub struct NoOp {}

impl Expr for NoOp {
    fn kind(&self) -> Kind {
        Kind::NoOp
    }

    fn display(&self) -> String {
        "".to_string()
    }

    fn eval(&self) -> Result<LoxObject, LoxError> {
        Ok(LoxObject::Nil)
    }
}