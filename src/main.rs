pub mod ast;
pub mod environment;
pub mod error;
pub mod function;
pub mod interpreter;
pub mod lox;
pub mod lox_callable;
pub mod parser;
pub mod resolver;
pub mod scanner;
pub mod test;
pub mod token;
pub mod tokentype;
pub mod value;

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
