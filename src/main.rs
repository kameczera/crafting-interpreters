use std::env;
use std::process;

use crate::lox::interpreter::*;

mod lox {
    pub mod scanner;
    pub mod token;
    pub mod token_type;
    pub mod interpreter;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let lox: Lox = Lox::new();
    if args.len() > 1 {
        println!("Usage: jlox [script]");
        process::exit(1);
    } else if args.len() == 5 {
        let _ = lox.run_file(&args[0]).unwrap_or_else(|_| {
            println!("Error");
        });
    } else {
        lox.run_prompt();
    }
}