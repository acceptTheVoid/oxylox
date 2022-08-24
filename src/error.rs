use std::fmt::Display;

use crate::{ast::TokenAstInfo, value::Value};

#[derive(Debug)]
pub enum Error {
    ScannerError(ScanError),
    ParseError(ParseError),
    RuntimeError(RuntimeError),
    Return(Value),
}

impl From<RuntimeError> for Error {
    fn from(re: RuntimeError) -> Self {
        Self::RuntimeError(re)
    }
}

impl From<ParseError> for Error {
    fn from(pe: ParseError) -> Self {
        Self::ParseError(pe)
    }
}

impl From<ScanError> for Error {
    fn from(se: ScanError) -> Self {
        Self::ScannerError(se)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RuntimeError(re) => write!(f, "{re}"),
            Self::Return(v) => write!(f, "{v}"),
            Self::ParseError(pe) => write!(f, "{pe}"),
            Self::ScannerError(se) => write!(f, "{se}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct ParseError {
    pub token: TokenAstInfo,
    pub msg: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token: {}, msg: {}", self.token, self.msg)
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug)]
pub struct ScanError {
    pub msg: String,
    pub line: usize,
}

impl Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line: {}, msg: {}", self.line, self.msg)
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub token: TokenAstInfo,
    pub msg: String,
}

impl RuntimeError {
    pub fn number_op_err(token: &TokenAstInfo) -> Self {
        Self {
            token: token.clone(),
            msg: "Operands must be numbers".into(),
        }
    }

    pub fn two_number_op_err(token: &TokenAstInfo) -> Self {
        Self {
            token: token.clone(),
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
