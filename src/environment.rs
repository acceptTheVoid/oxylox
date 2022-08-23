use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::Error,
    interpreter::{RuntimeError, TokenInfo},
    token::Token,
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn from(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Some(Rc::clone(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &Token, value: Value) -> Result<(), Error> {
        if self.enclosing.is_some() && self.values.contains_key(&name.lexeme) {
            Err(RuntimeError {
                token: name.into(),
                msg: format!("You cannot redefine local variables"),
            }
            .into())
        } else {
            self.values.insert(name.lexeme.to_string(), value);
            Ok(())
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, Error> {
        let key = &name.lexeme;

        if let Some(value) = self.values.get(key) {
            Ok(value.clone())
        } else {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow().get(name)
            } else {
                Err(RuntimeError {
                    msg: format!("Undefined variable '{key}'"),
                    token: TokenInfo::from(name),
                }
                .into())
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Value) -> Result<(), Error> {
        let key = &name.lexeme;
        if self.values.contains_key(key) {
            self.values.insert(key.clone(), value.clone());
            Ok(())
        } else {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow_mut().assign(name, value)
            } else {
                Err(RuntimeError {
                    msg: format!("Undefined variable '{}'", name.lexeme),
                    token: TokenInfo::from(name),
                }
                .into())
            }
        }
    }
}

impl From<HashMap<String, Value>> for Environment {
    fn from(globals: HashMap<String, Value>) -> Self {
        Self {
            enclosing: None,
            values: globals,
        }
    }
}
