use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::TokenAstInfo,
    error::{Error, RuntimeError},
    value::Value,
};

#[derive(Debug, Clone, Default)]
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

    pub fn define(&mut self, name: &TokenAstInfo, value: Value) -> Result<(), Error> {
        if self.enclosing.is_some() && self.values.contains_key(name.name.as_ref().unwrap()) {
            Err(RuntimeError {
                token: name.clone(),
                msg: "You cannot redefine local variables".into(),
            }
            .into())
        } else {
            self.values
                .insert(name.name.as_ref().unwrap().to_string(), value);
            Ok(())
        }
    }

    pub fn get(&self, name: &TokenAstInfo) -> Result<Value, Error> {
        let key = name.name.as_ref().unwrap();

        if let Some(value) = self.values.get(key) {
            Ok(value.clone())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow().get(name)  
        } else {
            Err(RuntimeError {
                msg: format!("Undefined variable '{key}'"),
                token: name.clone(),
            }
            .into())
        }
    }

    pub fn assign(&mut self, name: &TokenAstInfo, value: &Value) -> Result<(), Error> {
        let key = name.name.as_ref().unwrap();
        if self.values.contains_key(key) {
            self.values.insert(key.clone(), value.clone());
            Ok(())
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError {
                msg: format!("Undefined variable '{}'", name),
                token: name.clone(),
            }
            .into())
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
