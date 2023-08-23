use crate::token::Token;

pub trait Collapse {}

pub enum Expr {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
}

#[derive(Debug)]
pub enum Literal {
    Num(f64),
    String(String),
    True,
    False,
    Nil,
}

pub struct Unary {
    operator: Token,
    expr: Box<Expr>,
}

pub struct Binary {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

pub struct Grouping {
    expr: Box<Expr>,
}
