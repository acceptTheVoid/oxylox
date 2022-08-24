use crate::{
    error::{Error, ScanError},
    token::Token,
    tokentype::{TokenType, KEYWORDS},
    value::Value,
};

pub type ScanResult = Result<Vec<Token>, Error>;

// TODO: Лучше исправить работу со строками на работу с итератором `Chars`
pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    bytes_start: usize,
    bytes_cur: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            bytes_start: 0,
            bytes_cur: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> ScanResult {
        while !self.is_at_end() {
            self.start = self.current;
            self.bytes_start = self.bytes_cur;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Value::Nil,
            self.line,
        ));

        Ok(self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.bytes_cur >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        use TokenType::*;

        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '%' => self.add_token(Percent),
            '!' => {
                let r#match = self.r#match('=');
                self.add_token(if r#match { BangEq } else { Bang })
            }
            '=' => {
                let r#match = self.r#match('=');
                self.add_token(if r#match { EqEq } else { Eq })
            }
            '<' => {
                let r#match = self.r#match('=');
                self.add_token(if r#match { LessEq } else { Less })
            }
            '>' => {
                let r#match = self.r#match('=');
                self.add_token(if r#match { GreaterEq } else { Greater })
            }
            '/' => {
                if self.r#match('/') {
                    while !check_peek(self.peek(), '\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.r#match('*') {
                    let mut depth = 1;
                    while !self.is_at_end() {
                        let peek = self.peek();
                        let next = self.peek_next();
                        if check_peek(peek, '*') && check_peek(next, '/') {
                            if depth == 1 {
                                self.advance();
                                self.advance();
                                break;
                            } else {
                                depth -= 1;
                            }
                        } else if check_peek(peek, '\n') {
                            self.line += 1;
                        } else if check_peek(peek, '/') && check_peek(next, '*') {
                            depth += 1;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(Slash)
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string()?,
            d if d.is_ascii_digit() => self.number(),
            i if is_alpha(i) => self.identifier(),
            _ => {
                return Err(ScanError {
                    line: self.line,
                    msg: "Unexpected character".into(),
                }
                .into())
            }
        }

        Ok(())
    }

    fn advance(&mut self) -> char {
        let next = self.source.chars().nth(self.current).unwrap();
        self.bytes_cur += next.len_utf8();
        self.current += 1;
        next
    }

    fn add_token(&mut self, token: TokenType) {
        self.add_token_literal(token, Value::Nil)
    }

    fn add_token_literal(&mut self, token: TokenType, literal: Value) {
        let text = &self.source[self.bytes_start..self.bytes_cur];
        self.tokens
            .push(Token::new(token, text.to_string(), literal, self.line));
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.bytes_cur += expected.len_utf8();
        self.current += 1;
        true
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn string(&mut self) -> Result<(), Error> {
        let mut string = String::new();
        while let Some(ch) = self.peek() {
            string.push(match ch {
                '"' => break,
                '\n' => {
                    return Err(ScanError {
                        line: self.line,
                        msg: "Unterminated string".into(),
                    }.into())
                }
                '\\' => {
                    self.advance();
                    if !self.is_at_end() {
                        match self.advance() {
                            '0' => '\0',
                            't' => '\t',
                            'n' => '\n',
                            'r' => '\r',     
                            '\\' => '\\',
                            '"' => '"',
                            ch => return Err(ScanError {
                                line: self.line,
                                msg: format!("Unsupported symbol '{ch}'"),
                            }.into()),
                        }
                    } else {
                        break;
                    }
                },
                _ => {
                    self.advance()
                }
            })
        }

        if self.is_at_end() {
            return Err(ScanError {
                line: self.line,
                msg: "Unterminated string".into(),
            }
            .into());
        }

        self.advance();

        self.add_token_literal(TokenType::String, Value::String(string));

        Ok(())
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if check_peek(self.peek(), '.') && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let num = self.source[self.bytes_start..self.bytes_cur]
            .parse::<f64>()
            .unwrap();

        self.add_token_literal(TokenType::Number, Value::Number(num));
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn identifier(&mut self) {
        while check_peek_with(self.peek(), is_alpha_numeric) {
            self.advance();
        }

        let text = &self.source[self.bytes_start..self.bytes_cur];
        let r#type = match KEYWORDS.get(text) {
            None => TokenType::Identifier,
            Some(r#type) => *r#type,
        };

        self.add_token(r#type);
    }
}

fn is_alpha(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || c.is_ascii_digit()
}

fn is_digit(ch: Option<char>) -> bool {
    ch.filter(|ch| ch.is_ascii_digit()).is_some()
}

fn check_peek(ch: Option<char>, to_check: char) -> bool {
    ch.filter(|ch| *ch == to_check).is_some()
}

fn check_peek_with(ch: Option<char>, check: impl FnOnce(char) -> bool) -> bool {
    ch.filter(|ch| check(*ch)).is_some()
}
