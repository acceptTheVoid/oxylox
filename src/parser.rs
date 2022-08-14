use crate::{
    ast::*,
    lox::Lox,
    token::Token,
    tokentype::TokenType::{self, *},
    value::Value,
};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    cur: usize,
    lox_instance: &'a Lox,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, lox_instance: &'a Lox) -> Self {
        Self {
            tokens,
            cur: 0,
            lox_instance,
        }
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
            return false;
        }
        self.peek().r#type == token
    }

    fn match_any(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(*t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn expression(&mut self) -> Ast {
        self.equality()
    }

    fn equality(&mut self) -> Ast {
        let mut expr = self.comparsion();

        while self.match_any(&[TokenType::BangEq, TokenType::EqEq]) {
            let op = self.previous().clone();
            let right = self.comparsion();
            expr = Ast::Binary(Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            });
        }

        expr
    }

    fn comparsion(&mut self) -> Ast {
        let mut expr = self.term();

        while self.match_any(&[Greater, GreaterEq, Less, LessEq]) {
            let op = self.previous().clone();
            let right = self.term();
            expr = Ast::Binary(Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            });
        }

        expr
    }

    fn term(&mut self) -> Ast {
        let mut expr = self.factor();

        while self.match_any(&[Minus, Plus]) {
            let op = self.previous().clone();
            let right = self.factor();
            expr = Ast::Binary(Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            });
        }

        expr
    }

    fn factor(&mut self) -> Ast {
        let mut expr = self.unary();

        while self.match_any(&[Slash, Star]) {
            let op = self.previous().clone();
            let right = self.unary();
            expr = Ast::Binary(Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            });
        }

        expr
    }

    fn unary(&mut self) -> Ast {
        if self.match_any(&[Bang, Minus]) {
            let op = self.previous().clone();
            let right = self.unary();
            return Ast::Unary(Unary {
                op,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Ast {
        if self.match_any(&[False]) {
            return Ast::Literal(Literal {
                val: Value::Bool(false),
            });
        }
        if self.match_any(&[True]) {
            return Ast::Literal(Literal {
                val: Value::Bool(true),
            });
        }

        if self.match_any(&[Number, String, Nil]) {
            return Ast::Literal(Literal {
                val: self.previous().literal.clone(),
            });
        }

        if self.match_any(&[LeftParen]) {
            let expr = self.expression();
            self.consume(RightParen, "Expect ')' after expression");
            return Ast::Grouping(Grouping {
                expr: Box::new(expr),
            });
        }

        unimplemented!()
    }

    fn consume(&mut self, token: TokenType, msg: &str) {
        todo!()
    }
}
