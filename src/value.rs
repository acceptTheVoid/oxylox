use crate::function::Function;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Number(f64),
    String(String),
    Bool(bool),
    Fun(Function),
}

impl Eq for Value {}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;

        let to_write = match self {
            Nil => "Nil".to_string(),
            Number(n) => n.to_string(),
            String(s) => s.to_string(),
            Bool(b) => b.to_string(),
            Fun(fun) => match fun {
                Function::Native { .. } => "<native fun>".into(),
                Function::LoxFun { name, .. } => {
                    format!("<lox fun '{}'>", name.get_name())
                }
            },
        };
        write!(f, "{to_write}")
    }
}
