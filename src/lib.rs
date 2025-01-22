/// Main module for the Lox interpreter
pub mod lox;
/// Scanner module for lexical analysis
mod scanner;
/// Token module for lexical tokens
mod token;

/// Command-line argument structure for the Lox interpreter
/// 
/// # Fields
/// 
/// * `script` - Optional path to a Lox source file to execute
#[derive(clap::Parser, Debug)]
pub struct Args {
    pub script: Option<String>,
}