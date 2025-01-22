#![allow(unused)]

use rlox::lox::Lox;

use clap::Parser;

/// Entry point for the Lox interpreter
/// 
/// Parses command-line arguments and initializes the interpreter.
/// Executes either a script file or starts a REPL session.
fn main() {
    let args = rlox::Args::parse();
    let mut lox = Lox::new();
    lox.exec(&args).unwrap();
}
