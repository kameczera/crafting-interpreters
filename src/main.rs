use std::env;
use std::process;

use lox::ast_printer;
use lox::expr::*;
use lox::token::*;
use lox::token::Literal as Lit;
use lox::token_type::*;
use lox::ast_printer::*;

use crate::lox::lang::Lox;

mod lox {
    pub mod scanner;
    pub mod token;
    pub mod token_type;
    pub mod lang;
    pub mod parser;
    pub mod expr;
    pub mod ast_printer;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox: Lox = Lox::new();
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