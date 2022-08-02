use crate::tokentype::TokenType;

pub struct Token<'a> {
    r#type: TokenType,
    lexeme: &'a str,
    literal: Box<dyn std::fmt::Display>,
    line: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        r#type: TokenType,
        lexeme: &str,
        literal: Box<dyn std::fmt::Display>,
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

impl ToString for Token {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.r#type, self.lexeme, self.literal,
        )
    }
}
