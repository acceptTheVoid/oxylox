use super::expr::*;

pub trait Visitor {
    type Output;

    fn visit_binary(&mut self, bin: &Binary) -> Self::Output;
    fn visit_grouping(&mut self, group: &Grouping) -> Self::Output;
    fn visit_literal(&mut self, literal: &Literal) -> Self::Output;
    fn visit_unary(&mut self, unary: &Unary) -> Self::Output;  
}
