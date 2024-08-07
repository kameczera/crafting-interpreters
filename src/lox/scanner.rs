use super::token::*;
use super::token_type::*;
use std::collections::HashMap;

pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
    keywords: HashMap<Vec<u8>, TokenType>,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Self {
        Scanner {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
            keywords: HashMap::from([
                (b"and".to_vec(), TokenType::And),
                (b"class".to_vec(), TokenType::Class),
                (b"else".to_vec(), TokenType::Else),
                (b"false".to_vec(), TokenType::False),
                (b"for".to_vec(), TokenType::For),
                (b"fun".to_vec(), TokenType::Fun),
                (b"if".to_vec(), TokenType::If),
                (b"nil".to_vec(), TokenType::Nil),
                (b"or".to_vec(), TokenType::Or),
                (b"print".to_vec(), TokenType::Print),
                (b"return".to_vec(), TokenType::Return),
                (b"super".to_vec(), TokenType::Super),
                (b"this".to_vec(), TokenType::This),
                (b"true".to_vec(), TokenType::True),
                (b"var".to_vec(), TokenType::Var),
                (b"while".to_vec(), TokenType::While),
                ])
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, vec![], self.line, Literal::None));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            b'(' => self.add_token(TokenType::LeftParen),
            b')' => self.add_token(TokenType::RightParen),
            b'{' => self.add_token(TokenType::LeftBrace),
            b'}' => self.add_token(TokenType::RightBrace),
            b',' => self.add_token(TokenType::Comma),
            b'.' => self.add_token(TokenType::Dot),
            b'-' => self.add_token(TokenType::Minus),
            b'+' => self.add_token(TokenType::Plus),
            b';' => self.add_token(TokenType::Semicolon),
            b'*' => self.add_token(TokenType::Star),
            b'?' => self.add_token(TokenType::QuestionMark),
            b':' => self.add_token(TokenType::Colon),
            b'!' => {
                let token_type = if self.compl('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            },
            b'=' => {
                let token_type = if self.compl('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            },
            b'<' => {
                let token_type = if self.compl('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            },
            b'>' => {
                let token_type = if self.compl('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            },
            b'/' => {
                if self.compl('/'){
                    while self.peek() != b'\n' && !self.is_at_end() { self.advance(); };
                } else if self.compl('*'){
                    while self.peek() != b'*' && self.peek_next() != b'/' && !self.is_at_end() { self.advance(); };
                    self.current += 2;
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            b' ' => {},
            b'\r' => {},
            b'\n' => {},
            b'"' => {
                self.string();
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.idenfitier();
                }
            }
        }
    }

    fn idenfitier(&mut self){
        while self.is_alpha_numeric(self.peek()) { self.advance(); };
        let text = &self.source[self.start..self.current];
        let type_identifier = match self.keywords.get(text){
            Some(&t) => t,
            None => {
                TokenType::Identifier
            }
        };
        self.add_token(type_identifier);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) { self.advance();}
        if self.peek() == b'.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) { self.advance(); }
            
        }
        let val: f32 = String::from_utf8(self.source[self.start..self.current].to_vec())
        .unwrap()
        .parse()
        .unwrap();
        self.add_token_literal(TokenType::Number, Literal::Number(val));
    }

    fn string(&mut self) {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {self.line+= 1;}
            self.advance();
        }

        if self.is_at_end() {
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        let value = core::str::from_utf8(value).unwrap().to_string();
        self.add_token_literal(TokenType::String, Literal::String(value));
    }

    fn compl(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] as char != expected {
            return false;
        }
        self.current += 1;
        return true;
    }
    
    fn peek(&self) -> u8 {
        if self.is_at_end() { return b'\0'; }
        self.source[self.current] as u8
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() { return b'\0';}
        return self.source[self.current + 1];
    }

    fn is_alpha(&mut self,c: u8) -> bool {
        return (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_';
    }

    fn is_alpha_numeric(&mut self, c: u8) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }

    fn is_digit(&self, c: u8) -> bool {
        return c >= b'0' && c <= b'9';
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[(self.current - 1) as usize]
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, Literal::None);
    }
    
    fn add_token_literal(&mut self, token_type: TokenType, literal: Literal){
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text.to_vec(), self.line, literal));
    }
}
