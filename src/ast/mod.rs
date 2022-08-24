pub mod ast_printer;
pub mod expr;
pub mod stmt;
pub mod visitor;

pub use expr::*;

use crate::{token::Token, tokentype::TokenType};

#[derive(Debug, Clone)]
pub struct TokenAstInfo {
    pub line: usize,
    pub kind: TokenType,
    pub name: Option<String>,
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
            line: token.line,
            kind: token.r#type,
            name,
        }
    }
}
