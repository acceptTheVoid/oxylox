pub mod ast_printer;
pub mod expr;
pub mod stmt;
pub mod visitor;

pub use expr::*;

use crate::{token::Token, tokentype::TokenType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenAstInfo {
    // это поле можно было бы применять для ошибок но мне впадлу поэтому оно тут будет только для избегания 
    // коллизий при хеше
    pos: usize,
    pub line: usize,
    pub kind: TokenType,
    pub name: Option<String>,
}

impl TokenAstInfo {
    pub fn get_name(&self) -> &str {
        self.name.as_ref()
            .expect("Well, that shouldn't happen... ICE Code: 0x2: Tried to get name not from identifier")
    }
}

impl std::fmt::Display for TokenAstInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{} {name}", self.kind)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}

impl From<&Token> for TokenAstInfo {
    fn from(token: &Token) -> Self {
        let name = match token.r#type {
            TokenType::Identifier => Some(token.lexeme.clone()),
            _ => None,
        };

        Self {
            pos: token.pos,
            line: token.line,
            kind: token.r#type,
            name,
        }
    }
}
