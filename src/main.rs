pub mod lox;
pub mod tokentype;

use lox::Lox;
use std::{env::args, process::exit, io::Result};

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


