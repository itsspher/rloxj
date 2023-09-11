use by_address::ByAddress;

use crate::environment::Environment;
use crate::error::LoxError;
use crate::expr;
use crate::lox_object::LoxObject;
use crate::stmt;
use crate::token_type::TokenType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum Evalable {
    Expr(Rc<dyn expr::Expr>),
    Stmt(Rc<dyn stmt::Stmt>),
}

pub trait Evaluator {
    fn eval(&mut self, expr: Evalable) -> Result<LoxObject, LoxError>;
    fn eval_literal(&mut self, expr: Rc<expr::Literal>) -> Result<LoxObject, LoxError>;
    fn eval_unary(&mut self, expr: Rc<expr::Unary>) -> Result<LoxObject, LoxError>;
    fn eval_binary(&mut self, expr: Rc<expr::Binary>) -> Result<LoxObject, LoxError>;
    fn eval_grouping(&mut self, expr: Rc<expr::Grouping>) -> Result<LoxObject, LoxError>;
    fn eval_noop(&mut self, expr: Rc<expr::NoOp>) -> Result<LoxObject, LoxError>;
    fn eval_variable(&mut self, expr: Rc<expr::Variable>) -> Result<LoxObject, LoxError>;
    fn eval_assign(&mut self, expr: Rc<expr::Assign>) -> Result<LoxObject, LoxError>;
    fn eval_logical(&mut self, expr: Rc<expr::Logical>) -> Result<LoxObject, LoxError>;
    fn eval_call(&mut self, expr: Rc<expr::Call>) -> Result<LoxObject, LoxError>;

    fn eval_expression(&mut self, stmt: Rc<stmt::Expression>) -> Result<LoxObject, LoxError>;
    fn eval_print(&mut self, stmt: Rc<stmt::Print>) -> Result<LoxObject, LoxError>;
    fn eval_var(&mut self, stmt: Rc<stmt::Var>) -> Result<LoxObject, LoxError>;
    fn eval_block(&mut self, stmt: Rc<stmt::Block>) -> Result<LoxObject, LoxError>;
    fn eval_if(&mut self, stmt: Rc<stmt::If>) -> Result<LoxObject, LoxError>;
    fn eval_while(&mut self, stmt: Rc<stmt::While>) -> Result<LoxObject, LoxError>;
    fn eval_function(&mut self, stmt: Rc<stmt::Function>) -> Result<LoxObject, LoxError>;
    fn eval_return(&mut self, stmt: Rc<stmt::Return>) -> Result<LoxObject, LoxError>;
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    locals: HashMap<ByAddress<Rc<dyn expr::Expr>>, i32>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let env = Rc::new(RefCell::new(Environment::new()));
        Interpreter {
            environment: env,
            locals: HashMap::new(),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Rc<dyn stmt::Stmt>>) -> Result<(), LoxError> {
        for stmt in statements {
            stmt.eval(Rc::clone(&self.environment))?;
        }
        Ok(())
    }
    pub fn resolve(&mut self, expr: Rc<dyn expr::Expr>, depth: i32) {
        self.locals.insert(ByAddress(Rc::clone(&expr)), depth);
    }
}

