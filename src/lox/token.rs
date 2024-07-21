use super::token_type::*;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: Vec<u8>,
    literal: Literal,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: Vec<u8>, line: u32, literal: Literal) -> Self {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            line: line,
            literal: literal,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{:?} {:?}", self.token_type, self.lexeme);
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f32),
    None,
}