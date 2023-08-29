use crate::error::LoxError;
use crate::lox_object::LoxObject;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    pub values: HashMap<String, LoxObject>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Environment {
            values: self.values.clone(),
            enclosing: self.enclosing.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.values = source.values.clone();
        self.enclosing = source.enclosing.clone();
    }
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: LoxObject) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> Result<LoxObject, LoxError> {
        match self.values.get(&name.lexeme()) {
            Some(x) => Ok(x.clone()),
            None => match &self.enclosing {
                Some(parent) => parent.borrow_mut().get(name),
                None => {
                    let message: String = format!("Undefined variable {}.", name.lexeme());
                    Err(LoxError::error(
                        name.line(),
                        message,
                        name.position().try_into().unwrap(),
                    ))
                }
            },
        }
    }

    pub fn assign(&mut self, name: &Token, value: LoxObject) -> Result<(), LoxError> {
        if self.values.contains_key(&name.lexeme()) {
            self.values.insert(name.lexeme(), value);
            Ok(())
        } else {
            match &self.enclosing {
                Some(parent) => parent.borrow_mut().assign(name, value),
                None => {
                    let message = format!("Undefined variable {}.", name.lexeme());
                    Err(LoxError::error(
                        name.line(),
                        message,
                        name.position().try_into().unwrap(),
                    ))
                }
            }
        }
    }
}
