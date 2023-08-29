use crate::environment::Environment;
use crate::error::LoxError;
use crate::lox_object::LoxObject;
use crate::token::Token;
use crate::token_type::TokenType;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Expr {
    fn kind(&self) -> Kind;
    fn display(&self) -> String;
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError>;
}

#[derive(Debug)]
pub enum Kind {
    Literal,
    Unary,
    Binary,
    Grouping,
    NoOp,
    Variable(Token),
    Assign,
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
        println!("enetered display at {:?}", self.kind());
        match &self.value {
            LiteralKind::String(s) => s.clone(),
            LiteralKind::Num(n) => n.to_string(),
            LiteralKind::True => "true".to_string(),
            LiteralKind::False => "false".to_string(),
            LiteralKind::Nil => "nil".to_string(),
        }
    }

    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        match &self.value {
            LiteralKind::String(s) => Ok(LoxObject::String(s.clone())),
            LiteralKind::Num(n) => Ok(LoxObject::Number(n.clone())),
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
        println!("enetered display at {:?}", self.kind());
        let mut result: Vec<&str> = Vec::new();
        result.push("(");
        let binding = &self.operator.lexeme();
        result.push(binding);
        let binding = &self.expr.display();
        result.push(binding);
        result.push(")");
        result.into_iter().collect::<String>()
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        let expr = self.expr.eval(env)?;
        match self.operator.token_type() {
            TokenType::Minus => {
                is_num_operand(&self.operator, &expr)?;
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
        println!("enetered display at {:?}", self.kind());
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
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        let left = self.left.eval(Rc::clone(&env))?;
        let right = self.right.eval(Rc::clone(&env))?;
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
            TokenType::Greater => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Bool(a > b)),
                _ => throw_num_operands_error(&self.operator),
            },
            TokenType::GreaterEqual => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Bool(a >= b)),
                _ => throw_num_operands_error(&self.operator),
            },
            TokenType::Less => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Bool(a < b)),
                _ => throw_num_operands_error(&self.operator),
            },
            TokenType::LessEqual => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Bool(a <= b)),
                _ => throw_num_operands_error(&self.operator),
            },
            TokenType::EqualEqual => Ok(LoxObject::Bool(is_equal(&left, &right))),
            TokenType::BangEqual => Ok(LoxObject::Bool(!is_equal(&left, &right))),
            _ => unreachable!(),
        }
    }
}

// assumes rust's == operator has the behaviour we want
// this may not be the case though...
fn is_equal(left: &LoxObject, right: &LoxObject) -> bool {
    match (left, right) {
        (LoxObject::Nil, LoxObject::Nil) => true,
        (LoxObject::Nil, _) => false,
        (_, _) => left == right,
    }
}

fn is_num_operand(operator: &Token, expr: &LoxObject) -> Result<(), LoxError> {
    match expr {
        LoxObject::Number(_) => Ok(()),
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
        println!("enetered display at {:?}", self.kind());
        let mut result: Vec<&str> = Vec::new();
        result.push("(");
        result.push("group ");
        let binding = &self.expr.display();
        result.push(binding);
        result.push(")");
        result.into_iter().collect::<String>()
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        self.expr.eval(env)
    }
}

pub struct NoOp {}

impl Expr for NoOp {
    fn kind(&self) -> Kind {
        Kind::NoOp
    }

    fn display(&self) -> String {
        println!("enetered display at {:?}", self.kind());
        "".to_string()
    }

    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        Ok(LoxObject::Nil)
    }
}

pub struct Variable {
    pub name: Token,
}

impl Expr for Variable {
    fn kind(&self) -> Kind {
        Kind::Variable(self.name.clone())
    }

    fn display(&self) -> String {
        println!("enetered display at {:?}", self.kind());
        self.name.lexeme()
    }

    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        env.borrow_mut().get(&self.name)
    }
}

pub struct Assign {
    pub name: Token,
    pub value: Rc<dyn Expr>,
}

impl Expr for Assign {
    fn kind(&self) -> Kind {
        Kind::Assign
    }
    fn display(&self) -> String {
        println!("enetered display at {:?}", self.kind());
        self.name.lexeme()
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        let value = self.value.eval(Rc::clone(&env))?;
        env.borrow_mut().assign(&self.name, value.clone())?;
        return Ok(value);
    }
}
