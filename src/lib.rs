pub mod scanner;
pub mod lox;
pub mod token;

#[derive(clap::Parser, Debug)]
pub struct Args {
    pub script: Option<String>,
}