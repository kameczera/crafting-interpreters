use super::token::*;
use super::objects::*;

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Debug)]
pub struct Ternary {
    pub expression: Box<Expr>,
    pub true_part: Box<Expr>,
    pub false_part: Box<Expr>,
}

#[derive(Debug)]
pub struct Literal {
    pub value: Object,
}

#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct Variable {
    pub name: Token,
}

#[derive(Debug)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}
#[derive(Debug)]
pub enum Expr {
    Binary(Binary),
    Assign(Assign),
    Grouping(Grouping),
    Literal(Literal),
    Ternary(Ternary),
    Unary(Unary),
    Variable(Variable),
}

impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
        })
    }
    
    pub fn ternary(expression: Expr, true_part: Expr, false_part: Expr) -> Self {
        Expr::Ternary(Ternary {
            expression: Box::new(expression),
            true_part: Box::new(true_part),
            false_part: Box::new(false_part),
        })
    }

    pub fn unary(operator: Token, right: Expr) -> Self {
        Expr::Unary (Unary {
            operator: operator,
            right: Box::new(right),
        })
    }

    pub fn literal(value: Object) -> Self {
        Expr::Literal(Literal {
            value: value 
        })
    }

    pub fn grouping(expression: Expr) -> Self {
        Expr::Grouping(Grouping {
            expression: Box::new(expression),
        })
    }
    pub fn variable(name: Token) -> Self {
        Expr::Variable(Variable {
            name: name,
        })
    }

    pub fn assign(name: Token, value: Expr) -> Self {
        Expr::Assign(Assign {
            name: name,
            value: Box::new(value),
        })
    }
}
