/// Enumerates all possible token types in the Lox language
/// 
/// # Variants
/// 
/// ## Single Character Tokens
/// * `LeftParen`, `RightParen` - Parentheses
/// * `LeftBrace`, `RightBrace` - Braces
/// * `Comma`, `Dot`, etc. - Basic punctuation
/// 
/// ## One or Two Character Tokens
/// * `Bang`, `BangEqual` - Negation and inequality
/// * `Equal`, `EqualEqual` - Assignment and equality
/// * `Greater`, `GreaterEqual` - Comparison operators
/// 
/// ## Literals
/// * `Identifier` - Variable and function names
/// * `String` - String literals
/// * `Number` - Numeric literals
/// 
/// ## Keywords
/// Standard language keywords like `If`, `While`, `Class`, etc.
#[derive(Debug, Clone)]
pub enum TokenType {
    // single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // one or two character tokens
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,

    // literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If,
    Nil, Or, Print, Return, Super, This,
    True, Var, While,

    Eof,
}