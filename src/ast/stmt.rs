use super::Expr;

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}
