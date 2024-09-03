use std::collections::HashMap;
use std::mem;
use std::cell::RefCell;
use std::rc::Rc;
use super::exception::Exception;
use super::token::Token;
use super::objects::*;
use super::exception;

pub struct Environment<'a> {
    pub enclosing: Option<Rc<RefCell<Environment<'a>>>>,
    values: HashMap<Vec<u8>, Object>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            enclosing: None,
            values: HashMap::new(),
        }))
    }

    pub fn new_child(enclosing: Rc<RefCell<Environment<'a>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }))
    }

    pub fn set_father(&mut self, enclosing: Rc<RefCell<Environment<'a>>>) {
        self.enclosing = Some(enclosing);
    }

    pub fn define(&mut self, name: Vec<u8>, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Object, Exception> {
        match self.values.get(&name.lexeme) {
            Some(object) => return Ok(object.clone()),
            None => {
                if let Some(ref enclosing) = self.enclosing {
                    return enclosing.borrow().get(name);
                }
                let string = format!("Undefined variable '{}'.", String::from_utf8(name.lexeme.clone()).unwrap());
                return Err(Exception::error(name, string));
            }
        }
    }
    
    pub fn assign(&mut self, name: Token, value: &Object) -> Result<(), Exception> {
        match self.values.get_mut(&name.lexeme) {
            Some(x) => *x = value.clone(),
            None => {
                if let Some(ref enclosing) = self.enclosing {
                    return enclosing.borrow_mut().assign(name, value);
                }
                let lexeme_name = String::from_utf8(name.lexeme.clone()).unwrap();
                return Err(Exception::error(name, format!("Undefined variable '{}'.", lexeme_name)));
            }
        }
        Ok(())
    }
}