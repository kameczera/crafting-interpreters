use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process;

use crate::lox::expr;
use crate::lox::interpreter;

use super::interpreter::Interpreter;
use super::token_type::TokenType;

// use super::ast_printer;
use super::scanner::*;
use super::token::*;
use super::parser::*;


pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false, had_runtime_error: false }
    }

    pub fn run_file(&mut self, path: &String) -> Result<(), Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        self.run(buffer);
        if self.had_error {
            process::exit(1);
        }
        if self.had_runtime_error {
            process::exit(1);
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
        println!("{:?}", tokens);
        let expression = match parser.parse() {
            Ok(expr) => expr,
            Err((token, message)) => return self.token_error(token, message),
        };
        let mut interpreter: Interpreter = Interpreter::new();
        match interpreter.interpret(expression) {
            Ok(_) => (),
            Err(err) => self.run_time_error(err),
        }

    }

    pub fn error(&mut self, line: u32, message: String) {
        self.report(line, String::new(), message);
    }

    pub fn run_time_error(&mut self, err: (Token, String)) {
        println!("{} \n[line {}]", err.1, err.0.line);
        self.had_runtime_error = true;
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