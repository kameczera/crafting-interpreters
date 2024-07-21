use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

use super::scanner::*;
use super::token::*;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&self, path: &String) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        self.run(buffer);
        if self.had_error {
            println!("Error");
        }
        Ok(())
    }

    pub fn run_prompt(&self) {
        println!("Starting");
        loop {
            print!("> ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if input.is_empty() {
                break;
            };
            let code = input.into_bytes();
            self.run(code);
        }
    }

    pub fn run(&self, source: Vec<u8>) {
        let mut scanner: Scanner = Scanner::new(source);
        let tokens: &Vec<Token> = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(&mut self, line: u32, message: &String) {
        self.report(line, &String::new(), message);
    }

    fn report(&mut self, line: u32, col: &String, message: &String) {
        println!("[line {0}] Error {1}: {2}", line, col, message);
        self.had_error = true;
    }
}