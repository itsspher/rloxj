use crate::environment::Environment;
use crate::error::LoxError;
use crate::stmt::Stmt;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let env = Rc::new(RefCell::new(Environment::new()));
        Interpreter { environment: env }
    }
    pub fn interpret(&mut self, statements: Vec<Rc<dyn Stmt>>) -> Result<(), LoxError> {
        for stmt in statements {
            stmt.eval(Rc::clone(&self.environment))?;
        }
        Ok(())
    }
}
