use crate::{token::Token, value::Value};

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
}
