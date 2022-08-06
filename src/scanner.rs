use crate::{token::Token, tokentype::{TokenType, KEYWORDS}, lox::Lox};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,   
    start: usize,
    current: usize,
    line: usize,
    lox_instance: &'a mut Lox,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str, lox_instance: &'a mut Lox) -> Self {
        Scanner { 
            source, 
            tokens: Vec::new(), 
            start: 0,
            current: 0,
            line: 1,
            lox_instance,
        }
    }

    pub fn scan_tokens(&'a mut self) -> &'a Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(
            Token::new(
                TokenType::Eof,
                "",
                Box::new(""), 
                self.line)
            );
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
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
            '!' => {
                let r#match = self.r#match('=');
                self.add_token(if r#match { BangEq } else { Bang })
            },
            '=' => {
                let r#match = self.r#match('=');
                self.add_token(if r#match { EqEq } else { Eq })
            },
            '<' => {
                let r#match = self.r#match('=');
                self.add_token(if r#match { LessEq } else { Less })
            },
            '>' => {
                let r#match = self.r#match('=');
                self.add_token(if r#match { GreaterEq } else { Greater })
            },
            '/' => {
                if self.r#match('/') {
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                } else {
                    self.add_token(Slash)
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            d if d.is_ascii_digit() => self.number(),
            i if is_alpha_numeric(i) => self.identifier(),
            _ => {
                // TODO: Улучшить обработку ошибок символов
                // Сделать из ошибки на каждый символ одну ошибку на группы символов
                self.lox_instance.error(self.line, "Unexpected character")
            }
        }
    }

    fn advance(&mut self) -> char {
        // TODO: Очень некрасиво написано исправь епта
        // В целом надо провести окисление этого кода
        // Ведь он может быть написан намного красивее
        let next = self.source.chars().nth(self.current).unwrap(); 
        self.current += 1;
        next
    }

    fn add_token(&mut self, token: TokenType) {
        self.add_token_literal(token, Box::new(""))
    }

    fn add_token_literal(
        &mut self, 
        token: TokenType, 
        literal: Box<dyn std::fmt::Display>
    ) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token,
            text,
            literal,
            self.line,
        ));
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false }
        if self.source.chars().nth(self.current).unwrap() != expected { return false }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        // TODO: Тоже не очень красиво 
        // Надо переписать на Option
        if self.is_at_end() { return '\0' }
        self.source.chars().nth(self.current).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1 }
            self.advance();
        }

        if self.is_at_end() {
            self.lox_instance.error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        let val = &self.source[self.start + 1..self.current - 1];
        self.add_token_literal(TokenType::String, Box::new(val.to_string()));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() { self.advance(); }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() { self.advance(); }
        }

        let num = self.source[self.start..self.current].parse::<f64>().unwrap();

        self.add_token_literal(
            TokenType::Number, 
            Box::new(num),
        );
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0' }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) { self.advance(); }

        let text = &self.source[self.start..self.current];
        let r#type = match KEYWORDS.get(text) {
            None => TokenType::Identifier,
            Some(r#type) => *r#type,
        };

        self.add_token(r#type);
    }
}

fn is_alpha(c: char) -> bool {
    'a' <= c && c <= 'z' ||
    'A' <= c && c <= 'Z' ||
    c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || c.is_ascii_digit()
}
