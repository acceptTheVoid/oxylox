// TODO: Тебе над перенести все ошибки сюда
// TODO: Удачи...

use crate::{interpreter::RuntimeError, value::Value};

#[derive(Debug)]
pub enum Error {
    RuntimeError(RuntimeError),
    Return(Value),
}

impl From<RuntimeError> for Error {
    fn from(re: RuntimeError) -> Self {
        Self::RuntimeError(re)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeError(re) => write!(f, "{re}"),
            Self::Return(v) => write!(f, "{v}"),
        }
    }
}

impl std::error::Error for Error {}
