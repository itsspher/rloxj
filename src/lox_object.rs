#[derive(PartialEq, Clone)]
pub enum LoxObject {
    None,
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl LoxObject {
    pub fn display(&self) {
        match self {
            LoxObject::None => {}
            LoxObject::Nil => println!("nil"),
            LoxObject::Bool(b) => println!("{}", b),
            LoxObject::Number(n) => println!("{}", n),
            LoxObject::String(s) => println!("{}", s),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            LoxObject::None => "".to_string(),
            LoxObject::Nil => "nil".to_string(),
            LoxObject::Bool(b) => b.to_string(),
            LoxObject::Number(n) => n.to_string(),
            LoxObject::String(s) => s.clone(),
        }
    }
}
