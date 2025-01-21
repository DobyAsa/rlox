#![allow(dead_code)]
/// Main module for the Lox interpreter
mod lox;
/// Scanner module for lexical analysis
mod scanner;
/// Token module for lexical tokens
mod token;
use clap::Parser;

use crate::lox::Lox;

/// Command-line argument structure for the Lox interpreter
/// 
/// # Fields
/// 
/// * `script` - Optional path to a Lox source file to execute
#[derive(clap::Parser, Debug)]
pub struct Args {
    pub script: Option<String>,
}

/// Entry point for the Lox interpreter
/// 
/// Parses command-line arguments and initializes the interpreter.
/// Executes either a script file or starts a REPL session.
fn main() {
    let args = Args::parse();
    let mut lox = Lox::new();
    lox.exec(&args).unwrap();
}
