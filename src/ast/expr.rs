use super::visitor::Visitor;
use crate::{token::Token, value::Value};

pub struct Binary {
    pub left: Box<Ast>,
    pub op: Token,
    pub right: Box<Ast>,
}

pub struct Grouping {
    pub expr: Box<Ast>,
}

pub struct Literal {
    pub val: Value,
}

pub struct Unary {
    pub op: Token,
    pub right: Box<Ast>,
}

pub enum Ast {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Ast {
    pub fn accept<V: Visitor>(&self, visitor: &mut V) -> V::Output {
        match *self {
            Self::Binary(ref bin) => visitor.visit_binary(bin),
            Self::Grouping(ref group) => visitor.visit_grouping(group),
            Self::Literal(ref literal) => visitor.visit_literal(literal),
            Self::Unary(ref unary) => visitor.visit_unary(unary),
        }
    }
}
