use std::{cell::RefCell, rc::Rc};

use super::{
    environment::*, expr::{Literal as Lit, *}, stmt::*, token::{Literal as TokenLiteral, *}, token_type::TokenType
};


pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<Object, (Token, String)> {
        for statement in statements {
            if let Err(err) = self.execute(statement) {
                return Err(err);
            }
        }
        return Ok(Object::Nil);
    }
    
    fn evaluate(&mut self, expr: Expr) -> Result<Object, (Token, String)> {
        match expr {
            Expr::Binary(expr) => self.visit_binary(expr),
            Expr::Grouping(expr) => self.visit_grouping(expr),
            Expr::Literal(expr) => Ok(self.visit_literal(expr)),
            Expr::Unary(expr) => self.visit_unary(expr),
            Expr::Ternary(expr) => self.visit_ternary(expr),
            Expr::Variable(expr) => {self.visit_variable(expr)},
        }
    }
    
    fn execute(&mut self, statement: Statement) -> Result<Object, (Token, String)> {
        match statement {
            Statement::Expression(expression) => self.visit_expression_statement(*expression.expression),
            Statement::Print(print) => self.visit_print_statement(*print.expression),
            Statement::Var(var) => self.visit_var_statement(var),
        }
    }
    
    fn visit_expression_statement(&mut self, expr: Expr) -> Result<Object, (Token, String)> {
        let _ = self.evaluate(expr);
        return Ok(Object::Nil);
    }
    
    fn visit_print_statement(&mut self, expr: Expr) -> Result<Object, (Token, String)> {
        let value = self.evaluate(expr);
        match value {
            Ok(object) => {
                println!("{}", self.stringify(object));
                return Ok(Object::Nil);
            }
            Err(err) => return Err(err),
        }
    }
    
    fn visit_var_statement(&mut self, statement: Var) -> Result<Object, (Token, String)> {
        let mut value = Object::Nil;
        println!("{:?}", *statement.initializer);
        match *statement.initializer {
            Expr::Literal(Lit { value: Object::Nil }) => {
                value = Object::Nil;
            }
            _ => {
                value = match self.evaluate(*statement.initializer) {
                    Ok(object) => object,
                    Err(err) => return Err(err),
                };
            }
        }
        self.environment.define(statement.name.lexeme, value);
        return Ok(Object::Nil);
    }
    
    fn visit_binary(&mut self, expr: Binary) -> Result<Object, (Token, String)> {
        let left = match self.evaluate(*expr.left) {
            Ok(object) => object,
            Err(err) => return Err(err),
        };
        let right = match self.evaluate(*expr.right) {
            Ok(object) => object,
            Err(err) => return Err(err),
        };
        match expr.operator.token_type {
            TokenType::Minus => {
                if let Err((token, string)) = self.check_number_binary(expr.operator, &left, &right) {
                    return Err((token, string));
                }
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Number(left - right));
            }
            TokenType::Slash => {
                if let Err((token, string)) = self.check_number_binary(expr.operator, &left, &right) {
                    return Err((token, string));
                }
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Number(left / right));
            }
            TokenType::Star => {
                if let Err((token, string)) = self.check_number_binary(expr.operator, &left, &right) {
                    return Err((token, string));
                }
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Number(left * right));
            }
            TokenType::Plus => {
                let values = (left, right);
                if let (Object::String(mut left_value), Object::String(right_value)) = values {
                    left_value.push_str(&right_value);
                    return Ok(Object::String(left_value));
                } else if let (Object::Number(left_value), Object::Number(right_value)) = values {
                    return Ok(Object::Number(left_value + right_value));
                } else if let (Object::String(left_value)) = values.0 {
                    let mut string = left_value;
                    string.push_str(&self.stringify(values.1));
                    return Ok(Object::String(string));
                }
            }
            TokenType::Greater => {
                if let Err((token, string)) = self.check_number_binary(expr.operator, &left, &right) {
                    return Err((token, string));
                }
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Boolean(left > right));
            }
            TokenType::GreaterEqual => {
                if let Err((token, string)) = self.check_number_binary(expr.operator, &left, &right) {
                    return Err((token, string));
                }
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Boolean(left >= right));
            }
            TokenType::Less => {
                if let Err((token, string)) = self.check_number_binary(expr.operator, &left, &right) {
                    return Err((token, string));
                }
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Boolean(left < right));
            }
            TokenType::LessEqual => {
                if let Err((token, string)) = self.check_number_binary(expr.operator, &left, &right) {
                    return Err((token, string));
                }
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Boolean(left <= right));
            }
            TokenType::BangEqual => return Ok(Object::Boolean(!self.is_equal(left, right))),
            TokenType::EqualEqual => return Ok(Object::Boolean(self.is_equal(left, right))),
            _ => return Ok(Object::Nil),
        }
        // Unreachable
        return Err((
            Token::new(TokenType::And, b"".to_vec(), 0, TokenLiteral::None),
            String::from(""),
        ));
    }
    
    fn is_equal(&mut self, a: Object, b: Object) -> bool {
        if a == Object::Nil && b == Object::Nil {
            return true;
        } else if a == Object::Nil {
            return false;
        }
        // TODO: confirm that this is working fine
        return a == b;
    }
    
    fn stringify(&mut self, object: Object) -> String {
        if let Object::Nil = object {
            return String::from("nil");
        }
    
        if let Object::Number(number) = object {
            let text: String = number.to_string();
            if text.ends_with(".0") {
                return String::from(&text[0..text.len() - 2]);
            }
            return text;
        }
    
        if let Object::String(string) = object {
            return string;
        }
        return object.to_string();
    }
    
    fn visit_ternary(&mut self, expr: Ternary) -> Result<Object, (Token, String)> {
        let expression = match self.evaluate(*expr.expression) {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };
    
        match expression {
            Object::Boolean(bool) => {
                if bool {
                    let true_part = self.evaluate(*expr.true_part);
                    return true_part;
                } else {
                    let false_part = self.evaluate(*expr.false_part);
                    return false_part;
                }
            }
            _ => (),
        }
        // Unreachable
        return Err((
            Token::new(TokenType::And, b"".to_vec(), 0, TokenLiteral::None),
            String::from(""),
        ));
    }
    
    fn visit_grouping(&mut self, expr: Grouping) -> Result<Object, (Token, String)> {
        return self.evaluate(*expr.expression);
    }
    
    fn visit_literal(&mut self, expr: Lit) -> Object {
        return expr.value;
    }
    
    fn visit_unary(&mut self, expr: Unary) -> Result<Object, (Token, String)> {
        let mut right = match self.evaluate(*expr.right) {
            Ok(expr) => expr,
            Err(err) => return Err(err),
        };
        match expr.operator.token_type {
            TokenType::Minus => {
                self.check_number_operand(expr.operator, &right);
                if let Object::Number(number) = right {
                    return Ok(Object::Number(-number));
                }
            }
            TokenType::Bang => {
                return Ok(Object::Boolean(!self.is_truthy(right)));
            }
            _ => (),
        }
        // Unreachable
        return Err((
            Token::new(TokenType::And, b"".to_vec(), 0, TokenLiteral::None),
            String::from(""),
        ));
    }
    
    fn visit_variable(&mut self, expr: Variable) -> Result<Object, (Token, String)> {
        let t = self.environment.get(expr.name);
        return t;
    }
    
    fn check_number_operand(&mut self, operator: Token, operand: &Object) -> Result<(), (Token, String)> {
        if let Object::Number(number) = operand {
            return Ok(());
        }
        return Err((operator, String::from("Operand must be a number.")));
    }
    
    fn check_number_binary(&mut self, operator: Token,left: &Object,right: &Object) -> Result<(), (Token, String)> {
        if let Object::Number(_number) = left {
            if let Object::Number(_number) = right {
                return Ok(());
            }
        }
        return Err((operator, String::from("Operand must be numbers.")));
    }
    
    fn is_truthy(&mut self, object: Object) -> bool {
        match object {
            Object::Boolean(bool) => return bool,
            Object::Nil => return false,
            _ => return true,
        }
    }
}