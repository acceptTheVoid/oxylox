use super::{Expr, TokenAstInfo};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Print(Vec<Expr>),
    Var {
        name: TokenAstInfo,
        initializer: Expr,
    },
    Block(Vec<Stmt>),
    If {
        cond: Expr,
        then: Box<Stmt>,
        else_stmt: Option<Box<Stmt>>,
    },
    While {
        cond: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: TokenAstInfo,
        params: Vec<TokenAstInfo>,
        body: Vec<Stmt>,
    },
    Return {
        keyword: TokenAstInfo,
        val: Expr,
    },
}
