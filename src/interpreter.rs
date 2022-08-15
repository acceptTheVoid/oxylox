use crate::{ast::{visitor::ConsumeVisitor, Ast}, value::Value, tokentype::TokenType, token::Token};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&mut self, ast: Ast) -> Result<Value, InterpretError> {
        ast.consume(self)
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

impl ConsumeVisitor for Interpreter {
    type Output = Result<Value, InterpretError>;

    fn visit_binary(&mut self, bin: crate::ast::Binary) -> Self::Output {
        let left = bin.left.consume(self)?;
        let right = bin.right.consume(self)?;

        match bin.op.r#type {
            TokenType::Minus => self.minus(left, right, bin.op),
            TokenType::Slash => self.slash(left, right, bin.op),
            TokenType::Star => self.star(left, right, bin.op),
            TokenType::Plus => self.plus(left, right, bin.op),
            TokenType::Greater => self.greater(left, right, bin.op),
            TokenType::GreaterEq => self.greater_eq(left, right, bin.op),
            TokenType::Less => self.less(left, right, bin.op),
            TokenType::LessEq => self.less_eq(left, right, bin.op),
            TokenType::BangEq => Ok(Value::Bool(!left.eq(&right))),
            TokenType::EqEq => Ok(Value::Bool(left.eq(&right))),
            _ => unreachable!(),
        }
    }

    fn visit_grouping(&mut self, group: crate::ast::Grouping) -> Self::Output {
        group.expr.consume(self)
    }

    fn visit_literal(&mut self, literal: crate::ast::Literal) -> Self::Output {
        Ok(literal.val)
    }

    fn visit_unary(&mut self, unary: crate::ast::Unary) -> Self::Output {
        let right = unary.right.consume(self)?;

        match (unary.op.r#type, &right) {
            (TokenType::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
            (TokenType::Minus, _) => Err(InterpretError::number_op_err(unary.op)),
            (TokenType::Bang, _) => Ok(Value::Bool(!self.is_truthy(&right))),
            _ => unreachable!(),
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
