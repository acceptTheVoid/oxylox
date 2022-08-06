#![allow(unused)]

pub mod lox;
pub mod tokentype;
pub mod token;
pub mod scanner;

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
