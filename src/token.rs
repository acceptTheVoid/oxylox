use crate::{tokentype::TokenType, value::Value};

#[derive(Debug)]
pub struct Token {
    r#type: TokenType,
    pub lexeme: String,
    literal: Value,
    line: usize,
}

impl Token {
    pub fn new(
        r#type: TokenType,
        lexeme: String,
        literal: Value,
        line: usize,
    ) -> Self {
        Self { 
            r#type, 
            lexeme, 
            literal, 
            line 
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}",
            self.r#type, self.lexeme, self.literal,
        )
    }
}
