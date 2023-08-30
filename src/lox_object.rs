#[derive(PartialEq, Clone)]
pub enum LoxObject {
    None,
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Function(Callable),
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

#[derive(PartialEq, Clone)]
pub struct Callable {
    pub arity: usize,
}

impl Callable {
    pub fn call(arguments: Vec<LoxObject>) -> LoxObject {
        LoxObject::None //TODO
    }
}
