use crate::ast::Literal;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: i32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: i32,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
    pub fn to_string(self) -> String {
        match self.literal {
            Some(x) => {
                format!("{:?} {:?} {:?}", self.token_type, self.lexeme, x)
            }
            None => {
                format!("{:?} {:?}", self.token_type, self.lexeme)
            }
        }
    }
}
