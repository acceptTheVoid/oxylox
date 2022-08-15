// #![allow(unused)]

pub mod ast;
pub mod lox;
pub mod parser;
pub mod scanner;
pub mod test;
pub mod token;
pub mod tokentype;
pub mod value;
pub mod interpreter;

use lox::Lox;
use std::{env::args, io::Result, process::exit};

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    let mut lox = Lox::new();

    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1])?,
        _ => {
            println!("Usage: oxylox [script]");
            exit(64);
        }
    }

    Ok(())
}
