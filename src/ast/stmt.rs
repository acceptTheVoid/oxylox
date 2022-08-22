use crate::token::Token;

use super::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Vec<Expr>),
    Var {
        name: Token,
        initializer: Expr,
    },
    Block(Vec<Stmt>),
    If {
        cond: Expr,
        then: Box<Stmt>,
        else_stmt: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
    }
}
