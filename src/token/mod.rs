/// Token type definitions
mod types;

pub use types::TokenType;

/// Represents a single token in the Lox language
/// 
/// # Fields
/// 
/// * `token_type` - The category of the token
/// * `lexeme` - The actual text of the token
/// * `line` - The line number where the token appears
#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    /// Creates a new Token instance
    /// 
    /// # Arguments
    /// 
    /// * `token_type` - The type of the token
    /// * `lexeme` - The actual text of the token
    /// * `line` - The line number where the token appears
    /// 
    /// # Returns
    /// 
    /// A new Token with the specified properties
    pub fn new(token_type: TokenType, lexeme: &str, line: usize) -> Token {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            line,
        }
    }
}