use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{stmt::Stmt, TokenAstInfo},
    environment::Environment,
    error::Error,
    interpreter::Interpreter,
    lox_callable::Callable,
    value::Value,
};

#[derive(Clone)]
pub enum Function {
    Native {
        arity: usize,
        body: Box<fn(&[Value]) -> Value>,
    },
    LoxFun {
        name: TokenAstInfo,
        params: Vec<TokenAstInfo>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },
}

impl Callable<Result<Value, Error>> for Function {
    fn call(&self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, Error> {
        match self {
            Self::Native { body, .. } => Ok(body(args)),
            Self::LoxFun {
                params,
                body,
                closure,
                ..
            } => {
                let env = Rc::new(RefCell::new(Environment::from(closure)));
                for (param, arg) in params.iter().zip(args.iter()) {
                    env.borrow_mut().define(param, arg.clone())?;
                }

                match interpreter.execute_block(body, env) {
                    Ok(v) => Ok(v),
                    Err(e) => match e {
                        Error::Return(v) => Ok(v),
                        re => Err(re),
                    },
                }
            }
        }
    }

    fn arity(&self) -> usize {
        match self {
            Self::Native { arity, .. } => *arity,
            Self::LoxFun { params, .. } => params.len(),
        }
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native fun>")
    }
}

impl PartialEq for Function {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
