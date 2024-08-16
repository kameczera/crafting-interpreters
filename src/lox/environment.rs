use std::collections::HashMap;
use std::mem;
use super::token::Token;
use super::objects::*;

pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    values: HashMap<Vec<u8>, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::from([]),
        }
    }
    
    pub fn new_child(enclosing: Environment) -> Self {
        Environment {
            enclosing: Option::Some(Box::new(enclosing)),
            values: HashMap::from([]),
        }
    }

    pub fn set_father(&mut self, enclosing: Environment) {
        self.enclosing = Some(Box::new(enclosing));
    }

    pub fn swap_env(&mut self) -> Option<Box<Environment>> {
        mem::replace(&mut self.enclosing, None)
    }

    pub fn define(&mut self, name: Vec<u8>, value: Object) {
        self.values.insert(name, value);
        println!("{:?}", self.values);
    }

    pub fn get(&mut self, name: Token) -> Result<Object, (Token, String)> {
        match self.values.remove(&name.lexeme) {
            Some(object) => return Ok(object),
            None => {
                if let Some(ref mut enclosing) = self.enclosing {
                    return enclosing.get(name);
                }
                let string = format!("Undefined variable '{}'.", String::from_utf8(name.lexeme.clone()).unwrap());
                return Err((name, string));
            }
        }
    }

    pub fn assign(&mut self, name: Token, value: &Object) -> Result<(), (Token, String)> {
        match self.values.get_mut(&name.lexeme) {
            Some(x) => *x = value.clone(),
            None => {
                if let Some(ref mut enclosing) = self.enclosing {
                    return enclosing.assign(name, value);
                }
                let lexeme_name = String::from_utf8(name.lexeme.clone()).unwrap();
                return Err((name, format!("Undefined variable '{}'.", lexeme_name)))
            }
        }
        return Ok(());
    }
}