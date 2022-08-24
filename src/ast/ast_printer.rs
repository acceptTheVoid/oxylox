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
        #[allow(unused)]
        match stmt {
            Stmt::Expr(expr) => self.visit_expression(expr),
            Stmt::Print(expr) => {
                todo!()
                // format!("(print {})", self.visit_expression(expr))
            }
            Stmt::Var { name, initializer } => {
                todo!() // format!("({} {})", name.lexeme, self.visit_expression(initializer))
            }
            Stmt::Block(statements) => {
                let mut res = "{".to_string();
                for stmt in statements {
                    res.push_str(&self.visit_statement(stmt));
                }
                res.push('}');
                res
            }
            Stmt::If {
                cond,
                then,
                else_stmt,
            } => {
                todo!()
            }
            Stmt::While { cond, body } => {
                todo!()
            }
            Stmt::Function { name, params, body } => {
                todo!()
            }
            Stmt::Return { keyword, val } => {
                todo!()
            }
        }
    }

    fn visit_expression(&mut self, expr: &Expr) -> Self::Output {
        #[allow(unused)]
        match expr {
            Expr::Literal(lit) => format!("{lit}"),
            Expr::Grouping(expr) => format!("(group {})", self.visit_expression(expr)),
            Expr::Unary { op, right } => {
                todo!()
            }
            Expr::Binary { left, op, right } => todo!(),
            Expr::Variable(name) => todo!(), //format!("{}", name.lexeme),
            Expr::Assign { name, val } => {
                todo!() // format!("(assign {} {})", name.lexeme, self.visit_expression(val))
            }
            Expr::Logical { left, op, right } => {
                todo!()
            }
            Expr::Call {
                callee,
                paren,
                args,
            } => {
                todo!()
            }
            _ => todo!(),
        }
    }
}
