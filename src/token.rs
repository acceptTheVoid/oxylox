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
        lexeme: &'a str,
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

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}",
            self.r#type, self.lexeme, self.literal,
        )
    }
}

// impl<'a> ToString for Token<'a> {
//     fn to_string(&self) -> String {
//         format!(
//             "{} {} {}",
//             self.r#type, self.lexeme, self.literal,
//         )
//     }
// }
