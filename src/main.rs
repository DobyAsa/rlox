#![allow(dead_code)]

mod lox;

use clap::Parser;
use rlox::{lox::Lox, Args};

fn main() {
    let args = Args::parse();
    let mut lox = Lox::new();
    lox.exec(&args).unwrap();
}
