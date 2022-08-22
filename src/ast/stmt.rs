use crate::token::Token;

use super::Expr;

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var { name: Token, initializer: Expr },
    Block(Vec<Stmt>),
}
