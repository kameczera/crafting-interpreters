use super::token::Token;

#[derive(Debug,)]
pub struct Error {
    pub token: Token,
    pub string: String,
}

#[derive(Debug,)]
pub enum Exception {
    Error(Error),
    Continue,
    Break,
    Null,
}

impl Exception {
    pub fn error(token: Token, string: String) -> Self {
        Exception::Error(Error {
            token: token,
            string: string
        })
    }
}