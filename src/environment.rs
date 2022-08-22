use std::collections::HashMap;

use crate::{
    interpreter::{RuntimeError, TokenInfo},
    token::Token,
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Environment {
    stack: Vec<EnvNode>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            stack: vec![EnvNode::new()],
        }
    }

    pub fn new_node(&mut self) {
        self.stack.push(EnvNode::new())
    }

    pub fn pop_node(&mut self) {
        self.stack.pop();
    }

    // FIXME: Если переменная переприсваивается в локальной области видимости
    // Это должна быть ошибка
    pub fn define(&mut self, name: String, value: Value) {
        self.stack.last_mut().unwrap().define(name, value)
    }

    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        for env in self.stack.iter().rev() {
            match env.get(&name.lexeme) {
                Some(v) => return Ok(v.clone()),
                None => (),
            }
        }

        Err(RuntimeError {
            msg: format!("Undefined variable '{}'", name.lexeme),
            token: TokenInfo::from(name),
        })
    }

    pub fn assign(&mut self, name: &Token, value: &Value) -> Result<(), RuntimeError> {
        for env in self.stack.iter_mut().rev() {
            match env.assign(&name.lexeme, value) {
                Some(_) => return Ok(()),
                None => (),
            }
        }

        Err(RuntimeError {
            msg: format!("Undefined variable '{}'", name.lexeme),
            token: TokenInfo::from(name),
        })
    }
}

#[derive(Debug, Clone)]
struct EnvNode {
    vars: HashMap<String, Value>,
}

impl EnvNode {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    fn define(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }

    fn get(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)

        // if self.values.borrow().contains_key(&name.lexeme) {
        //     Ok(self.values.borrow_mut().get(&name.lexeme).cloned().unwrap())
        // } else if self.parent.is_some() {
        //     self.parent.as_ref().unwrap().get(name)
        // } else {
        //     Err(RuntimeError {
        //         msg: format!("Undefined variable '{}'", name.lexeme),
        //         token: name,
        //     })
        // }
    }

    fn assign(&mut self, name: &str, value: &Value) -> Option<()> {
        if self.vars.contains_key(name) {
            self.vars.get_mut(name).map(|v| *v = value.clone());
            Some(())
        } else {
            None
        }

        // if let Some(v) = self.values.borrow_mut().get_mut(&name.lexeme) {
        //     *v = value;
        //     Ok(())
        // } else if self.parent.is_some() {
        //     self.parent.as_ref().unwrap().assign(name, value)
        // } else {
        //     Err(RuntimeError {
        //         msg: format!("Undefined variable '{}'", name.lexeme),
        //         token: name,
        //     })
        // }
    }
}
