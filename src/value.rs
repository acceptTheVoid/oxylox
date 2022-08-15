#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Number(f64),
    String(String),
    Bool(bool),
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
        };
        write!(f, "{to_write}")
    }
}
