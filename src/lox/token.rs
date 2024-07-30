use super::token_type::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Vec<u8>,
    pub literal: Literal,
    pub line: u32,
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

    pub fn to_string_lexeme(&self) -> String {
        String::from_utf8(self.lexeme.clone()).expect("Our bytes should be valid utf8")
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f32),
    None,
}