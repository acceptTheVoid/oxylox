use crate::interpreter::{Interpreter, RuntimeError};
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::token::Token;
use crate::tokentype::TokenType;
use std::fs::File;
use std::io::{self, Read, Result, Write};
use std::process::exit;

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_error: false,
            had_runtime_error: false,
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(&mut self, path: &str) -> Result<()> {
        let mut file = File::options().read(true).open(path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        self.run(&buf);

        if self.had_error {
            exit(65)
        }

        if self.had_runtime_error {
            exit(70);
        }

        Ok(())
    }

    pub fn run_prompt(&mut self) {
        println!("Enter blank line to exit");
        let cin = io::stdin();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            cin.read_line(&mut line).unwrap();
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            self.run(line);
            self.had_error = false;
        }
    }

    fn run(&mut self, line: &str) {
        let scanner = Scanner::new(line);

        match scanner.scan_tokens() {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(stmt) => {
                        match self.interpreter.interpret(stmt) {
                            Ok(_) => (),
                            Err(e) => self.runtime_error(e),
                        }
                        // let mut printer = crate::ast::ast_printer::AstPrint;
                        // println!("{}", printer.print(&stmt));
                    }
                    Err(e) => {
                        for e in e {
                            self.parse_error(&e.token, &e.msg)
                        }
                    },
                };
            }
            Err((line, msg)) => self.scan_error(line, &msg),
        }
    }

    fn scan_error(&mut self, line: usize, msg: &str) {
        self.report(line, "", msg);
    }

    fn parse_error(&mut self, token: &Token, msg: &str) {
        if token.r#type == TokenType::Eof {
            self.report(token.line, " at end", msg);
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), msg);
        }
    }

    fn runtime_error(&mut self, error: RuntimeError) {
        eprintln!("{}\n[line {}]", error.msg, error.token.line);
        self.had_runtime_error = true;
    }

    fn report(&mut self, line: usize, r#where: &str, msg: &str) {
        eprintln!("[line {line}] Error{where}: {msg}");
        self.had_error = true;
    }
}
