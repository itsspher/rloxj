use by_address::ByAddress;

use crate::environment::Environment;
use crate::error::LoxError;
use crate::expr::Expr;
use crate::stmt::Stmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<ByAddress<Rc<dyn Expr>>, i32>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let env = Rc::new(RefCell::new(Environment::new()));
        Interpreter {
            environment: env,
            locals: HashMap::new(),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Rc<dyn Stmt>>) -> Result<(), LoxError> {
        for stmt in statements {
            stmt.eval(Rc::clone(&self.environment))?;
        }
        Ok(())
    }
    pub fn resolve(&mut self, expr: Rc<dyn Expr>, depth: i32) {
        self.locals.insert(ByAddress(expr.clone()), depth);
    }
}
