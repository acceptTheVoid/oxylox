use crate::{
    ast::{stmt::Stmt, visitor::Visitor, Expr},
    environment::Environment,
    token::Token,
    tokentype::TokenType,
    value::Value,
};

pub struct Interpreter {
    environment: Environment,
}

impl Visitor for Interpreter {
    type Output = Result<Value, RuntimeError>;

    fn visit_statement(&mut self, stmt: &Stmt) -> Self::Output {
        match stmt {
            Stmt::Expr(expr) => {
                self.visit_expression(expr)?;
            }
            Stmt::Print(expr) => {
                let res = self.visit_expression(expr)?;
                println!("{res}")
            }
            Stmt::Var { name, initializer } => {
                let val = self.visit_expression(initializer)?;
                self.environment.define(name.lexeme.to_string(), val);
            }
            Stmt::Block(statements) => {
                self.environment.new_node();
                let res = self.execute_block(statements);
                self.environment.pop_node();
                res?;
            }
            Stmt::If {
                cond,
                then,
                else_stmt,
            } => {
                let cond = self.visit_expression(cond)?;
                if self.is_truthy(&cond) {
                    self.visit_statement(then)?;
                } else if else_stmt.is_some() {
                    self.visit_statement(&else_stmt.as_ref().unwrap())?;
                }
            }
            Stmt::While { cond, body } => {
                loop {
                    let cond = self.visit_expression(cond)?;
                    if !self.is_truthy(&cond) { break; }
                    self.visit_statement(body)?;
                }
            }
        };

        Ok(Value::Nil)
    }

    fn visit_expression(&mut self, expr: &crate::ast::Expr) -> Self::Output {
        match expr {
            Expr::Literal(val) => Ok(val.clone()), // TODO: Ужасный клон это говнище надо исправить
            Expr::Grouping(expr) => self.visit_expression(expr),
            Expr::Unary { op, right } => {
                let right = self.visit_expression(right)?;

                match (op.r#type, &right) {
                    (TokenType::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                    (TokenType::Minus, _) => Err(RuntimeError::number_op_err(&op)),
                    (TokenType::Bang, _) => Ok(Value::Bool(!self.is_truthy(&right))),
                    _ => unreachable!(),
                }
            }
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
            }
            Expr::Variable(name) => self.environment.get(&name),
            Expr::Assign { name, val } => {
                let val = self.visit_expression(val)?;
                self.environment.assign(&name, &val)?;
                Ok(val)
            }
            Expr::Logical { left, op, right } => {
                let left = self.visit_expression(left)?;

                if op.r#type == TokenType::Or {
                    if self.is_truthy(&left) { return Ok(left) }
                } else {
                    if !self.is_truthy(&left) { return Ok(left) }
                }

                Ok(self.visit_expression(right)?)
            }
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, stmt: Vec<Stmt>) -> Result<(), RuntimeError> {
        for stmt in stmt {
            self.visit_statement(&stmt)?;
        }

        Ok(())
    }

    fn execute_block(&mut self, statements: &[Stmt]) -> Result<Value, RuntimeError> {
        for stmt in statements {
            self.visit_statement(stmt)?;
        }

        Ok(Value::Nil)
    }

    fn is_truthy(&self, val: &Value) -> bool {
        match *val {
            Value::Nil => false,
            Value::Bool(b) => b,
            _ => true,
        }
    }

    fn plus(&self, l: Value, r: Value, op: Token) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
            (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
            _ => Err(RuntimeError {
                token: TokenInfo::from(&op),
                msg: "Operands must be either two strings or two numbers".to_string(),
            }),
        }
    }

    fn minus(&self, l: Value, r: Value, op: Token) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
            _ => Err(RuntimeError::two_number_op_err(&op)),
        }
    }

    fn slash(&self, l: Value, r: Value, op: Token) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
            _ => Err(RuntimeError::two_number_op_err(&op)),
        }
    }

    fn star(&self, l: Value, r: Value, op: Token) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
            _ => Err(RuntimeError::two_number_op_err(&op)),
        }
    }

    fn greater(&self, l: Value, r: Value, op: Token) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
            _ => Err(RuntimeError::number_op_err(&op)),
        }
    }

    fn greater_eq(&self, l: Value, r: Value, op: Token) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
            _ => Err(RuntimeError::number_op_err(&op)),
        }
    }

    fn less(&self, l: Value, r: Value, op: Token) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
            _ => Err(RuntimeError::number_op_err(&op)),
        }
    }

    fn less_eq(&self, l: Value, r: Value, op: Token) -> Result<Value, RuntimeError> {
        match (l, r) {
            (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
            _ => Err(RuntimeError::number_op_err(&op)),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct RuntimeError {
    pub token: TokenInfo,
    pub msg: String,
}

impl RuntimeError {
    fn number_op_err(token: &Token) -> Self {
        Self {
            token: token.into(),
            msg: "Operands must be numbers".into(),
        }
    }

    fn two_number_op_err(token: &Token) -> Self {
        Self {
            token: token.into(),
            msg: "Operands must be two numbers".into(),
        }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RuntimeError {}
