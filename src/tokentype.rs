use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Токены состоящие из одного символа
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Percent,
    Slash,
    Star,

    // Токены из одного или двух символов
    Bang,
    BangEq,
    Eq,
    EqEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,

    // Литералы
    Identifier,
    String,
    Number,

    // Ключевые слова
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Конец файла
    Eof,
}

use TokenType::*;

lazy_static::lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("and", And),
        ("class", Class),
        ("else", Else),
        ("false", False),
        ("for", For),
        ("fun", Fun),
        ("if", If),
        ("nil", Nil),
        ("or", Or),
        ("print", Print),
        ("return", Return),
        ("super", Super),
        ("this", This),
        ("true", True),
        ("var", Var),
        ("while", While),
    ]);
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            // Токены состоящие из одного символа
            LeftParen => "(",
            RightParen => ")",
            LeftBrace => "{",
            RightBrace => "+",
            Comma => ",",
            Dot => ".",
            Minus => "-",
            Plus => "+",
            Semicolon => ";",
            Percent => "%",
            Slash => "/",
            Star => "*",

            // Токены из одного или двух символов
            Bang => "!",
            BangEq => "!=",
            Eq => "=",
            EqEq => "==",
            Greater => ">",
            GreaterEq => ">=",
            Less => "<",
            LessEq => "<=",

            // Литералы
            Identifier => "Identifier",
            String => "String",
            Number => "Number",

            // Ключевые слова
            And => "and",
            Class => "class",
            Else => "else",
            False => "false",
            Fun => "fun",
            For => "for",
            If => "if",
            Nil => "nil",
            Or => "or",
            Print => "print",
            Return => "return",
            Super => "super",
            This => "this",
            True => "true",
            Var => "var",
            While => "while",

            // Конец файла
            Eof => "eof",
        };

        write!(f, "{to_write}")
    }
}
