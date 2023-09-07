use std::{collections::HashMap, rc::Rc};

use crate::{expr::Expr, interpreter::Interpreter, stmt, token::Token};

pub struct Resolver {
    pub interpreter: Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(&self, interpreter: Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn end_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: Token) {
        let scope: &mut HashMap<String, bool> = match self.scopes.last_mut() {
            Some(s) => s,
            None => return,
        };
        scope.insert(name.lexeme(), false);
    }

    pub fn define(&mut self, name: Token) {
        match self.scopes.last_mut() {
            None => return,
            Some(s) => s.insert(name.lexeme(), true),
        };
    }

    pub fn resolve_local(&mut self, expr: Rc<dyn Expr>, name: Token) {
        for i in (0..=self.scopes.len() - 1).rev() {
            if self.scopes.get(i).unwrap().contains_key(&name.lexeme()) {
                self.interpreter.resolve(
                    expr.clone(),
                    (self.scopes.len() - 1 - i).try_into().unwrap(),
                )
            }
        }
    }

    pub fn resolve_function(&mut self, function: Rc<stmt::Function>) {
        self.begin_scope();
        for param in &function.params {
            self.declare(param.clone());
            self.define(param.clone());
        }
        self.end_scope();
    }
}
