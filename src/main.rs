#![allow(dead_code)]

mod lox;

use std::{fs, io};
use std::io::BufRead;
use clap::Parser;
use lox::Lox;

#[derive(clap::Parser, Debug)]
struct Args {
    script: String,
}

fn main() {
    let args = Args::parse();

    let lox = Lox::new();

    lox.exec(&args);
}
