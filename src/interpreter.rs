use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{stmt::Stmt, visitor::Visitor, Expr, TokenAstInfo},
    environment::Environment,
    error::{Error, RuntimeError},
    function::Function,
    lox_callable::Callable,
    token::Token,
    tokentype::TokenType,
    value::Value,
};

#[derive(Default)]
pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Visitor for Interpreter {
    type Output = Result<Value, Error>;

    fn visit_statement(&mut self, stmt: &Stmt) -> Self::Output {
        match stmt {
            Stmt::Expr(expr) => {
                self.visit_expression(expr)?;
            }
            Stmt::Print(exprs) => {
                let mut res = String::new();
                for expr in exprs {
                    res += &self.visit_expression(expr)?.to_string();
                }
                println!("{res}")
            }
            Stmt::Var { name, initializer } => {
                let val = self.visit_expression(initializer)?;
                self.environment.borrow_mut().define(name, val)?;
            }
            Stmt::Block(statements) => {
                let env = Rc::new(RefCell::new(Environment::from(&self.environment)));
                self.execute_block(statements, env)?;
            }
            Stmt::If {
                cond,
                then,
                else_stmt,
            } => {
                let cond = self.visit_expression(cond)?;
                if is_truthy(&cond) {
                    self.visit_statement(then)?;
                } else if else_stmt.is_some() {
                    self.visit_statement(else_stmt.as_ref().unwrap())?;
                }
            }
            Stmt::While { cond, body } => loop {
                let cond = self.visit_expression(cond)?;
                if !is_truthy(&cond) {
                    break;
                }
                self.visit_statement(body)?;
            },
            Stmt::Function { name, params, body } => {
                let fun = Function::LoxFun {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: Rc::clone(&self.environment),
                };
                let fun = Value::Fun(fun);
                self.environment.borrow_mut().define(name, fun)?;
            }
            Stmt::Return { val, .. } => {
                let val = self.visit_expression(val)?;

                return Err(Error::Return(val));
            }
        };

        Ok(Value::Nil)
    }

    fn visit_expression(&mut self, expr: &crate::ast::Expr) -> Self::Output {
        match expr {
            Expr::Literal(val) => Ok(val.clone()),
            Expr::Grouping(expr) => self.visit_expression(expr),
            Expr::Unary { op, right } => {
                let right = self.visit_expression(right)?;

                match (op.kind, &right) {
                    (TokenType::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                    (TokenType::Minus, _) => Err(RuntimeError::number_op_err(op).into()),
                    (TokenType::Bang, _) => Ok(Value::Bool(!is_truthy(&right))),
                    _ => unreachable!(),
                }
            }
            Expr::Binary { left, op, right } => {
                let left = self.visit_expression(left)?;
                let right = self.visit_expression(right)?;
                let op = op;

                match op.kind {
                    TokenType::Minus => self.minus(left, right, op),
                    TokenType::Slash => self.slash(left, right, op),
                    TokenType::Star => self.star(left, right, op),
                    TokenType::Plus => self.plus(left, right, op),
                    TokenType::Greater => self.greater(left, right, op),
                    TokenType::GreaterEq => self.greater_eq(left, right, op),
                    TokenType::Less => self.less(left, right, op),
                    TokenType::LessEq => self.less_eq(left, right, op),
                    TokenType::BangEq => Ok(Value::Bool(!left.eq(&right))),
                    TokenType::EqEq => Ok(Value::Bool(left.eq(&right))),
                    TokenType::Percent => self.percent(left, right, op),
                    _ => unreachable!(),
                }
                .map_err(|re| re.into())
            }
            Expr::Variable(name) => self.environment.borrow().get(name),
            Expr::Assign { name, val } => {
                let val = self.visit_expression(val)?;
                self.environment.borrow_mut().assign(name, &val)?;
                Ok(val)
            }
            Expr::Logical { left, op, right } => {
                let left = self.visit_expression(left)?;

                if op.kind == TokenType::Or {
                    if is_truthy(&left) {
                        return Ok(left);
                    }
                } else if !is_truthy(&left) {
                    return Ok(left);
                }

                Ok(self.visit_expression(right)?)
            }
            Expr::Call {
                callee,
                paren,
                args,
            } => {
                let callee = self.visit_expression(callee)?;

                let mut arguments = vec![];
                for arg in args {
                    arguments.push(self.visit_expression(arg)?);
                }

                if let Value::Fun(fun) = callee {
                    if arguments.len() != fun.arity() {
                        Err(RuntimeError {
                            token: paren.clone(),
                            msg: format!(
                                "Expected {} arguments but got {}",
                                fun.arity(),
                                arguments.len()
                            ),
                        }
                        .into())
                    } else {
                        fun.call(self, &arguments)
                    }
                } else {
                    Err(RuntimeError {
                        token: paren.clone(),
                        msg: "Can only call functions and classes".into(),
                    }
                    .into())
                }
            }
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let clock = |_: &[Value]| -> Result<Value, Error> {
            match std::time::UNIX_EPOCH.elapsed() {
                Ok(d) => Ok(Value::Number(d.as_secs_f64())),
                Err(e) => panic!("{e}"),
            }
        };

        let num = |val: &[Value]| -> Result<Value, Error> {
            match &val[0] {
                Value::Number(n) => Ok(Value::Number(*n)),
                Value::Bool(b) => Ok(if *b { Value::Number(1.) } else { Value::Number(0.) }),
                Value::Nil => Ok(Value::Number(0.)),
                Value::String(s) => {
                    match s.parse() {
                        Ok(n) => Ok(Value::Number(n)),
                        Err(e) => Err(Error::NativeCallError(format!("{e}")))
                    }
                }
                _ => Err(Error::NativeCallError(format!("{} cannot be parsed into number", val[0])))
            }
        };

        let tpe = |val: &[Value]| -> Result<Value, Error> {
            let tpe = match &val[0] {
                Value::Nil => "nil".to_string(),
                Value::Bool(_) => "bool".to_string(),
                Value::Number(_) => "number".to_string(),
                Value::String(_) => "string".to_string(),
                Value::Fun(_) => format!("{}", &val[0]), 
            };

            Ok(Value::String(tpe))
        };

        let string = |val: &[Value]| -> Result<Value, Error> {
            Ok(Value::String(val[0].to_string()))
        };

        let bool = |val: &[Value]| -> Result<Value, Error> {
            Ok(Value::Bool(is_truthy(&val[0])))
        };

        let clock = Value::Fun(Function::Native {
            arity: 0,
            body: Box::new(clock),
        });

        let num = Value::Fun(Function::Native {
            arity: 1,
            body: Box::new(num),
        });

        let string = Value::Fun(Function::Native {
            arity: 1,
            body: Box::new(string),
        });

        let bool = Value::Fun(Function::Native {
            arity: 1,
            body: Box::new(bool),
        });

        let tpe = Value::Fun(Function::Native {
            arity: 1,
            body: Box::new(tpe),
        });

        let globals = Rc::new(RefCell::new(Environment::new()));
        globals.borrow_mut().define_native("clock", clock);
        globals.borrow_mut().define_native("num", num);
        globals.borrow_mut().define_native("str", string);
        globals.borrow_mut().define_native("bool", bool);
        globals.borrow_mut().define_native("type", tpe);

        let environment = Rc::clone(&globals);
        Self {
            globals,
            environment,
        }
    }

    pub fn interpret(&mut self, stmt: Vec<Stmt>) -> Result<(), Error> {
        for stmt in stmt {
            self.visit_statement(&stmt)?;
        }

        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: &[Stmt],
        env: Rc<RefCell<Environment>>,
    ) -> Result<Value, Error> {
        let prev = Rc::clone(&self.environment);
        let steps = || -> Result<Value, Error> {
            self.environment = env;
            for stmt in statements {
                self.visit_statement(stmt)?;
            }

            Ok(Value::Nil)
        };
        let res = steps();
        self.environment = prev;
        res
    }

    fn plus(&self, l: Value, r: Value, op: &TokenAstInfo) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
            _ => Err(RuntimeError {
                token: op.clone(),
                msg: "Operands must be either two strings or two numbers".to_string(),
            }),
        }
    }

    fn minus(&self, l: Value, r: Value, op: &TokenAstInfo) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
            _ => Err(RuntimeError::two_number_op_err(op)),
        }
    }

    fn slash(&self, l: Value, r: Value, op: &TokenAstInfo) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
            _ => Err(RuntimeError::two_number_op_err(op)),
        }
    }

    fn percent(&self, l: Value, r: Value, op: &TokenAstInfo) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l % r)),
            _ => Err(RuntimeError::two_number_op_err(op)),
        }
    }

    fn star(&self, l: Value, r: Value, op: &TokenAstInfo) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
            _ => Err(RuntimeError::two_number_op_err(op)),
        }
    }

    fn greater(&self, l: Value, r: Value, op: &TokenAstInfo) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
            _ => Err(RuntimeError::number_op_err(op)),
        }
    }

    fn greater_eq(&self, l: Value, r: Value, op: &TokenAstInfo) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
            _ => Err(RuntimeError::number_op_err(op)),
        }
    }

    fn less(&self, l: Value, r: Value, op: &TokenAstInfo) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
            _ => Err(RuntimeError::number_op_err(op)),
        }
    }

    fn less_eq(&self, l: Value, r: Value, op: &TokenAstInfo) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
            _ => Err(RuntimeError::number_op_err(op)),
        }
    }
}

fn is_truthy(val: &Value) -> bool {
    match *val {
        Value::Nil => false,
        Value::Bool(b) => b,
        _ => true,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenInfo {
    pub lexeme: String,
    pub line: usize,
}

impl From<&Token> for TokenInfo {
    fn from(token: &Token) -> Self {
        Self {
            lexeme: token.lexeme.clone(),
            line: token.line,
        }
    }
}
