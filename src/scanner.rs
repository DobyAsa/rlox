#![allow(unused)]
use crate::{lox::Lox, token::{Token, TokenType}};

/// Lexical scanner for the Lox language
/// 
/// Breaks source code into tokens for parsing.
/// 
/// # Fields
/// 
/// * `source` - The source code being scanned
/// * `start` - Start position of the current lexeme
/// * `current` - Current position in the source code
/// * `line` - Current line number being processed
/// * `tokens` - Collection of scanned tokens
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    pub tokens: Vec<Token>,
}

impl Scanner {
    /// Creates a new Scanner instance
    /// 
    /// # Arguments
    /// 
    /// * `source` - The source code to be scanned
    /// 
    /// # Returns
    /// 
    /// A new Scanner initialized to the beginning of the source
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }

    /// Scans the entire source code and produces tokens
    /// 
    /// Processes the source code from start to finish, generating
    /// a sequence of tokens for the parser.
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

    }

    /// check if we are at the end of the source code
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// scan a single token
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
            _ => {
                Lox::error(self.line, "Unexpected character.");
                panic!("Unexpected character.");
            },
        }
    }

    /// add a token to the list of tokens
    fn add_token(&mut self, token_type: TokenType, lexeme: &str) {
        self.tokens.push(Token {
            token_type,
            lexeme: lexeme.to_string(),
            line: self.line,
        });
    }

    /// advance the current position and return the character at the current position
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    /// check if the current character matches the expected character
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

    /// peek at the next character
    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    /// peek at the next character
    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_at_end_empty_source() {
        let scanner = Scanner::new(String::new());
        assert!(scanner.is_at_end(), "Should be at end with empty source");
    }

    #[test]
    fn test_is_at_end_at_start() {
        let scanner = Scanner::new("hello".to_string());
        assert!(!scanner.is_at_end(), "Should not be at end at start of source");
    }

    #[test]
    fn test_is_at_end_in_middle() {
        let mut scanner = Scanner::new("hello".to_string());
        scanner.current = 2;
        assert!(!scanner.is_at_end(), "Should not be at end in middle of source");
    }

    #[test]
    fn test_is_at_end_at_end() {
        let mut scanner = Scanner::new("hello".to_string());
        scanner.current = 5; // length of "hello"
        assert!(scanner.is_at_end(), "Should be at end of source");
    }

    #[test]
    fn test_is_at_end_past_end() {
        let mut scanner = Scanner::new("hello".to_string());
        scanner.current = 6; // beyond length of "hello"
        assert!(scanner.is_at_end(), "Should be at end when past source length");
    }

    #[test]
    fn test_is_at_end_with_unicode() {
        let mut scanner = Scanner::new("你好".to_string());
        scanner.current = 6; // UTF-8 byte length of "你好"
        assert!(scanner.is_at_end(), "Should handle end position correctly with Unicode string");
    }

    #[test]
    fn test_advance_single_char() {
        let mut scanner = Scanner::new("a".to_string());
        assert_eq!(scanner.advance(), 'a');
        assert_eq!(scanner.current, 1);
    }

    #[test]
    fn test_advance_multiple_chars() {
        let mut scanner = Scanner::new("hello".to_string());
        assert_eq!(scanner.advance(), 'h');
        assert_eq!(scanner.advance(), 'e');
        assert_eq!(scanner.current, 2);
    }

    #[test]
    fn test_advance_with_unicode() {
        let mut scanner = Scanner::new("你好".to_string());
        assert_eq!(scanner.advance(), '你');
        assert_eq!(scanner.advance(), '好');
        assert_eq!(scanner.current, 2);
    }

    #[test]
    fn test_advance_with_special_chars() {
        let mut scanner = Scanner::new("a\n\t".to_string());
        assert_eq!(scanner.advance(), 'a');
        assert_eq!(scanner.advance(), '\n');
        assert_eq!(scanner.advance(), '\t');
        assert_eq!(scanner.current, 3);
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_advance_at_end() {
        let mut scanner = Scanner::new("a".to_string());
        scanner.advance(); // 消费 'a'
        scanner.advance(); // 应该 panic，因为已经到达末尾
    }

    #[test]
    fn test_match_with_matching_char() {
        let mut scanner = Scanner::new("==".to_string());
        scanner.advance(); // 移动到第一个 '=' 之后
        assert!(scanner.match_('='), "应该匹配第二个 '='");
        assert_eq!(scanner.current, 2);
    }

    #[test]
    fn test_match_with_non_matching_char() {
        let mut scanner = Scanner::new("=!".to_string());
        scanner.advance(); // 移动到 '=' 之后
        assert!(!scanner.match_('='), "不应该匹配 '='");
        assert_eq!(scanner.current, 1); // current 不应该改变
    }

    #[test]
    fn test_match_at_end() {
        let mut scanner = Scanner::new("=".to_string());
        scanner.advance(); // 移动到末尾
        assert!(!scanner.match_('='), "在末尾不应该匹配任何字符");
        assert_eq!(scanner.current, 1);
    }

    #[test]
    fn test_match_with_unicode() {
        let mut scanner = Scanner::new("你好".to_string());
        scanner.advance(); // 移动到 '你' 之后
        assert!(scanner.match_('好'), "应该匹配 '好' 字");
        assert_eq!(scanner.current, 2);
    }

    #[test]
    fn test_match_with_special_chars() {
        let mut scanner = Scanner::new("\n\n".to_string());
        scanner.advance(); // 移动到第一个换行符之后
        assert!(scanner.match_('\n'), "应该匹配第二个换行符");
        assert_eq!(scanner.current, 2);
    }

    #[test]
    fn test_peek_next_empty_source() {
        let scanner = Scanner::new(String::new());
        assert_eq!(scanner.peek_next(), None, "空源代码应该返回 None");
    }

    #[test]
    fn test_peek_next_single_char() {
        let scanner = Scanner::new("a".to_string());
        assert_eq!(scanner.peek_next(), None, "单字符源代码在开始位置应该返回 None");
    }

    #[test]
    fn test_peek_next_multiple_chars() {
        let mut scanner = Scanner::new("hello".to_string());
        assert_eq!(scanner.peek_next(), Some('e'), "应该返回第二个字符");
        
        scanner.current = 3; // 移动到 'l'
        assert_eq!(scanner.peek_next(), Some('o'), "应该返回最后一个字符");
    }

    #[test]
    fn test_peek_next_at_end() {
        let mut scanner = Scanner::new("hi".to_string());
        scanner.current = 1; // 移动到最后一个字符
        assert_eq!(scanner.peek_next(), None, "在最后一个字符时应该返回 None");
    }

    #[test]
    fn test_peek_next_with_unicode() {
        let scanner = Scanner::new("你好世界".to_string());
        assert_eq!(scanner.peek_next(), Some('好'), "应该正确返回下一个 Unicode 字符");
    }

    #[test]
    fn test_peek_next_with_special_chars() {
        let scanner = Scanner::new("a\n\t".to_string());
        assert_eq!(scanner.peek_next(), Some('\n'), "应该正确返回特殊字符");
    }

    // 辅助函数：提取Token类型的向量
    fn get_token_types(tokens: &[Token]) -> Vec<TokenType> {
        tokens.iter().map(|t| t.token_type.clone()).collect()
    }

    #[test]
    fn test_single_character_tokens() {
        let source = "(){},.-+;*";
        let mut scanner = Scanner::new(source.to_string());
        scanner.scan_tokens();
        
        let expected = vec![
            TokenType::LeftParen,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::Comma,
            TokenType::Dot,
            TokenType::Minus,
            TokenType::Plus,
            TokenType::Semicolon,
            TokenType::Star,
        ];
        assert_eq!(get_token_types(&scanner.tokens), expected);
    }

    #[test]
    fn test_double_character_tokens() {
        let mut scanner = Scanner::new("!= == <= >= = < >".to_string());
        scanner.scan_tokens();
        
        let expected = vec![
            TokenType::BangEqual,
            TokenType::EqualEqual,
            TokenType::LessEqual,
            TokenType::GreaterEqual,
            TokenType::Equal,
            TokenType::Less,
            TokenType::Greater,
        ];
        assert_eq!(get_token_types(&scanner.tokens), expected);
    }

    #[test]
    fn test_comment_skipping() {
        let mut scanner = Scanner::new("// comment\n+".to_string());
        scanner.scan_tokens();
        
        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Plus);
        assert_eq!(scanner.tokens[0].line, 2);
    }

    #[test]
    fn test_whitespace_handling() {
        let mut scanner = Scanner::new("  \t\r\n".to_string());
        scanner.scan_tokens();
        
        assert!(scanner.tokens.is_empty());
        assert_eq!(scanner.line, 2);
    }

    #[test]
    fn test_line_counting() {
        let mut scanner = Scanner::new("+\n-\n*".to_string());
        scanner.scan_tokens();
        
        assert_eq!(scanner.tokens[0].line, 1);
        assert_eq!(scanner.tokens[1].line, 2);
        assert_eq!(scanner.tokens[2].line, 3);
    }

    #[test]
    fn test_mixed_operators() {
        let mut scanner = Scanner::new("!===// test".to_string());
        scanner.scan_tokens();
        
        let expected = vec![
            TokenType::BangEqual,
            TokenType::EqualEqual,
        ];
        assert_eq!(get_token_types(&scanner.tokens), expected);
    }

    #[test]
    fn test_unterminated_comment() {
        let mut scanner = Scanner::new("// This comment has no newline".to_string());
        scanner.scan_tokens();
        
        assert!(scanner.tokens.is_empty());
        assert_eq!(scanner.line, 1);
    }

    #[test]
    #[should_panic(expected = "Unexpected character.")]
    fn test_invalid_character() {
        let mut scanner = Scanner::new("~".to_string());
        scanner.scan_tokens();
    }

    #[test]
    fn test_multiline_processing() {
        let mut scanner = Scanner::new("{\n// comment\n}".to_string());
        scanner.scan_tokens();
        
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].line, 1);
        assert_eq!(scanner.tokens[1].line, 3);
    }

    #[test]
    #[should_panic(expected = "Unexpected character.")]
    fn test_number_literal_not_implemented() {
        let mut scanner = Scanner::new("123".to_string());
        scanner.scan_tokens();
    }
}