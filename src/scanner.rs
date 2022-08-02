use crate::{token::Token, tokentype::TokenType};

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,   
    start: usize,
    current: usize,
    line: usize, 
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner { 
            source, 
            tokens: Vec::new(), 
            start: 0,
            current: 0,
            line: 1,
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
}
