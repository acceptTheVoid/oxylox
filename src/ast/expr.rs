use crate::{token::Token, value::Value};

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Value),
    Unary {
        op: Token,
        right: Box<Expr>,
    },
    /// Contains name of the variable
    Variable(Token),
    Logical {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Assign {
        name: Token,
        val: Box<Expr>,
    },
}
