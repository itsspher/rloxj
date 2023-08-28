use crate::environment::Environment;
use crate::error::LoxError;
use crate::expr;
use crate::lox_object::LoxObject;
use crate::token::Token;
use std::rc::Rc;

pub trait Stmt {
    fn kind(&self) -> Kind;
    fn eval(&self, env: Rc<Environment>) -> Result<LoxObject, LoxError>;
}

pub enum Kind {
    Expression,
    Print,
    Var,
}

pub struct Expression {
    pub expr: Rc<dyn expr::Expr>,
}

impl Stmt for Expression {
    fn kind(&self) -> Kind {
        Kind::Expression
    }
    fn eval(&self, env: Rc<Environment>) -> Result<LoxObject, LoxError> {
        self.expr.eval()
    }
}

pub struct Print {
    pub expr: Rc<dyn expr::Expr>,
}

impl Stmt for Print {
    fn kind(&self) -> Kind {
        Kind::Print
    }
    fn eval(&self, env: Rc<Environment>) -> Result<LoxObject, LoxError> {
        println!("{}", self.expr.eval()?.to_string());
        Ok(LoxObject::None)
    }
}

pub struct Var {
    pub name: Token,
    pub initializer: Rc<dyn expr::Expr>,
}

impl Stmt for Var {
    fn kind(&self) -> Kind {
        Kind::Var
    }
    fn eval(&self, env: Rc<Environment>) -> Result<LoxObject, LoxError> {
        let value = self.initializer.eval()?;
        env.define(self.name.lexeme(), value);
        Ok(LoxObject::None)
    }
}
