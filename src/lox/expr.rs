use super::token::*;
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
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Ternary(Ternary),
    Unary(Unary),
    Variable(Variable),
}

#[derive(PartialEq, Debug)]
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
    pub fn bool(self) -> bool {
        if let Object::Boolean(bool) = self {
            bool
        } else {
            // Unreachable
            panic!("Not a bool!");
        }
    }

    pub fn is_string(&self) -> bool {
        if let Object::String(s) = self {
            return true
        } else {
            return false
        }
    }

    pub fn string(self) -> String {
        if let Object::String(string) = self {
            string
        } else {
            // Unreachable
            panic!("Not a string!");
        }
    }
    
    pub fn number(self) -> f32 {
        if let Object::Number(number) = self {
            number
        } else {
            // Unreachable
            panic!("Not a number!");
        }
    }

    pub fn is_number(&self) -> bool {
        if let Object::Number(s) = self {
            return true
        } else {
            return false
        }
    }
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
}
