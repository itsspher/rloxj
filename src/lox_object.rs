use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    error::LoxError,
    stmt::{self, Stmt},
};

#[derive(PartialEq, Clone)]
pub enum LoxObject {
    None,
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Function(Rc<Function>),
}

impl LoxObject {
    pub fn display(&self) {
        match self {
            LoxObject::None => {}
            LoxObject::Nil => println!("nil"),
            LoxObject::Bool(b) => println!("{}", b),
            LoxObject::Number(n) => println!("{}", n),
            LoxObject::String(s) => println!("{}", s),
            LoxObject::Function(_) => println!("Function entered"),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            LoxObject::None => "".to_string(),
            LoxObject::Nil => "nil".to_string(),
            LoxObject::Bool(b) => b.to_string(),
            LoxObject::Number(n) => n.to_string(),
            LoxObject::String(s) => s.clone(),
            LoxObject::Function(_) => "Function callable".to_string(),
        }
    }
}

pub struct Function {
    pub arity: usize,
    pub declaration: Rc<&stmt::Function>,
}

impl Function {
    pub fn call(
        &self,
        env: Rc<RefCell<Environment>>,
        args: Vec<LoxObject>,
    ) -> Result<LoxObject, LoxError> {
        let mut scoped_env = Environment::new_with_enclosing(env);
        for (pos, _val) in args.clone().into_iter().enumerate() {
            scoped_env.define(self.declaration.params[pos].lexeme(), args[pos].clone());
        }
        self.declaration
            .body
            .eval(Rc::clone(&Rc::new(RefCell::new(scoped_env))))
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        let arity_match = self.arity == other.arity;
        let declaration_match = Rc::ptr_eq(&self.declaration, &other.declaration);
        arity_match && declaration_match
    }
}
