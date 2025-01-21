#![allow(dead_code)]

mod lox;

use clap::Parser;
use lox::Lox;

#[derive(clap::Parser, Debug)]
struct Args {
    script: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut lox = Lox::new();

    lox.exec(&args).unwrap();
}
