use crate::{tokentype::TokenType, value::Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub pos: usize,
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Value,
    pub line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Value, line: usize, pos: usize) -> Self {
        Self {
            pos,
            r#type,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}", self.r#type, self.lexeme, self.literal,)
    }
}
