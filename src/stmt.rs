use crate::environment::Environment;
use crate::error::LoxError;
use crate::expr;
use crate::lox_object::LoxObject;
use crate::token::Token;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Stmt {
    fn kind(&self) -> Kind;
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError>;
}

pub enum Kind {
    Expression,
    Print,
    Var,
    Block(Vec<Rc<dyn Stmt>>),
    If,
    While,
    Function,
}

pub struct Expression {
    pub expr: Rc<dyn expr::Expr>,
}

impl Stmt for Expression {
    fn kind(&self) -> Kind {
        Kind::Expression
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        self.expr.eval(env)
    }
}

pub struct Print {
    pub expr: Rc<dyn expr::Expr>,
}

impl Stmt for Print {
    fn kind(&self) -> Kind {
        Kind::Print
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        println!("{}", self.expr.eval(env)?.to_string());
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
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        let value = self.initializer.eval(Rc::clone(&env))?;
        env.borrow_mut()
            .define(self.name.lexeme().clone(), value.clone());
        Ok(LoxObject::None)
    }
}

pub struct Block {
    pub statements: Vec<Rc<dyn Stmt>>,
}

impl Stmt for Block {
    fn kind(&self) -> Kind {
        Kind::Block(self.statements.clone())
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        let scoped_env = Rc::new(RefCell::new(Environment::new_with_enclosing(env)));
        for stmt in &self.statements {
            stmt.eval(Rc::clone(&scoped_env))?;
        }
        Ok(LoxObject::None)
    }
}

pub struct If {
    pub condition: Rc<dyn expr::Expr>,
    pub then_branch: Rc<dyn Stmt>,
    pub else_branch: Rc<dyn Stmt>,
}

impl Stmt for If {
    fn kind(&self) -> Kind {
        Kind::If
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        match is_truthy(self.condition.eval(Rc::clone(&env))?) {
            true => self.then_branch.eval(Rc::clone(&env))?,
            false => self.else_branch.eval(Rc::clone(&env))?,
        };
        Ok(LoxObject::None)
    }
}

pub struct While {
    pub condition: Rc<dyn expr::Expr>,
    pub body: Rc<dyn Stmt>,
}

impl Stmt for While {
    fn kind(&self) -> Kind {
        Kind::While
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        while is_truthy(self.condition.eval(Rc::clone(&env))?) {
            self.body.eval(Rc::clone(&env))?;
        }

        Ok(LoxObject::None)
    }
}

pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Block,
}

impl Stmt for Function {
    fn kind(&self) -> Kind {
        Kind::Function
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        let function = LoxObject::Function(Rc::new(crate::lox_object::Function {
            arity: self.params.len(),
            declaration: Rc::new(self),
        }));
    }
}

pub fn is_truthy(object: LoxObject) -> bool {
    match object {
        LoxObject::None | LoxObject::Nil => false,
        LoxObject::Bool(b) => b,
        _ => true,
    }
}
