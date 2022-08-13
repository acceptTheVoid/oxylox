use super::{visitor::Visitor, expr::*};

pub struct AstPrint;
impl AstPrint {
    pub fn print(&mut self, expr: &Ast) -> String {
        expr.accept(self)
    }
}

impl Visitor for AstPrint {
    type Output = String;

    fn visit_binary(&mut self, bin: &Binary) -> Self::Output {
        format!("({} {} {})", bin.op.lexeme, bin.left.accept(self), bin.right.accept(self))
    }

    fn visit_grouping(&mut self, group: &Grouping) -> Self::Output {
        format!("(group {})", group.expr.accept(self))
    }

    fn visit_literal(&mut self, literal: &Literal) -> Self::Output {
        format!("{}", literal.val)
    }

    fn visit_unary(&mut self, unary: &Unary) -> Self::Output {
        format!("({} {})", unary.op.lexeme, unary.right.accept(self))
    }
}
