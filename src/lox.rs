use crate::Args;
use std::{fs, io};

struct Scanner {
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
            _ => Lox::error(self.line, "Unexpected character."),
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

enum TokenType {
    // single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: &str, line: usize) -> Token {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            line,
        }
    }
}

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn exec(&mut self, args: &Args) -> io::Result<()> {
        match &args.script {
            Some(path) => self.run_file(path),
            None => self.run_prompt(),
        }
    }

    fn run_file(&mut self, path: &str) -> io::Result<()> {
        let content = fs::read_to_string(path)?;
        self.run(&content);

        if self.had_error {
            std::process::exit(65);
        }
        Ok(())
    }

    fn run_prompt(&mut self) -> io::Result<()> {
        let reader = io::stdin();

        for line in reader.lines() {
            let line = line?;
            println!("> {}", line);
            self.run(&line);
            self.had_error = false;
        }

        Ok(())
    }

    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        // 临时打印 tokens 用于调试
        for token in tokens {
            println!("{:?}", token.lexeme);
        }
    }

    fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: usize, position: &str, message: &str) {
        eprintln!("[line {}] Error: {}: {}", line, position, message);
    }
}
