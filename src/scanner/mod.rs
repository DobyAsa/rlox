use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, "", self.line));
        std::mem::take(&mut self.tokens)
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '/' => self.add_token(TokenType::Slash),
            '!' => self.match_token('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.match_token('=', TokenType::EqualEqual, TokenType::Equal),
            '>' => self.match_token('=', TokenType::GreaterEqual, TokenType::Greater),
            '<' => self.match_token('=', TokenType::LessEqual, TokenType::Less),
            ' ' | '\r' | '\t' => (), // 忽略空白字符
            '\n' => self.line += 1,
            _ => {},
        }
    }

    fn match_token(&mut self, expected: char, matched: TokenType, unmatched: TokenType) {
        let is_match = self.next_char(expected);
        self.add_token(if is_match { matched } else { unmatched });
    }

    fn next_char(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let c: char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            &self.source[self.start..self.current],
            self.line,
        ));
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}