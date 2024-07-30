use super::expr::*;
use super::token;
use super::token_type::*;
use super::token::*;
use std::process;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }
    
    pub fn parse(&mut self) -> Expr {
        return self.expression();
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.mtch(vec![TokenType::BangEqual,TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.mtch(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.mtch(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.mtch(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::binary(Box::new(expr), operator, Box::new(right));
        }
        return expr;
    }

    fn unary(&mut self) -> Expr {
        if self.mtch(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.primary();
            return Expr::unary(operator, Box::new(right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.mtch(vec![TokenType::False]) {
            return Expr::literal(Object::Boolean(false));
        } 
        if self.mtch(vec![TokenType::True]) {
            return Expr::literal(Object::Boolean(true));
        }
        if self.mtch(vec![TokenType::Nil]) {
            return Expr::literal(Object::Nil);
        }
        if self.mtch(vec![TokenType::Number, TokenType::String]) {
            match(self.previous().literal.clone()) {
               Literal::String(s) => return Expr::literal(Object::String(s)),
               Literal::Number(s) => return Expr::literal(Object::Number(s)),
               _ => (), // Unreacheble
            }
            
        }
        if self.mtch(vec![TokenType::String]) {

        }
        if self.mtch(vec![TokenType::LeftParen]) {
            let expr: Expr = self.expression();
            self.consume(TokenType::RightParen, String::from("Expect ')' after expression."));
            return Expr::grouping(Box::new(expr));
        }
        
        self.error()
    }

    fn mtch(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if (self.check(token_type)) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> &Token {
        if self.check(token_type) {
            return self.advance();
        }
        println!("Error");
        process::exit(1);
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        if matches!(self.peek().token_type, token_type) {
            return true;
        }
        return matches!(self.peek().token_type, token_type);
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        if let TokenType::Eof = self.peek().token_type {
            return true;
        }
        return false;
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }

    fn error(&self) -> Expr {
        println!("Error");
        process::exit(1);
    }

    fn synchrnonize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if let TokenType::Semicolon = self.peek().token_type {
                return;
            }
            match self.peek().token_type {
                TokenType::Class => break,
                TokenType::Fun => break,
                TokenType::Var => break,
                TokenType::For => break,
                TokenType::If => break,
                TokenType::While => break,
                TokenType::Print => break,
                TokenType::Return => break,
                _ => (),
            }
            self.advance();
        }
    }

}