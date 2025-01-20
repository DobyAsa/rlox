use std::{fs, io};
use crate::Args;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            had_error: false
        }
    }

    pub fn exec(&self, args: &Args) {

    }
    fn run_file(&mut self, path: &str) -> io::Result<()> {
        let bytes = fs::read(path)?;

        for line in std::str::from_utf8(&bytes).unwrap().lines() {
            let line = line.trim();
            self.run(line);
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

    fn run(&mut self, input: &str) {
        println!("Running: {}", input);
    }

    fn error(&self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&self, line: i32, position: &str, message: &str) {
        eprintln!("[line {}] Error: {}: {}", line, position, message);
    }
}