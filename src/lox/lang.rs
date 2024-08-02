use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

use crate::TokenType;

use super::ast_printer;
use super::scanner::*;
use super::token::*;
use super::parser::*;


pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: &String) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        self.run(buffer);
        if self.had_error {
            println!("Error");
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) {
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

    pub fn run(&mut self, source: Vec<u8>) {
        let mut scanner: Scanner = Scanner::new(source);
        let tokens: &Vec<Token> = scanner.scan_tokens();
        let mut parser: Parser = Parser::new(tokens);
        let expression = match parser.parse() {
            Ok(expr) => expr,
            Err((token, message)) => return self.token_error(token, message),
        };
        println!("{}", ast_printer::print(expression));
        for token in tokens {
            println!("{:?}", token);
        }

    }

    pub fn error(&mut self, line: u32, message: String) {
        self.report(line, String::new(), message);
    }

    pub fn report(&mut self, line: u32, col: String, message: String) {
        println!("[line {0}] Error {1}: {2}", line, col, message);
        self.had_error = true;
    }

    pub fn token_error(&mut self, token: Token, message: String){
        if let TokenType::Eof = token.token_type {
            self.report(token.line, String::from("at end"), message);
        } else {
            self.report(token.line, format!("at {}", String::from_utf8_lossy(&token.lexeme)), message);

        }
    }
}