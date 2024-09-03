use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

use super::{
    environment, exception::{self, Exception}, objects::*
};

use super::{
    environment::*,
    expr::{Literal as Lit, *},
    stmt::*,
    token::{Literal as TokenLiteral, *},
    token_type::TokenType,
};

pub struct Interpreter<'a> {
    environment: Rc<RefCell<environment::Environment<'a>>>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<Object, Exception> {
        for statement in statements {
            match self.execute(statement) {
                Ok(object) => match object {
                    Object::Nil => (),
                    _ => println!("{}", self.stringify(object)),
                },
                Err(err) => return Err(err),
            }
        }
        return Ok(Object::Nil);
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Object, Exception> {
        match expr {
            Expr::Binary(expr) => self.visit_binary(expr),
            Expr::Grouping(expr) => self.visit_grouping(expr),
            Expr::Literal(expr) => self.visit_literal(expr),
            Expr::Logical(expr) => self.visit_logical_expr(expr),
            Expr::Unary(expr) => self.visit_unary(expr),
            Expr::Ternary(expr) => self.visit_ternary(expr),
            Expr::Variable(expr) => self.visit_variable(expr),
            Expr::Assign(expr) => self.visit_assign_expr(expr),
            Expr::Null => return Ok(Object::Nil),
        }
    }

    fn execute(&mut self, statement: Statement) -> Result<Object, Exception> {
        match statement {
            Statement::Expression(expression) => self.visit_expression_statement(*expression.expression),
            Statement::Print(print) => self.visit_print_statement(*print.expression),
            Statement::Var(var) => self.visit_var_statement(var),
            Statement::Block(block) => self.visit_block_statement(block),
            Statement::If(if_branch) => self.visit_if_statement(if_branch),
            Statement::While(while_branch) => self.visit_while_statement(while_branch),
            Statement::Break => self.visit_break_statement(),
            // Statement::Continue => self.visit_continue_statement(),
            // Null is used just for else statements
            Statement::Null => Ok(Object::Nil),
            _ => Ok(Object::Nil),
        }
    }

    fn execute_block(&mut self, statements: Vec<Statement>, mut old_env: Rc<RefCell<environment::Environment<'a>>>,) -> Result<Object, Exception> {
        for statement in statements {
            self.execute(statement)?;
        }
        mem::swap(&mut self.environment, &mut old_env);
        return Ok(Object::Nil);
    }

    fn visit_block_statement(&mut self, block: Block) -> Result<Object, Exception> {
        let new_env = Environment::new_child(Rc::clone(&self.environment));
        let old_env = mem::replace(&mut self.environment, new_env);
        self.execute_block(block.statements, old_env)
    }

    fn visit_expression_statement(&mut self, expr: Expr) -> Result<Object, Exception> {
        match self.evaluate(expr) {
            Ok(object) => return Ok(object),
            Err(err) => return Err(err),
        }
    }

    fn visit_if_statement(&mut self, if_branch: If) -> Result<Object, Exception> {
        let boolean = match self.evaluate(*if_branch.condition) {
            Ok(bool) => bool,
            Err(err) => return Err(err),
        };
        if self.is_truthy(&boolean) {
            self.execute(*if_branch.then_branch)?;
        } else {
            match *if_branch.else_branch {
                Statement::Null => return Ok(Object::Nil),
                _ => {
                    self.execute(*if_branch.else_branch)?;
                }
            }
        }
        return Ok(Object::Nil);
    }

    fn visit_print_statement(&mut self, expr: Expr) -> Result<Object, Exception> {
        let value = self.evaluate(expr);
        match value {
            Ok(object) => {
                println!("{}", self.stringify(object));
                return Ok(Object::Nil);
            }
            Err(err) => return Err(err),
        }
    }

    fn visit_var_statement(&mut self, statement: Var) -> Result<Object, Exception> {
        let mut value = Object::Nil;
        match *statement.initializer {
            Expr::Literal(Lit { value: Object::Nil }) => (),
            _ => {
                value = match self.evaluate(*statement.initializer) {
                    Ok(object) => object,
                    Err(err) => return Err(err),
                };
            }
        }
        self.environment
            .borrow_mut()
            .define(statement.name.lexeme, value);
        return Ok(Object::Nil);
    }

    fn visit_while_statement(&mut self, statement: While) -> Result<Object, Exception> { 
        loop {
            let condition = self.evaluate(*statement.condition.clone())?;
            if self.is_truthy(&condition) {
                match self.execute(*statement.body.clone()) {
                    Ok(_) => (),
                    Err(err) => {
                        match err {
                            Exception::Error(_) => return Err(err),
                            Exception::Continue => continue,
                            Exception::Break => {break},
                            _ => (),
                        }
                        
                    },
                }
            } else {
                break;
            }
        }

        return Ok(Object::Nil);
    }

    fn visit_break_statement(&mut self) -> Result<Object, Exception> {
        return Err(Exception::Break)
    }

    fn visit_assign_expr(&mut self, expr: Assign) -> Result<Object, Exception> {
        let value = self.evaluate(*expr.value)?;
        match self.environment.borrow_mut().assign(expr.name, &value) {
            Ok(()) => return Ok(value),
            Err(err) => return Err(err),
        }
    }

    fn visit_binary(&mut self, expr: Binary) -> Result<Object, Exception> {
        let left = self.evaluate(*expr.left)?;
        let right = self.evaluate(*expr.right)?;
        match expr.operator.token_type {
            TokenType::Minus => {
                if let Err(Exception) = self.check_number_binary(expr.operator, &left, &right)
                {
                    return Err(Exception);
                }
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Number(left - right));
            }
            TokenType::Slash => {
                self.check_number_binary(expr.operator, &left, &right)?;
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Number(left / right));
            }
            TokenType::Star => {
                self.check_number_binary(expr.operator, &left, &right)?;
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
                self.check_number_binary(expr.operator, &left, &right)?;
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Boolean(left > right));
            }
            TokenType::GreaterEqual => {
                self.check_number_binary(expr.operator, &left, &right)?;
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Boolean(left >= right));
            }
            TokenType::Less => {
                self.check_number_binary(expr.operator, &left, &right)?;
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Boolean(left < right));
            }
            TokenType::LessEqual => {
                self.check_number_binary(expr.operator, &left, &right)?;
                let left = Object::number(left);
                let right = Object::number(right);
                return Ok(Object::Boolean(left <= right));
            }
            TokenType::BangEqual => return Ok(Object::Boolean(!self.is_equal(left, right))),
            TokenType::EqualEqual => return Ok(Object::Boolean(self.is_equal(left, right))),
            _ => return Ok(Object::Nil),
        }
        // Unreachable
        return Err(Exception::Null);
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

    fn visit_ternary(&mut self, expr: Ternary) -> Result<Object, Exception> {
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
        return Err(Exception::Null);
    }

    fn visit_grouping(&mut self, expr: Grouping) -> Result<Object, Exception> {
        return self.evaluate(*expr.expression);
    }

    fn visit_literal(&mut self, expr: Lit) -> Result<Object, Exception> {
        Ok(expr.value)
    }

    fn visit_logical_expr(&mut self, expr: Logical) -> Result<Object, Exception> {
        let left = self.evaluate(*expr.left)?;
        if let TokenType::Or = expr.operator.token_type {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }
        return self.evaluate(*expr.right);
    }

    fn visit_unary(&mut self, expr: Unary) -> Result<Object, Exception> {
        let mut right = self.evaluate(*expr.right)?;
        match expr.operator.token_type {
            TokenType::Minus => {
                self.check_number_operand(expr.operator, &right)?;
                if let Object::Number(number) = right {
                    return Ok(Object::Number(-number));
                }
            }
            TokenType::Bang => {
                return Ok(Object::Boolean(!self.is_truthy(&right)));
            }
            _ => (),
        }
        // Unreachable
        return Err(Exception::Null);
    }

    fn visit_variable(&mut self, expr: Variable) -> Result<Object, Exception> {
        let t = self.environment.borrow_mut().get(expr.name);
        return t;
    }

    fn check_number_operand(
        &mut self,
        operator: Token,
        operand: &Object,
    ) -> Result<(), Exception> {
        if let Object::Number(number) = operand {
            return Ok(());
        }
        return Err(Exception::error(operator, String::from("Operand must be a number.")));
    }

    fn check_number_binary(
        &mut self,
        operator: Token,
        left: &Object,
        right: &Object,
    ) -> Result<(), Exception> {
        if let Object::Number(_number) = left {
            if let Object::Number(_number) = right {
                return Ok(());
            }
        }
        return Err(Exception::error(operator, String::from("Operand must be numbers.")));
    }

    fn is_truthy(&mut self, object: &Object) -> bool {
        match object {
            Object::Boolean(bool) => return *bool,
            Object::Nil => return false,
            _ => return true,
        }
    }
}
