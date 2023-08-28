use crate::expr::Literal;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: i32,
    position: i32,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: i32,
        position: i32,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            position,
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type.clone()
    }

    pub fn literal(&self) -> Literal {
        self.literal.clone().unwrap()
    }

    pub fn line(&self) -> i32 {
        self.line
    }

    pub fn position(&self) -> i32 {
        self.position
    }

    pub fn lexeme(&self) -> String {
        self.lexeme.clone()
    }
}
