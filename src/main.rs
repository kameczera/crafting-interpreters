use std::env;
use std::process;

mod lox {
    pub mod scanner;
    pub mod token;
    pub mod token_type;
    pub mod lang;
    pub mod parser;
    pub mod expr;
    // pub mod ast_nner;
    pub mod interpreter;
    pub mod stmt;
    pub mod environment;
    pub mod objects;
    pub mod exception;
    pub mod lox_callable;
}

use crate::lox::lang::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox: Lox = Lox::new();
    if args.len() > 2 {
        println!("Usage: jlox [script]");
        process::exit(1);
    } else if args.len() == 2 {
        let _ = lox.run_file(&args[1]).unwrap_or_else(|_| {
            println!("Error");
        });
    } else {
        lox.run_prompt();
    }
}

// fn main() {
//     let expression: Expr = Expr::binary(
//         Box::new(Expr::unary(
//             Token::new(TokenType::Minus, vec![b'-'], 1, Lit::None),
//             Box::new(Expr::literal(Object::number(123.0)))
//         )),
//         Token::new(TokenType::Star, vec![b'*'], 1, Lit::None),
//         Box::new(Expr::grouping(
//             Box::new(Expr::literal(Object::Number(45.67)))
//         ))
//     );
//     println!("{}", ast_printer::print(expression));
// }