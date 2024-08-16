use super::expr::*;
use super::stmt::Statement;
use super::token::*;
use super::token::Literal as Lit;
use super::token_type::*;
use super::objects::*;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl Parser<'_> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }
    
    pub fn parse(&mut self) -> Result<Vec<Statement>, (Token, String)> {
        let mut statements = vec![];
        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(err) => return Err(err),
            }
        }
        return Ok(statements);
    }

    fn expression(&mut self) -> Result<Expr, (Token, String)> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Expr, (Token, String)> {
        let expr = match self.ternary() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };

        if self.mtch(vec![TokenType::Equal]) {
            let equals: Token = self.previous().clone();
            let value = match self.assignment() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };

            match expr {
                Expr::Variable(variable) => {
                    let name = variable.name;
                    return Ok(Expr::assign(name, value));
                }
                _ => return Err((equals, String::from("Invalid assignment target."))),
            }
        }
        return Ok(expr);
    }

    fn declaration(&mut self) -> Result<Statement, (Token, String)> {
        if self.mtch(vec![TokenType::Var]) {
            return self.var_declaration();
        }

        match self.statement() {
            Ok(statement) => return Ok(statement),
            Err(err) => {
                self.synchronize();
                return Err(err);
            }
        }
    }

    fn statement(&mut self) -> Result<Statement, (Token, String)> {
        if self.mtch(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.mtch(vec![TokenType::LeftBrace]) {
            let statements = match self.block() {
                Ok(statements) => statements,
                Err(err) => return Err(err),
            };
            return Ok(Statement::block(statements));
        }
        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Result<Statement, (Token, String)> {
        let unwraped_expr = match self.expression() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };
        match self.consume(TokenType::Semicolon, String::from("Expect ';' after value.")) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
        return Ok(Statement::print(unwraped_expr));
    }

    fn var_declaration(&mut self) -> Result<Statement, (Token, String)> {
        let name = match self.consume(TokenType::Identifier, String::from("Expect variable name")) {
            Ok(token) => token.clone(),
            Err(err) => return Err(err),
        };
        let mut initializer = Expr::literal(Object::Nil);
        if self.mtch(vec![TokenType::Equal]) {
            match self.expression() {
                Ok(expr) => {
                    initializer = expr;
                },
                Err(err) => return Err(err),
            }
        }
        match self.consume(TokenType::Semicolon, String::from("Expect ';' after variable declaration.")) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
        return Ok(Statement::var(name, initializer));
    }
    
    fn expression_statement(&mut self) -> Result<Statement, (Token, String)> {
        let unwraped_expr = match self.expression() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };
        match self.consume(TokenType::Semicolon, String::from("Expect ';' after expression.")) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
        return Ok(Statement::expression(unwraped_expr));
    }
    
    fn block(&mut self) -> Result<Vec<Statement>, (Token, String)> {
        let mut statements: Vec<Statement> = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(err) => return Err(err),
            }
        }
        self.consume(TokenType::RightBrace, String::from("Expect '}' after block."));
        return Ok(statements);
    }
    
    fn ternary(&mut self) -> Result<Expr, (Token, String)> {
        let mut unwraped_expr = match self.equality() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };
        if self.mtch(vec![TokenType::QuestionMark]) {
            let true_part = match self.equality() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            if !self.mtch(vec![TokenType::Colon]) {
                return Err((self.peek().clone(), String::from("Expect ':'")));
            }
            let false_part = match self.equality() {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            unwraped_expr = Expr::ternary(unwraped_expr, true_part, false_part);
        }
        return Ok(unwraped_expr);
    }

    fn equality(&mut self) -> Result<Expr, (Token, String)> {
        let mut unwraped_expr = match self.comparison() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };
        while self.mtch(vec![TokenType::BangEqual,TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            match right {
                Ok(expr) => unwraped_expr = Expr::binary(unwraped_expr, operator, expr),
                Err(err) => return Err(err),
            };
        }
        return Ok(unwraped_expr);
    }

    fn comparison(&mut self) -> Result<Expr, (Token, String)> {
        let mut unwraped_expr = match self.term() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };
        while self.mtch(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.term();
            match right {
                Ok(expr) => unwraped_expr = Expr::binary(unwraped_expr, operator, expr),
                Err(err) => return Err(err),
            };
        }
        return Ok(unwraped_expr);
    }

    fn term(&mut self) -> Result<Expr, (Token, String)> {
        let mut unwraped_expr = match self.factor() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };
        while self.mtch(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            match right {
                Ok(expr) => unwraped_expr = Expr::binary(unwraped_expr, operator, expr),
                Err(err) => return Err(err),
            };
        }
        return Ok(unwraped_expr);
    }

    fn factor(&mut self) -> Result<Expr, (Token, String)> {
        let mut unwraped_expr = match self.unary() {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };
        while self.mtch(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            match right {
                Ok(expr) => unwraped_expr = Expr::binary(unwraped_expr, operator, expr),
                Err(err) => return Err(err),
            };
        }
        return Ok(unwraped_expr);
    }

    fn unary(&mut self) -> Result<Expr, (Token, String)> {
        if self.mtch(vec![TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous().clone();
            let right = self.primary();
            let right = match right {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            return Ok(Expr::unary(operator, right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, (Token, String)> {
        if self.mtch(vec![TokenType::False]) {
            return Ok(Expr::literal(Object::Boolean(false)));
        } 
        if self.mtch(vec![TokenType::True]) {
            return Ok(Expr::literal(Object::Boolean(true)));
        }
        if self.mtch(vec![TokenType::Nil]) {
            return Ok(Expr::literal(Object::Nil));
        }
        if self.mtch(vec![TokenType::Number, TokenType::String]) {
            match self.previous().literal.clone() {
               Lit::String(s) => return Ok(Expr::literal(Object::String(s))),
               Lit::Number(s) => return Ok(Expr::literal(Object::Number(s))),
               _ => (), // Unreacheble
            }
            
        }
        if self.mtch(vec![TokenType::Identifier]) {
            return Ok(Expr::variable(self.previous().clone()));
        }
        if self.mtch(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            let expr = match expr {
                Ok(expr) => expr,
                Err(err) => return Err(err),
            };
            match self.consume(TokenType::RightParen, String::from("Expect ')' after expression.")) {
                Ok(_) => (),
                Err(err) => return Err(err),
            }
            return Ok(Expr::grouping(expr));
        }
        Err((self.peek().clone(), String::from("Expected expression."))) // Result<&Token, (u32, String, String)>
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

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<&Token, (Token, String)> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        return Err((self.peek().clone(), message))
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        // Use of discriminant (PartialEq)
        if self.peek().token_type == token_type {
            return true;
        }
        return self.peek().token_type == token_type;
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

    fn synchronize(&mut self) {
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