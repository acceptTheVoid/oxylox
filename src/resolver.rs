use std::collections::HashMap;

use crate::{ast::{visitor::Visitor, stmt::Stmt, Expr, TokenAstInfo}, interpreter::Interpreter, error::{Error, ParseError}};

#[derive(Debug, Clone, Copy)]
enum FunctionType {
    Fun,
    None,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    cur_function: FunctionType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            scopes: vec![],
            interpreter,
            cur_function: FunctionType::None,
        }
    }

    pub fn resolve(&mut self, stmts: &[Stmt]) -> Result<(), Vec<Error>> {
        let mut errors = vec![];
        
        for stmt in stmts {
            if let Err(e) = self.visit_statement(stmt) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &TokenAstInfo) -> Result<(), Error> {
        match self.scopes.last_mut() {
            Some(scope) => { 
                if let Some(_) = scope.insert(name.to_string(), false) {
                    return Err(Error::ParseError(ParseError {
                        token: name.clone(),
                        msg: "Already a variable with this name in this scope".into(),
                    }))
                } 
            },
            None => (),
        }
        
        Ok(())
    }

    fn define(&mut self, name: &str) {
        match self.scopes.last_mut() {
            Some(scope) => { scope.insert(name.to_string(), true); },
            None => (),
        }
    }

    fn resolve_local(&mut self, name: &TokenAstInfo) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(name.get_name()) {
                self.interpreter.resolve(name, i);
                return
            }
        }
    }

    fn resolve_function(&mut self, fun: &Stmt, fun_type: FunctionType) -> Result<(), Error> {
        let enclosing = self.cur_function;
        self.cur_function = fun_type;
        self.begin_scope();
        if let Stmt::Function { params, body, .. } = fun {
            for param in params {
                let name = param;
                self.declare(name)?;
                self.define(name.get_name());
            }

            for stmt in body {
                self.visit_statement(stmt)?;
            }
        } else {
            panic!("Well, that shouldn't happen... ICE Code: 0x1: Failed to resolve function")
        }
        self.end_scope();
        self.cur_function = enclosing;

        Ok(())
    }
}

impl<'a> Visitor for Resolver<'a> {
    type Output = Result<(), Error>;

    fn visit_expression(&mut self, expr: &crate::ast::Expr) -> Self::Output {
        match expr {
            Expr::Variable(token) => {
                self.resolve_local(token);
            }
            Expr::Assign { name, val } => {
                self.visit_expression(val)?;
                self.resolve_local(name)
            }
            Expr::Binary { left, right, .. } => {
                self.visit_expression(left)?;
                self.visit_expression(right)?;
            },
            Expr::Grouping(expr) => {
                self.visit_expression(expr)?;
            },
            Expr::Literal(_) => (),
            Expr::Unary { right, .. } => { 
                self.visit_expression(right)?;
            },
            Expr::Logical { left, right, .. } => {
                self.visit_expression(left)?;
                self.visit_expression(right)?;
            },
            Expr::Call { callee, args, .. } => {
                self.visit_expression(callee)?;

                for arg in args {
                    self.visit_expression(arg)?;
                }
            },
        }

        Ok(())
    }

    fn visit_statement(&mut self, stmt: &crate::ast::stmt::Stmt) -> Self::Output {
        match stmt {
            Stmt::Block(stmts) => {
                self.begin_scope();
                for stmt in stmts {
                    self.visit_statement(stmt)?;
                }
                self.end_scope();
            }
            Stmt::Var { name, initializer } => {
                self.declare(name)?;
                self.visit_expression(initializer)?;
                self.define(name.get_name());
            }
            Stmt::Function { name, .. } => {
                self.declare(name)?;
                self.define(name.get_name());

                self.resolve_function(stmt, FunctionType::Fun)?;
            }
            Stmt::Expr(expr) => self.visit_expression(expr)?,
            Stmt::If { cond, then, else_stmt } => {
                self.visit_expression(cond)?;
                self.visit_statement(then)?;
                if let Some(stmt) = else_stmt {
                    self.visit_statement(stmt)?;
                }
            }
            Stmt::Return { val, keyword } => {
                if let FunctionType::None = self.cur_function {
                    return Err(ParseError {
                        token: keyword.clone(),
                        msg: "Can't return from top-level code".into(),
                    }.into())
                }

                self.visit_expression(val)?;
            }
            Stmt::While { cond, body } => {
                self.visit_expression(cond)?;
                self.visit_statement(body)?;
            }
            Stmt::Print(exprs) => {
                for expr in exprs {
                    self.visit_expression(expr)?;
                }
            },
        };

        Ok(())
    }
}
