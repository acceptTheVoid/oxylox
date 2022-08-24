use crate::value::Value;

use super::{stmt::Stmt, TokenAstInfo};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: TokenAstInfo,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Value),
    Unary {
        op: TokenAstInfo,
        right: Box<Expr>,
    },
    /// Contains name of the variable
    Variable(TokenAstInfo),
    Logical {
        left: Box<Expr>,
        op: TokenAstInfo,
        right: Box<Expr>,
    },
    Assign {
        name: TokenAstInfo,
        val: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: TokenAstInfo,
        args: Vec<Expr>,
    },
    Lambda {
        params: Vec<TokenAstInfo>,
        body: Vec<Stmt>,
    },
}
