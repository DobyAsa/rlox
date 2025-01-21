use std::{fs, io};
use crate::Args;
use crate::Scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn exec(&mut self, args: &Args) -> io::Result<()> {
        match &args.script {
            Some(path) => self.run_file(path),
            None => self.run_prompt(),
        }
    }

    fn run_file(&mut self, path: &str) -> io::Result<()> {
        let content = fs::read_to_string(path)?;
        self.run(&content);

        if self.had_error {
            std::process::exit(65);
        }
        Ok(())
    }

    fn run_prompt(&mut self) -> io::Result<()> {
        let reader = io::stdin();

        for line in reader.lines() {
            let line = line?;
            println!("> {}", line);
            self.run(&line);
            self.had_error = false;
        }

        Ok(())
    }

    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source.to_string());
        let tokens = scanner.scan_tokens();

        // 临时打印 tokens 用于调试
        for token in tokens {
            println!("{:?}", token.lexeme);
        }
    }

    pub fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    fn report(line: usize, position: &str, message: &str) {
        eprintln!("[line {}] Error: {}: {}", line, position, message);
    }
}
