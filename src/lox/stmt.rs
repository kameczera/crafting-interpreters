use super::{expr::*, token::Token};

#[derive(Clone, Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub expression: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Print {
    pub expression: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Var {
    pub name: Token,
    pub initializer: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct If {
    pub condition: Box<Expr>,
    pub then_branch: Box<Statement>,
    pub else_branch: Box<Statement>,
}

#[derive(Clone, Debug)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: Box<Statement>,
}

#[derive(Clone, Debug)]
pub enum Statement {
    Block(Block),
    Expression(Expression),
    If(If),
    Print(Print),
    Var(Var),
    While(While),
    Break,
    Continue,
    Null,
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

    pub fn if_branch(condition: Expr, then_branch: Statement, else_brach: Statement) -> Self {
        Statement::If(If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_brach)
        })
    }
    
    pub fn while_branch(condition: Expr, body: Statement) -> Self {
        Statement::While(While {
            condition: Box::new(condition),
            body: Box::new(body),
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