impl Evaluator for Interpreter {
    fn eval(&mut self, expr: Evalable) -> Result<LoxObject, LoxError> {
        match expr {
            Evalable::Expr(e) => match e.kind() {
                expr::Kind::Literal => match e.downcast_rc::<expr::Literal>() {
                    Ok(cast) => self.eval_literal(cast),
                    Err(_) => unreachable!(),
                },
                expr::Kind::Unary => match e.downcast_rc::<expr::Unary>() {
                    Ok(cast) => self.eval_unary(cast),
                    Err(_) => unreachable!(),
                },
                expr::Kind::Binary => match e.downcast_rc::<expr::Binary>() {
                    Ok(cast) => self.eval_binary(cast),
                    Err(_) => unreachable!(),
                },
                expr::Kind::Grouping => match e.downcast_rc::<expr::Grouping>() {
                    Ok(cast) => self.eval_grouping(cast),
                    Err(_) => unreachable!(),
                },
                expr::Kind::NoOp => match e.downcast_rc::<expr::NoOp>() {
                    Ok(cast) => self.eval_noop(cast),
                    Err(_) => unreachable!(),
                },
                expr::Kind::Variable(_) => match e.downcast_rc::<expr::Variable>() {
                    Ok(cast) => self.eval_variable(cast),
                    Err(_) => unreachable!(),
                },
                expr::Kind::Assign => match e.downcast_rc::<expr::Assign>() {
                    Ok(cast) => self.eval_assign(cast),
                    Err(_) => unreachable!(),
                },
                expr::Kind::Logical => match e.downcast_rc::<expr::Logical>() {
                    Ok(cast) => self.eval_logical(cast),
                    Err(_) => unreachable!(),
                },
                expr::Kind::Call => match e.downcast_rc::<expr::Call>() {
                    Ok(cast) => self.eval_call(cast),
                    Err(_) => unreachable!(),
                },
            },
            Evalable::Stmt(s) => match s.kind() {
                stmt::Kind::Expression => match s.downcast_rc::<stmt::Expression>() {
                    Ok(cast) => self.eval_expression(cast),
                    Err(_) => unreachable!(),
                },
                stmt::Kind::Print => match s.downcast_rc::<stmt::Print>() {
                    Ok(cast) => self.eval_print(cast),
                    Err(_) => unreachable!(),
                },
                stmt::Kind::Var => match s.downcast_rc::<stmt::Var>() {
                    Ok(cast) => self.eval_var(cast),
                    Err(_) => unreachable!(),
                },
                stmt::Kind::Block(_) => match s.downcast_rc::<stmt::Block>() {
                    Ok(cast) => self.eval_block(cast),
                    Err(_) => unreachable!(),
                },
                stmt::Kind::If => match s.downcast_rc::<stmt::If>() {
                    Ok(cast) => self.eval_if(cast),
                    Err(_) => unreachable!(),
                },
                stmt::Kind::While => match s.downcast_rc::<stmt::While>() {
                    Ok(cast) => self.eval_while(cast),
                    Err(_) => unreachable!(),
                },
                stmt::Kind::Function => match s.downcast_rc::<stmt::Function>() {
                    Ok(cast) => self.eval_function(cast),
                    Err(_) => unreachable!(),
                },
                stmt::Kind::Return => match s.downcast_rc::<stmt::Return>() {
                    Ok(cast) => self.eval_return(cast),
                    Err(_) => unreachable!(),
                },
            },
        }
    }
    fn eval_literal(&mut self, expr: Rc<expr::Literal>) -> Result<LoxObject, LoxError> {
        match &expr.value {
            expr::LiteralKind::String(s) => Ok(LoxObject::String(s.clone())),
            expr::LiteralKind::Num(n) => Ok(LoxObject::Number(n.clone())),
            expr::LiteralKind::True => Ok(LoxObject::Bool(true)),
            expr::LiteralKind::False => Ok(LoxObject::Bool(false)),
            expr::LiteralKind::Nil => Ok(LoxObject::Nil),
        }
    }
    fn eval_unary(&mut self, expr: Rc<expr::Unary>) -> Result<LoxObject, LoxError> {
        let right = self.eval(Evalable::Expr(Rc::clone(&expr.expr)))?;
        match expr.operator.token_type() {
            TokenType::Minus => {
                expr::is_num_operand(&expr.operator, &right)?;
                match right {
                    LoxObject::Number(n) => Ok(LoxObject::Number(-n)),
                    _ => unreachable!(),
                }
            }
            TokenType::Bang => match right {
                LoxObject::Bool(b) => Ok(LoxObject::Bool(!b)),
                LoxObject::Nil => Ok(LoxObject::Bool(true)),
                _ => Err(LoxError::error(
                    expr.operator.line(),
                    "Cannot convert expression to truthy/falsy.".to_string(),
                    expr.operator.position().try_into().unwrap(),
                )),
            },
            _ => unreachable!(),
        }
    }
    fn eval_binary(&mut self, expr: Rc<expr::Binary>) -> Result<LoxObject, LoxError> {
        let left = self.eval(Evalable::Expr(Rc::clone(&expr.left)))?;
        let right = self.eval(Evalable::Expr(Rc::clone(&expr.right)))?;
        match expr.operator.token_type() {
            TokenType::Minus => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Number(a - b)),
                _ => expr::throw_num_operands_error(&expr.operator),
            },
            TokenType::Slash => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Number(a / b)),
                _ => expr::throw_num_operands_error(&expr.operator),
            },
            TokenType::Star => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Number(a * b)),
                _ => expr::throw_num_operands_error(&expr.operator),
            },
            TokenType::Plus => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Number(a + b)),
                (LoxObject::String(a), LoxObject::String(b)) => Ok(LoxObject::String(a + &b)),
                _ => expr::throw_num_operands_error(&expr.operator),
            },
            TokenType::Greater => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Bool(a > b)),
                _ => expr::throw_num_operands_error(&expr.operator),
            },
            TokenType::GreaterEqual => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Bool(a >= b)),
                _ => expr::throw_num_operands_error(&expr.operator),
            },
            TokenType::Less => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Bool(a < b)),
                _ => expr::throw_num_operands_error(&expr.operator),
            },
            TokenType::LessEqual => match (left, right) {
                (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Bool(a <= b)),
                _ => expr::throw_num_operands_error(&expr.operator),
            },
            TokenType::EqualEqual => Ok(LoxObject::Bool(expr::is_equal(&left, &right))),
            TokenType::BangEqual => Ok(LoxObject::Bool(!expr::is_equal(&left, &right))),
            _ => unreachable!(),
        }
    }
    fn eval_grouping(&mut self, expr: Rc<expr::Grouping>) -> Result<LoxObject, LoxError> {
        self.eval(Evalable::Expr(Rc::clone(&expr.expr)))
    }
    fn eval_noop(&mut self, _expr: Rc<expr::NoOp>) -> Result<LoxObject, LoxError> {
        Ok(LoxObject::None)
    }
    fn eval_variable(&mut self, expr: Rc<expr::Variable>) -> Result<LoxObject, LoxError> {
        self.environment.borrow_mut().get(&expr.name)
    }
    fn eval_assign(&mut self, expr: Rc<expr::Assign>) -> Result<LoxObject, LoxError> {
        let value = self.eval(Evalable::Expr(Rc::clone(&expr.value)))?;
        self.environment
            .borrow_mut()
            .assign(&expr.name, value.clone())?;
        Ok(value)
    }
    fn eval_logical(&mut self, expr: Rc<expr::Logical>) -> Result<LoxObject, LoxError> {
        let left = self.eval(Evalable::Expr(Rc::clone(&expr.left)))?;

        if expr.operator.token_type() == TokenType::Or {
            if stmt::is_truthy(left.clone()) {
                return Ok(left);
            }
        } else {
            if !stmt::is_truthy(left.clone()) {
                return Ok(left);
            }
        }

        self.eval(Evalable::Expr(Rc::clone(&expr.right)))
    }
    fn eval_call(&mut self, expr: Rc<expr::Call>) -> Result<LoxObject, LoxError> {
        let callee = self.eval(Evalable::Expr(Rc::clone(&expr.callee)))?;
        let mut arguments: Vec<LoxObject> = Vec::new();
        for argument in &expr.arguments {
            arguments.push(self.eval(Evalable::Expr(Rc::clone(&argument)))?);
        }

        let function = match callee {
            LoxObject::Function(c) => {
                if arguments.len() != c.arity {
                    return Err(LoxError::error(
                        expr.paren.line(),
                        "Parameters and arguments mismatch in number.".to_string(),
                        expr.paren.position().try_into().unwrap(),
                    ));
                } else {
                    c
                }
            }
            _ => {
                return Err(LoxError::error(
                    expr.paren.line(),
                    "Can only call functions and classes".to_string(),
                    expr.paren.position().try_into().unwrap(),
                ))
            }
        };

        Ok(function.call(arguments)?)
    }

    fn eval_expression(&mut self, stmt: Rc<stmt::Expression>) -> Result<LoxObject, LoxError> {
        self.eval(Evalable::Expr(Rc::clone(&stmt.expr)))
    }
    fn eval_print(&mut self, stmt: Rc<stmt::Print>) -> Result<LoxObject, LoxError> {
        println!(
            "{}",
            self.eval(Evalable::Expr(Rc::clone(&stmt.expr)))?
                .to_string()
        );
        Ok(LoxObject::None)
    }
    fn eval_var(&mut self, stmt: Rc<stmt::Var>) -> Result<LoxObject, LoxError> {
        let value = self.eval(Evalable::Expr(Rc::clone(&stmt.initializer)))?;
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme().clone(), value.clone());
        Ok(LoxObject::None)
    }
    fn eval_block(&mut self, stmt: Rc<stmt::Block>) -> Result<LoxObject, LoxError> {
        let scoped_env = Rc::new(RefCell::new(Environment::new_with_enclosing(
            self.environment.clone(),
        )));
        let previous = self.environment.clone();
        self.environment = scoped_env;
        for statement in &stmt.statements {
            match self.eval(Evalable::Stmt(Rc::clone(&statement)))? {
                LoxObject::ReturnValue(r) => {
                    self.environment = previous;
                    return Ok(LoxObject::ReturnValue(r.clone()));
                }
                _ => {}
            }
        }
        self.environment = previous;
        Ok(LoxObject::None)
    }
    fn eval_if(&mut self, stmt: Rc<stmt::If>) -> Result<LoxObject, LoxError> {
        match stmt::is_truthy(self.eval(Evalable::Expr(Rc::clone(&stmt.condition)))?) {
            true => self.eval(Evalable::Stmt(Rc::clone(&stmt.then_branch))),
            false => self.eval(Evalable::Stmt(Rc::clone(&stmt.else_branch))),
        }
    }
    fn eval_while(&mut self, stmt: Rc<stmt::While>) -> Result<LoxObject, LoxError> {
        while stmt::is_truthy(self.eval(Evalable::Expr(Rc::clone(&stmt.condition)))?) {
            match self.eval(Evalable::Stmt(Rc::clone(&stmt.body)))? {
                LoxObject::ReturnValue(r) => return Ok(LoxObject::ReturnValue(r.clone())),
                _ => {}
            };
        }

        Ok(LoxObject::None)
    }
    fn eval_function(&mut self, stmt: Rc<stmt::Function>) -> Result<LoxObject, LoxError> {
        let function = LoxObject::Function(Rc::new(crate::lox_object::FunctionObject {
            arity: stmt.params.len(),
            declaration: Rc::clone(&stmt),
            environment: Rc::clone(&self.environment),
        }));
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme(), function);
        Ok(LoxObject::None)
    }
    fn eval_return(&mut self, stmt: Rc<stmt::Return>) -> Result<LoxObject, LoxError> {
        let result = match stmt.value.clone() {
            Some(s) => self.eval(Evalable::Expr(s))?,
            None => LoxObject::None,
        };
        Ok(LoxObject::ReturnValue(Rc::new(result)))
    }
}
