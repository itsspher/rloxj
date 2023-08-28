use crate::error::LoxError;
use crate::lox_object::LoxObject;
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Environment {
    values: RefCell<HashMap<String, LoxObject>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn define(&mut self, name: String, value: LoxObject) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> Result<LoxObject, LoxError> {
        match self.values.borrow_mut().get(&name.lexeme()) {
            Some(x) => Ok(x.clone()),
            None => {
                let message: String = format!("Undefined variable {}.", name.lexeme());
                Err(LoxError::error(
                    name.line(),
                    message,
                    name.position().try_into().unwrap(),
                ))
            }
        }
    }
}
