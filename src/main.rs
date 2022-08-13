// #![allow(unused)]

pub mod lox;
pub mod tokentype;
pub mod token;
pub mod scanner;
pub mod value;
pub mod ast;

use lox::Lox;
use token::Token;
use tokentype::TokenType;
use std::{env::args, io::Result, process::exit};
use ast::expr::*;
use ast::ast_printer::AstPrint;

fn main() -> Result<()> {
    // let args: Vec<String> = args().collect();
    // let mut lox = Lox::new();

    // match args.len() {
    //     1 => lox.run_prompt(),
    //     2 => lox.run_file(&args[1])?,
    //     _ => {
    //         println!("Usage: oxylox [script]");
    //         exit(64);
    //     }
    // }

    let expr = Ast::Binary(Binary {
        op: Token::new(TokenType::Star, "*".to_string(), value::Value::Nil, 0),
        left: Box::new(Ast::Unary(Unary { 
            op: Token::new(TokenType::Minus, "-".to_string(), value::Value::Nil, 0), 
            right: Box::new(Ast::Literal(Literal { val: value::Value::Number(123.) }))
        })),
        right: Box::new(Ast::Grouping(Grouping {
            expr: Box::new(Ast::Literal(Literal { val: value::Value::Number(45.67) }))
        }))
    });

    let mut printer = AstPrint;
    println!("{}", printer.print(&expr));

    Ok(())
}
