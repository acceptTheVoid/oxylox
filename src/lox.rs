use std::fs::File;
use std::process::exit;
use std::io::{BufReader, Result, BufRead, self, Write};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&self, path: &str) -> Result<()> {
        let file = File::options().read(true).open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            self.run(&line.expect(&format!("Failed to read line from {path}")))
        }

        if self.had_error { 
            exit(65) 
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
            if line.trim().is_empty() { break; }
            self.run(line.trim());
            self.had_error = false;
        }
    }

    fn run(&self, line: &str) {
        let scanner = Scanner::new(line);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{token}")
        }
    }

    fn error(&self, line: usize, msg: &str) {
        self.report(line, "", msg);
    }

    fn report(&mut self, line: usize, r#where: &str, msg: &str) {
        println!(
            "[line {line}] Error{where}: {msg}"
        );
        self.had_error = true;
    }
}

