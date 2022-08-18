use crate::{
    ast::{*, stmt::Stmt},
    token::Token,
    tokentype::TokenType::{self, *},
    value::Value,
};

use std::string::String;

pub struct Parser {
    tokens: Vec<Token>,
    cur: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cur: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Stmt, ParseError> {
        Ok(Stmt::Expr(self.expression()?))
    }

    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.cur]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.cur - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.cur += 1
        }
        self.previous()
    }

    fn check(&self, token: TokenType) -> bool {
        if self.is_at_end() {
            return false
        }
        self.peek().r#type == token
    }

    fn match_any(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(*t) {
                self.advance();
                return true
            }
        }
        false
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        Ok(self.equality()?)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparsion()?;

        while self.match_any(&[TokenType::BangEq, TokenType::EqEq]) {
            let op = self.previous().clone();
            let right = self.comparsion()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparsion(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_any(&[Greater, GreaterEq, Less, LessEq]) {
            let op = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_any(&[Minus, Plus]) {
            let op = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_any(&[Slash, Star]) {
            let op = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_any(&[Bang, Minus]) {
            let op = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary { op, right: Box::new(right) })
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_any(&[False]) {
            return Ok(Expr::Literal(Value::Bool(false)))
        }

        if self.match_any(&[True]) {
            return Ok(Expr::Literal(Value::Bool(true)))
        }

        if self.match_any(&[Number, String, Nil]) {
            return Ok(Expr::Literal(self.previous().literal.clone()))
        }

        if self.match_any(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)))
        }

        Err(ParseError { token: self.peek().clone(), msg: "Expect expression".to_string(), })
    }

    fn consume(&mut self, token: TokenType, msg: &str) -> Result<&Token, ParseError> {
        if self.check(token) { return Ok(self.advance()) }

        Err(ParseError { token: self.peek().clone(), msg: msg.to_string() })
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().r#type == Semicolon { return }
        
            match self.peek().r#type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub msg: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token: {}, msg: {}", self.token, self.msg)
    }
}

impl std::error::Error for ParseError {}
