use super::{expr::*, stmt::Stmt, visitor::Visitor};

pub struct AstPrint;
impl AstPrint {
    pub fn print(&mut self, stmt: &Stmt) -> String {
        self.visit_statement(stmt)
    }
}

impl Visitor for AstPrint {
    type Output = String;

    fn visit_statement(&mut self, stmt: &Stmt) -> Self::Output {
        match stmt {
            Stmt::Expr(expr) => self.visit_expression(expr),
            Stmt::Print(expr) => format!("(print {})", self.visit_expression(expr)),
            Stmt::Var { name, initializer } => {
                format!("({} {})", name.lexeme, self.visit_expression(initializer))
            }
            Stmt::Block(statements) => {
                let mut res = "{".to_string();
                for stmt in statements {
                    res.push_str(&self.visit_statement(stmt));
                }
                res.push('}');
                res
            }
        }
    }

    fn visit_expression(&mut self, expr: &Expr) -> Self::Output {
        match expr {
            Expr::Literal(lit) => format!("{lit}"),
            Expr::Grouping(expr) => format!("(group {})", self.visit_expression(expr)),
            Expr::Unary { op, right } => {
                format!("({} {})", op.lexeme, self.visit_expression(right))
            }
            Expr::Binary { left, op, right } => format!(
                "({} {} {})",
                op.lexeme,
                self.visit_expression(left),
                self.visit_expression(right)
            ),
            Expr::Variable(name) => format!("{}", name.lexeme),
            Expr::Assign { name, val } => {
                format!("(assign {} {})", name.lexeme, self.visit_expression(val))
            }
        }
    }
}
