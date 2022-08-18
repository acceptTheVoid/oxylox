use super::{expr::*, visitor::Visitor, stmt::Stmt};

pub struct AstPrint;
impl AstPrint {
    pub fn print(&mut self, stmt: &Stmt) -> String {
        self.visit_statement(stmt)
    }
}

impl Visitor for AstPrint {
    type Output = String;

    fn visit_statement(&mut self, stmt: &Stmt) -> Self::Output {
        match *stmt {
            Stmt::Expr(ref expr) => self.visit_expression(expr),
            Stmt::Print(_) => unreachable!(),
        }
    }

    fn visit_expression(&mut self, expr: &Expr) -> Self::Output {
        match *expr {
            Expr::Literal(ref lit) => format!("{lit}"),
            Expr::Grouping(ref expr) => format!("(group {}", self.visit_expression(expr)),
            Expr::Unary { ref op, ref right } => format!("({} {})", op.lexeme, self.visit_expression(right)),
            Expr::Binary { 
                ref left, 
                ref op, 
                ref right 
            } => format!("({} {} {})", op.lexeme, self.visit_expression(left), self.visit_expression(right)),
        }
    }
}
