use crate::error::{Error, ParseError, RuntimeError};
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::resolver::Resolver;
use crate::scanner::Scanner;
use crate::tokentype::TokenType;
use std::fs::File;
use std::io::{self, Read, Result, Write};
use std::process::exit;

#[derive(Default)]
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
        let tokens = match scanner.scan_tokens() {
            Ok(t) => t,
            Err(e) => {
                self.error(e);
                return;
            }
        };

        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(e) => {
                dbg!(&e);
                for pe in e {
                    self.error(pe.into());
                }
                return;
            }
        };

        let mut resolver = Resolver::new(&mut self.interpreter);
        if let Err(e) = resolver.resolve(&ast) {
            for e in e {
                self.error(e);
            }
            return;
        }

        match self.interpreter.interpret(ast) {
            Ok(_) => (),
            Err(e) => self.error(e),
        };
    }

    fn error(&mut self, err: Error) {
        match err {
            Error::ScannerError(se) => self.report(se.line, "", &se.msg),
            Error::ParseError(ParseError { token, msg, .. }) => {
                if token.kind == TokenType::Eof {
                    self.report(token.line, " at end", &msg);
                } else if let Some(name) = token.name {
                    self.report(token.line, &format!(" at '{name}'"), &msg);
                } else {
                    self.report(token.line, &format!(" at '{}'", token.kind), &msg);
                }
            }
            Error::RuntimeError(RuntimeError { token, msg, .. }) => {
                eprintln!("{msg}\n[line {}]", token.line);
                self.had_runtime_error = true;
            }
            Error::NativeCallError(msg) => {
                eprintln!("Error in native function: {msg}");
                self.had_runtime_error = true;
            }
            Error::Return(_) => unreachable!(
                "Well, that shouldn't happen... ICE Code: 0x0: Got return statement as error"
            ),
        }
    }

    fn report(&mut self, line: usize, r#where: &str, msg: &str) {
        eprintln!("[line {line}] Error{where}: {msg}");
        self.had_error = true;
    }
}
