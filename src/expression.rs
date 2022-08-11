use crate::{token::Token, value::Value};

trait Visitor<T> {
    fn visit_binary(&self, bin: &Binary) -> T;
    fn visit_grouping(&self, group: &Grouping) -> T;
    fn visit_literal(&self, literal: &Literal) -> T;
    fn visit_unary(&self, unary: &Unary) -> T;  
}

struct AstPrint;

impl AstPrint {
    // fn print(&self, expr: Box<dyn Expr>) -> String {
    //     expr.eval()
    // }

    fn parenthesize(&self, name: &str, exprs: &[&Box<dyn Expr>]) -> String {
        let mut s = format!("({name}");

        for expr in exprs {
            s.push_str(&format!(" {}", expr.eval().unwrap().to_string()))
        }
        s.push(')');

        s
    }
}

impl Visitor<String> for AstPrint {
    fn visit_binary(&self, bin: &Binary) -> String {
        self.parenthesize(&bin.op.lexeme, &[&bin.left, &bin.right])
    }

    fn visit_grouping(&self, group: &Grouping) -> String {
        todo!()
    }

    fn visit_literal(&self, literal: &Literal) -> String {
        todo!()
    }

    fn visit_unary(&self, unary: &Unary) -> String {
        todo!()
    }

    
}

trait Expr {
    fn eval(&self) -> Result<Value, (String, Token)>;
}

pub struct Binary {
    left: Box<dyn Expr>,
    op: Token,
    right: Box<dyn Expr>,
}

impl Expr for Binary {
    fn eval(&self) -> Result<Value, (String, Token)> {
        todo!()
    }
}

pub struct Grouping {
    expr: Box<dyn Expr>,
}

impl Expr for Grouping {
    fn eval(&self) -> Result<Value, (String, Token)> {
        todo!()
    }
}

pub struct Literal {
    val: Value,
}

impl Expr for Literal {
    fn eval(&self) -> Result<Value, (String, Token)> {
        todo!()
    }
}

pub struct Unary {
    op: Token,
    right: Box<dyn Expr>,
}

impl Expr for Unary {
    fn eval(&self) -> Result<Value, (String, Token)> {
        todo!()
    }
}
