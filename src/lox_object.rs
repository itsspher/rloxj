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
    Function(Rc<FunctionObject>),
    ReturnValue(Rc<LoxObject>),
}

impl LoxObject {
    //pub fn display(&self) {
    //    match self {
    //        LoxObject::None => {}
    //        LoxObject::Nil => println!("nil"),
    //        LoxObject::Bool(b) => println!("{}", b),
    //        LoxObject::Number(n) => println!("{}", n),
    //        LoxObject::String(s) => println!("{}", s),
    //        LoxObject::Function(_) => println!("Function entered"),
    //        LoxObject::ReturnValue(r) => r.display(),
    //    }
    //}
    pub fn to_string(&self) -> String {
        match self {
            LoxObject::None => "".to_string(),
            LoxObject::Nil => "nil".to_string(),
            LoxObject::Bool(b) => b.to_string(),
            LoxObject::Number(n) => n.to_string(),
            LoxObject::String(s) => s.clone(),
            LoxObject::Function(_) => "Function callable".to_string(),
            LoxObject::ReturnValue(r) => r.to_string(),
        }
    }
}

pub struct FunctionObject {
    pub arity: usize,
    pub declaration: Rc<stmt::Function>,
    pub environment: Rc<RefCell<Environment>>,
}

impl FunctionObject {
    pub fn call(&self, args: Vec<LoxObject>) -> Result<LoxObject, LoxError> {
        let scoped_env = Rc::new(RefCell::new(Environment::new_with_enclosing(Rc::clone(
            &self.environment,
        ))));
        for (pos, _val) in args.clone().into_iter().enumerate() {
            scoped_env
                .borrow_mut()
                .define(self.declaration.params[pos].lexeme(), args[pos].clone());
        }
        let block = stmt::Block {
            statements: self.declaration.body.clone(),
            function_block: true,
        };

        match block.eval(Rc::clone(&scoped_env))? {
            LoxObject::ReturnValue(r) => Ok((*r).clone()),
            _ => Ok(LoxObject::Nil),
        }
    }
}

impl PartialEq for FunctionObject {
    fn eq(&self, other: &Self) -> bool {
        let arity_match = self.arity == other.arity;
        let declaration_match = Rc::ptr_eq(&self.declaration, &other.declaration);
        arity_match && declaration_match
    }
}
