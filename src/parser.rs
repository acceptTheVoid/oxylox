use crate::{
    ast::{stmt::Stmt, *},
    error::ParseError,
    token::Token,
    tokentype::TokenType::{self, *},
    value::Value,
};

type StmtRes = Result<Stmt, ParseError>;
type ExprRes = Result<Expr, ParseError>;

pub struct Parser {
    tokens: Vec<Token>,
    cur: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cur: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<ParseError>> {
        let mut statements = vec![];
        let mut errors = vec![];

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    errors.push(e);
                    self.synchronize()
                }
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
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

    fn declaration(&mut self) -> StmtRes {
        if self.match_any(&[Var]) {
            self.var_declaration()
        } else if self.match_any(&[Fun]) {
            self.function_declaration("function")
        } else {
            self.statement()
        }
    }

    fn function_declaration(&mut self, kind: &str) -> StmtRes {
        let name = self
            .consume(Identifier, &format!("Expect {kind} name"))?
            .into();
        self.consume(LeftParen, &format!("Expect '(' after {kind} name"))?;

        let mut params = vec![];
        if !self.check(RightParen) {
            loop {
                if params.len() >= 255 {
                    return Err(ParseError {
                        token: self.peek().into(),
                        msg: "Can't have more than 255 parametres".into(),
                    });
                }

                params.push(self.consume(Identifier, "Expect parameter name")?.into());
                if !self.match_any(&[Comma]) {
                    break;
                }
            }
        }
        self.consume(RightParen, "Expect ')' after parameters")?;

        self.consume(LeftBrace, &format!("Expect '{{' before {kind} body"))?;
        let body = self.block()?;
        Ok(Stmt::Function { name, params, body })
    }

