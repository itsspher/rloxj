use crate::expr::Literal;
use crate::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
    position: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
        position: usize,
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

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn lexeme(&self) -> String {
        self.lexeme.clone()
    }
}
