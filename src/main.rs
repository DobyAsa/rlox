#![allow(dead_code)]
mod lox;
mod scanner;
mod token;
use clap::Parser;

use crate::lox::Lox;

#[derive(clap::Parser, Debug)]
pub struct Args {
    pub script: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut lox = Lox::new();
    lox.exec(&args).unwrap();
}
