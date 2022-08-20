use std::{collections::HashMap, rc::Rc};

use crate::{interpreter::RuntimeError, token::Token, value::Value};

pub struct Environment {
    enclosing: Option<Rc<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn inner(env: Environment) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(Rc::new(env)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Value, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).cloned().unwrap())
        }

        if self.enclosing.is_some() { return self.enclosing.as_ref().unwrap().get(name) }

        Err(RuntimeError {
            msg: format!("Undefined variable '{}'", name.lexeme),
            token: name,
        })

        // todo!()
        // self.values
        //     .get(&name.lexeme)
        //     .cloned()
        //     .ok_or(RuntimeError {
        //         msg: format!("Undefined variable '{}'", name.lexeme),
        //         token: name,
        //     })
    }

    pub fn assign(&mut self, name: Token, value: Value) -> Result<(), RuntimeError> {
        if let None = self.values.get_mut(&name.lexeme).map(|v| *v = value) {
            Err(RuntimeError {
                msg: format!("Undefined variable '{}'", name.lexeme),
                token: name,
            })
        } else {
            Ok(())
        }
    }
}