    fn var_declaration(&mut self) -> StmtRes {
        let name = self.consume(Identifier, "Expect variable name")?.into();
        let mut initializer = Expr::Literal(Value::Nil);
        if self.match_any(&[Eq]) {
            initializer = self.expression()?;
        }

        self.consume(Semicolon, "Expect ';' after variable declaration")?;
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> StmtRes {
        if self.match_any(&[For]) {
            return self.for_statement();
        }

        if self.match_any(&[If]) {
            return self.if_statement();
        }

        if self.match_any(&[Print]) {
            return self.print_statement();
        }

        if self.match_any(&[Return]) {
            return self.return_statement();
        }

        if self.match_any(&[While]) {
            return self.while_statement();
        }

        if self.match_any(&[LeftBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }

        self.expression_statement()
    }

    fn return_statement(&mut self) -> StmtRes {
        let keyword = self.previous().into();
        let mut val = Expr::Literal(Value::Nil);
        if !self.check(Semicolon) {
            val = self.expression()?;
        }

        self.consume(Semicolon, "Expect ';' after return value")?;
        Ok(Stmt::Return { keyword, val })
    }

    fn for_statement(&mut self) -> StmtRes {
        self.consume(LeftParen, "Expect '(' after 'for'")?;

        let init;
        if self.match_any(&[Semicolon]) {
            init = None;
        } else if self.match_any(&[Var]) {
            init = Some(self.var_declaration()?);
        } else {
            init = Some(self.expression_statement()?);
        }

        let mut cond = None;
        if !self.check(Semicolon) {
            cond = Some(self.expression()?);
        }
        self.consume(Semicolon, "Expect ';' after loop condition")?;

        let mut inc = None;
        if !self.check(Semicolon) {
            inc = Some(self.expression()?);
        }
        self.consume(RightParen, "Expect ')' after for clauses")?;

        let mut body = self.statement()?;

        if inc.is_some() {
            body = Stmt::Block(vec![body, Stmt::Expr(inc.unwrap())]);
        }

        if cond.is_none() {
            cond = Some(Expr::Literal(Value::Bool(true)));
        }
        body = Stmt::While {
            cond: cond.unwrap(),
            body: Box::new(body),
        };

        if init.is_some() {
            body = Stmt::Block(vec![init.unwrap(), body]);
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> StmtRes {
        self.consume(LeftParen, "Expect '(' after 'if'")?;
        let cond = self.expression()?;
        self.consume(RightParen, "Expected ')' after if condition")?;

        let then = Box::new(self.statement()?);
        let mut else_stmt = None;
        if self.match_any(&[Else]) {
            else_stmt = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If {
            cond,
            then,
            else_stmt,
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = vec![];

        while !self.check(RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(RightBrace, "Expect '}' after block")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> StmtRes {
        let mut exprs = vec![];
        loop {
            exprs.push(self.expression()?);
            if self.check(Semicolon) {
                break;
            }

            self.consume(Comma, "Expect ',' between expressions")?;
        }
        // let val = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value")?;
        Ok(Stmt::Print(exprs))
    }

    fn while_statement(&mut self) -> StmtRes {
        self.consume(LeftParen, "Expect '(' after while")?;
        let cond = self.expression()?;
        self.consume(RightParen, "Expect ')' after while condition")?;
        let body = Box::new(self.statement()?);

        Ok(Stmt::While { cond, body })
    }

    fn expression_statement(&mut self) -> StmtRes {
        let expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value")?;
        Ok(Stmt::Expr(expr))
    }

    fn expression(&mut self) -> ExprRes {
        self.assignment()
    }

    fn assignment(&mut self) -> ExprRes {
        let expr = self.or()?;

        if self.match_any(&[Eq]) {
            let equals = self.previous().into();
            let val = self.assignment()?;

            return if let Expr::Variable(name) = expr {
                Ok(Expr::Assign {
                    name,
                    val: Box::new(val),
                })
            } else {
                Err(ParseError {
                    token: equals,
                    msg: "Invalid assignment target".into(),
                })
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> ExprRes {
        let mut expr = self.and()?;

        while self.match_any(&[Or]) {
            let op = self.previous().into();
            let right = Box::new(self.equality()?);
            expr = Expr::Logical {
                left: Box::new(expr),
                op,
                right,
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> ExprRes {
        let mut expr = self.equality()?;

        while self.match_any(&[And]) {
            let op = self.previous().into();
            let right = Box::new(self.equality()?);
            expr = Expr::Logical {
                left: Box::new(expr),
                op,
                right,
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> ExprRes {
        let mut expr = self.comparsion()?;

        while self.match_any(&[TokenType::BangEq, TokenType::EqEq]) {
            let op = self.previous().into();
            let right = self.comparsion()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparsion(&mut self) -> ExprRes {
        let mut expr = self.term()?;

        while self.match_any(&[Greater, GreaterEq, Less, LessEq]) {
            let op = self.previous().into();
            let right = self.term()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> ExprRes {
        let mut expr = self.factor()?;

        while self.match_any(&[Minus, Plus]) {
            let op = self.previous().into();
            let right = self.factor()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ExprRes {
        let mut expr = self.unary()?;

        while self.match_any(&[Slash, Star, Percent]) {
            let op = self.previous().into();
            let right = self.unary()?;
            expr = Expr::Binary {
                op,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ExprRes {
        if self.match_any(&[Bang, Minus]) {
            let op = self.previous().into();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                op,
                right: Box::new(right),
            });
        }

        self.call()
    }

    fn call(&mut self) -> ExprRes {
        let mut expr = self.primary()?;

        loop {
            if self.match_any(&[LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> ExprRes {
        let mut args = vec![];
        if !self.check(RightParen) {
            loop {
                if args.len() >= 255 {
                    return Err(ParseError {
                        token: self.peek().into(),
                        msg: "Can't have more than 255 arguments".into(),
                    });
                }

                args.push(self.expression()?);
                if !self.match_any(&[Comma]) {
                    break;
                }
            }
        }

        let paren = self
            .consume(RightParen, "Expect ')' after arguments")?
            .into();
        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            args,
        })
    }

    fn primary(&mut self) -> ExprRes {
        if self.match_any(&[False]) {
            return Ok(Expr::Literal(Value::Bool(false)));
        }

        if self.match_any(&[True]) {
            return Ok(Expr::Literal(Value::Bool(true)));
        }

        if self.match_any(&[Number, String, Nil]) {
            return Ok(Expr::Literal(self.previous().literal.clone()));
        }

        if self.match_any(&[Identifier]) {
            return Ok(Expr::Variable(self.previous().into()));
        }

        if self.match_any(&[LeftParen]) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(ParseError {
            token: self.peek().into(),
            msg: "Expect expression".to_string(),
        })
    }

    fn consume(&mut self, token: TokenType, msg: &str) -> Result<&Token, ParseError> {
        if self.check(token) {
            return Ok(self.advance());
        }

        Err(ParseError {
            token: self.peek().into(),
            msg: msg.to_string(),
        })
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().r#type == Semicolon {
                return;
            }

            match self.peek().r#type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => (),
            }

            self.advance();
        }
    }
}
