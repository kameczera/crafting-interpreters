use super::token::*;

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Literal {
    pub value: Object,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub enum Object {
    Boolean(bool),
    String(String),
    Number(f32),
    Nil,
}

impl Object {
    pub fn to_string(self) -> String {
        match self {
            Object::Number(number) => number.to_string(),
            Object::String(string) => string,
            Object::Boolean(bool) => if bool {
                String::from("true")
            } else {
                String::from("false")
            },
            Object::Nil => String::from("nil"),
        }
    }

    pub fn bool(bool: bool) -> Self {
        Object::Boolean(bool)
    }

    pub fn string(string: String) -> Self {
        Object::String(string)
    }
    
    pub fn number(number: f32) -> Self {
        Object::Number(number)
    }

    pub fn nil() -> Self {
        Object::Nil
    }
}

impl Expr {
    pub fn binary(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Expr::Binary(Binary {
            left: left,
            operator: operator,
            right: right,
        })
    }

    pub fn unary(operator: Token, right: Box<Expr>) -> Self {
        Expr::Unary (Unary {
            operator: operator,
            right: right,
        })
    }

    pub fn literal(value: Object) -> Self {
        Expr::Literal(Literal {
            value: value 
        })
    }

    pub fn grouping(expression: Box<Expr>) -> Self {
        Expr::Grouping (Grouping {
            expression: expression,
        })
    }
}
