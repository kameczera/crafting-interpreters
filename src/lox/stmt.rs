use super::{expr::*, token::Token};

pub struct Block {
    pub statements: Vec<Statement>,
}

pub struct Expression {
    pub expression: Box<Expr>,
}

pub struct Print {
    pub expression: Box<Expr>,
}

pub struct Var {
    pub name: Token,
    pub initializer: Box<Expr>,
}

pub enum Statement {
    Block(Block),
    Expression(Expression),
    Print(Print),
    Var(Var),
}

impl Statement {
    pub fn expression(expression: Expr) -> Self {
        Statement::Expression(Expression {
            expression: Box::new(expression),
        })
    }
    
    pub fn print(expression: Expr) -> Self {
        Statement::Print(Print {
            expression: Box::new(expression),
        })
    }

    pub fn var(name: Token, initializer: Expr) -> Self {
        Statement::Var(Var {
            name: name,
            initializer: Box::new(initializer),
        })
    }

    pub fn block(statements: Vec<Statement>) -> Self {
        Statement::Block(Block {
            statements: statements,
        })
    }
}