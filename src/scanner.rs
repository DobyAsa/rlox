use crate::{lox::Lox, token::{Token, TokenType}};



pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, "("),
            ')' => self.add_token(TokenType::RightParen, ")"),
            '{' => self.add_token(TokenType::LeftBrace, "{"),
            '}' => self.add_token(TokenType::RightBrace, "}"),
            ',' => self.add_token(TokenType::Comma, ","),
            '.' => self.add_token(TokenType::Dot, "."),
            '-' => self.add_token(TokenType::Minus, "-"),
            '+' => self.add_token(TokenType::Plus, "+"),
            ';' => self.add_token(TokenType::Semicolon, ";"),
            '*' => self.add_token(TokenType::Star, "*"),
            '!' => {
                let token_type = if self.match_('=') {
                    (TokenType::BangEqual, "!=")
                } else {
                    (TokenType::Bang, "!")
                };
                self.add_token(token_type.0, token_type.1)
            },
            '=' => {
                let token_type = if self.match_('=') {
                    (TokenType::EqualEqual, "==")
                } else {
                    (TokenType::Equal, "=")
                };
                self.add_token(token_type.0, token_type.1)
            },
            '<' => {
                let token_type = if self.match_('=') {
                    (TokenType::LessEqual, "<=")
                } else {
                    (TokenType::Less, "<")
                };
                self.add_token(token_type.0, token_type.1)
            },
            '>' => {
                let token_type = if self.match_('=') {
                    (TokenType::GreaterEqual, ">=")
                } else {
                    (TokenType::Greater, ">")
                };
                self.add_token(token_type.0, token_type.1)
            },
            '/' => {
                if self.match_('/') {
                    // 注释
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, "/");
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            _ => Lox::error(self.line, "Unexpected character."),
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: &str) {
        self.tokens.push(Token {
            token_type,
            lexeme: lexeme.to_string(),
            line: self.line,
        });
    }
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }
    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }
    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }
    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }
    
}