use crate::environment::Environment;
use crate::error::LoxError;
use crate::expr;
use crate::lox_object::LoxObject;
use crate::resolver::Resolver;
use crate::token::Token;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Stmt: downcast_rs::Downcast {
    fn kind(&self) -> Kind;
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError>;
    fn resolve(self: Rc<Self>, resolver: Rc<RefCell<&mut Resolver>>) -> Result<(), LoxError>;
}
downcast_rs::impl_downcast!(Stmt);

pub enum Kind {
    Expression,
    Print,
    Var,
    Block(Vec<Rc<dyn Stmt>>),
    If,
    While,
    Function,
    Return,
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
    fn resolve(self: Rc<Self>, resolver: Rc<RefCell<&mut Resolver>>) -> Result<(), LoxError> {
        Rc::clone(&self.expr).resolve(Rc::clone(&resolver))?;
        Ok(())
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
    fn resolve(self: Rc<Self>, resolver: Rc<RefCell<&mut Resolver>>) -> Result<(), LoxError> {
        Rc::clone(&self.expr).resolve(Rc::clone(&resolver))?;
        Ok(())
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
    fn resolve(self: Rc<Self>, resolver: Rc<RefCell<&mut Resolver>>) -> Result<(), LoxError> {
        resolver.borrow_mut().declare(self.name.clone());
        Rc::clone(&self.initializer).resolve(Rc::clone(&resolver))?;
        resolver.borrow_mut().define(self.name.clone());
        Ok(())
    }
}

pub struct Block {
    pub statements: Vec<Rc<dyn Stmt>>,
    pub function_block: bool,
}

impl Stmt for Block {
    fn kind(&self) -> Kind {
        Kind::Block(self.statements.clone())
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        let scoped_env = Rc::new(RefCell::new(Environment::new_with_enclosing(env)));
        for stmt in &self.statements {
            match stmt.eval(Rc::clone(&scoped_env))? {
                LoxObject::ReturnValue(r) => return Ok(LoxObject::ReturnValue(r.clone())),
                _ => {}
            }
        }
        Ok(LoxObject::None)
    }
    fn resolve(self: Rc<Self>, resolver: Rc<RefCell<&mut Resolver>>) -> Result<(), LoxError> {
        resolver.borrow_mut().begin_scope();
        for statement in &self.statements {
            Rc::clone(statement).resolve(Rc::clone(&resolver))?;
        }
        resolver.borrow_mut().end_scope();
        Ok(())
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
            true => self.then_branch.eval(Rc::clone(&env)),
            false => self.else_branch.eval(Rc::clone(&env)),
        }
    }
    fn resolve(self: Rc<Self>, resolver: Rc<RefCell<&mut Resolver>>) -> Result<(), LoxError> {
        Rc::clone(&self.condition).resolve(Rc::clone(&resolver))?;
        Rc::clone(&self.then_branch).resolve(Rc::clone(&resolver))?;
        Rc::clone(&self.else_branch).resolve(Rc::clone(&resolver))?;
        Ok(())
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
            match self.body.eval(Rc::clone(&env))? {
                LoxObject::ReturnValue(r) => return Ok(LoxObject::ReturnValue(r.clone())),
                _ => {}
            };
        }

        Ok(LoxObject::None)
    }
    fn resolve(self: Rc<Self>, resolver: Rc<RefCell<&mut Resolver>>) -> Result<(), LoxError> {
        Rc::clone(&self.condition).resolve(Rc::clone(&resolver))?;
        Rc::clone(&self.body).resolve(Rc::clone(&resolver))?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Rc<dyn Stmt>>,
}

impl Stmt for Function {
    fn kind(&self) -> Kind {
        Kind::Function
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        let function = LoxObject::Function(Rc::new(crate::lox_object::FunctionObject {
            arity: self.params.len(),
            declaration: Rc::new(self.clone()),
            environment: Rc::clone(&env),
        }));
        env.borrow_mut().define(self.name.lexeme(), function);
        Ok(LoxObject::None)
    }
    fn resolve(self: Rc<Self>, resolver: Rc<RefCell<&mut Resolver>>) -> Result<(), LoxError> {
        resolver.borrow_mut().declare(self.name.clone());
        resolver.borrow_mut().define(self.name.clone());
        resolver.borrow_mut().resolve_function(Rc::clone(&self));
        Ok(())
    }
}

pub struct Return {
    pub keyword: Token,
    pub value: Option<Rc<dyn expr::Expr>>,
}

impl Stmt for Return {
    fn kind(&self) -> Kind {
        Kind::Return
    }
    fn eval(&self, env: Rc<RefCell<Environment>>) -> Result<LoxObject, LoxError> {
        let result = match self.value.clone() {
            Some(s) => s.eval(env)?,
            None => LoxObject::None,
        };
        Ok(LoxObject::ReturnValue(Rc::new(result)))
    }
    fn resolve(self: Rc<Self>, resolver: Rc<RefCell<&mut Resolver>>) -> Result<(), LoxError> {
        match &self.value {
            Some(s) => Rc::clone(&s).resolve(Rc::clone(&resolver))?,
            None => {}
        };
        Ok(())
    }
}

pub fn is_truthy(object: LoxObject) -> bool {
    match object {
        LoxObject::None | LoxObject::Nil => false,
        LoxObject::Bool(b) => b,
        _ => true,
    }
}
