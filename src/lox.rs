use std::{fs, io};
use crate::{scanner::Scanner, Args};

/// The main interpreter for the Lox language
/// 
/// Manages the interpreter state and handles error reporting during execution.
/// This struct serves as the primary entry point for executing Lox programs.
pub struct Lox {
    had_error: bool,
}

impl Lox {
    /// Creates a new instance of the Lox interpreter
    /// 
    /// Initializes a fresh interpreter state with no errors.
    /// 
    /// # Returns
    /// 
    /// A new `Lox` instance with error tracking initialized to `false`
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    /// Executes the Lox interpreter based on command-line arguments
    /// 
    /// # Arguments
    /// 
    /// * `args` - Command-line arguments containing an optional script file path
    /// 
    /// # Returns
    /// 
    /// * `io::Result<()>` - Success if execution completes without IO errors
    /// 
    /// # Behavior
    /// 
    /// - If a script path is provided, executes the file
    /// - Otherwise, starts an interactive REPL session
    pub fn exec(&mut self, args: &Args) -> io::Result<()> {
        match &args.script {
            Some(path) => self.run_file(path),
            None => self.run_prompt(),
        }
    }

    /// Executes a Lox script from a file
    /// 
    /// Reads the entire file content and processes it as Lox source code.
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the Lox script file
    /// 
    /// # Returns
    /// 
    /// * `io::Result<()>` - Success if file operations complete without errors
    /// 
    /// # Notes
    /// 
    /// Exits with code 65 if any errors occur during execution
    fn run_file(&mut self, path: &str) -> io::Result<()> {
        let content = fs::read_to_string(path)?;
        self.run(&content);

        if self.had_error {
            std::process::exit(65);
        }
        Ok(())
    }

    /// Starts an interactive Read-Eval-Print Loop (REPL)
    /// 
    /// Provides a command-line interface for executing Lox code interactively.
    /// Each line is executed independently, and errors don't affect subsequent lines.
    /// 
    /// # Returns
    /// 
    /// * `io::Result<()>` - Success if all IO operations complete without errors
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

    /// Executes a string of Lox source code
    /// 
    /// Processes the source through the scanner and executes the resulting tokens.
    /// Currently prints tokens for debugging purposes.
    /// 
    /// # Arguments
    /// 
    /// * `source` - The Lox source code to execute
    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source.to_string());
        scanner.scan_tokens();

        for token in scanner.tokens {
            println!("{:?}", token);
        }
    }

    /// Reports an error at a specific line in the source
    /// 
    /// # Arguments
    /// 
    /// * `line` - The line number where the error occurred
    /// * `message` - A description of the error
    pub fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    /// Formats and prints an error message to stderr
    /// 
    /// # Arguments
    /// 
    /// * `line` - The line number where the error occurred
    /// * `position` - Additional position information (e.g., column or token)
    /// * `message` - The error message to display
    fn report(line: usize, position: &str, message: &str) {
        eprintln!("[line {}] Error: {}: {}", line, position, message);
    }
}
