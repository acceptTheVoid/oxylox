use super::{expr::*, stmt::Stmt};

pub trait Visitor {
    type Output;

    fn visit_expression(&mut self, expr: &Expr) -> Self::Output;
    fn visit_statement(&mut self, stmt: &Stmt) -> Self::Output;
}

