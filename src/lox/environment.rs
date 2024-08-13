use std::{cell::RefCell, collections::HashMap, rc::Rc};
use super::{expr::Object, token::Token};

pub struct Environment {
    values: HashMap<Vec<u8>, Object>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::from([]),
        }
    }

    pub fn define(&mut self, name: Vec<u8>, value: Object) {
        self.values.insert(name, value);
        println!("{:?}", self.values);
    }

    pub fn get(&mut self, name: Token) -> Result<Object, (Token, String)> {
        match self.values.remove(&name.lexeme) {
            Some(object) => return Ok(object),
            None => {
                let string = format!("Undefined variable '{}'.", String::from_utf8(name.lexeme.clone()).unwrap());
                return Err((name, string));
            }
        }
    }
}