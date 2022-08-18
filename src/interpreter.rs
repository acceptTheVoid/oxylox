use crate::{value::Value, tokentype::TokenType, token::Token, ast::{stmt::Stmt, visitor::Visitor, Expr}};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&mut self, stmt: &Stmt) -> Result<Value, InterpretError> {
        self.visit_statement(stmt)
    }

    fn is_truthy(&self, val: &Value) -> bool {
        match *val {
            Value::Nil => false,
            Value::Bool(b) => b,
            _ => true,
        }
    }

    fn plus(&self, l: Value, r: Value, op: Token) -> Result<Value, InterpretError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
            _ => Err(InterpretError { 
                token: op, 
                msg: "Operands must be either two strings or two numbers".to_string(), 
            }),
        }
    }

    fn minus(&self, l: Value, r: Value, op: Token) -> Result<Value, InterpretError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
            _ => Err(InterpretError { 
                token: op, 
                msg: "Operands must be two numbers".to_string(), 
            }),
        }
    }

    fn slash(&self, l: Value, r: Value, op: Token) -> Result<Value, InterpretError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
            _ => Err(InterpretError { 
                token: op, 
                msg: "Operands must be two numbers".to_string(), 
            }),
        }
    }

    fn star(&self, l: Value, r: Value, op: Token) -> Result<Value, InterpretError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
            _ => Err(InterpretError { 
                token: op, 
                msg: "Operands must be two numbers".to_string(), 
            }),
        }
    }

    fn greater(&self, l: Value, r: Value, op: Token) -> Result<Value, InterpretError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
            _ => Err(InterpretError::number_op_err(op)),
        }
    }

    fn greater_eq(&self, l: Value, r: Value, op: Token) -> Result<Value, InterpretError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
            _ => Err(InterpretError::number_op_err(op)),
        }
    }

    fn less(&self, l: Value, r: Value, op: Token) -> Result<Value, InterpretError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
            _ => Err(InterpretError::number_op_err(op)),
        }
    }

    fn less_eq(&self, l: Value, r: Value, op: Token) -> Result<Value, InterpretError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
            _ => Err(InterpretError::number_op_err(op)),
        }
    }
}

impl Visitor for Interpreter {
    type Output = Result<Value, InterpretError>;

    fn visit_statement(&mut self, stmt: &Stmt) -> Self::Output {
        match *stmt {
            Stmt::Expr(ref expr) => self.visit_expression(expr),
            _ => todo!(),
        }
    }

    fn visit_expression(&mut self, expr: &crate::ast::Expr) -> Self::Output {
        match expr {
            Expr::Literal(val) => Ok(val.clone()), // TODO: Ужасный клон это говнище надо исправить
            Expr::Grouping(expr) => self.visit_expression(expr),
            Expr::Unary { op, right } => {
                let right = self.visit_expression(right)?;

                match (op.r#type, &right) {
                    (TokenType::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                    (TokenType::Minus, _) => Err(InterpretError::number_op_err(op.clone())), // TODO: Та же история
                    (TokenType::Bang, _) => Ok(Value::Bool(!self.is_truthy(&right))),
                    _ => unreachable!(),
                }
            },
            Expr::Binary { left, op, right } => {
                let left = self.visit_expression(left)?;
                let right = self.visit_expression(right)?;
                let op = op.clone();

                match op.r#type {
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
                    _ => unreachable!(),
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct InterpretError {
    pub token: Token,
    pub msg: String,
}

impl InterpretError {
    fn number_op_err(token: Token) -> Self {
        Self { token, msg: "Operands must be numbers".to_string() }
    }
}

impl std::fmt::Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InterpretError { }
