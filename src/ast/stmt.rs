use crate::token::Token;

use super::Expr;

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var { name: Token, initializer: Expr },
}
