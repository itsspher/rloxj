use crate::environment::Environment;
use crate::error::LoxError;
use crate::stmt::Stmt;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Rc::new(Environment::new()),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Rc<dyn Stmt>>) -> Result<(), LoxError> {
        for stmt in statements {
            stmt.eval(Rc::clone(&self.environment))?;
        }
        Ok(())
    }
}