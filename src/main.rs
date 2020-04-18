use lox::parser;
use lox::scanner;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

fn read_from_file(filename: &Path) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(filename.to_str().unwrap());
    match contents {
        Ok(string) => Ok(string),
        Err(e) => Err(e),
    }
}

struct Interpreter {
    had_error: bool,
}

impl Interpreter {
    fn error(&self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&self, line: i32, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
    }
}

fn run_file(contents: &str) {
    run(contents);
}

fn run_prompt() {
    let mut buffer = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut buffer).unwrap();
        run(&buffer);
        buffer.clear();
    }
}

fn run(contents: &str) {
    let mut scan = lox::scanner::Scanner::new();
    let tokens = scan.scan_tokens(contents);
    // println!("{:?}",tokens);
}

fn main() {
    let temp = lox::parser::Parser::new();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // No args (run prompt)
            run_prompt();
        }
        2 => {
            // runfile with arg0
            let contents = read_from_file(Path::new("/home/jeremy/rust/lox/lox/src/test.lox"));
            match contents {
                Ok(code) => run_file(&code),
                Err(e) => eprintln!("Failed to read file: {}, {}", "test.lox", e),
            }
        }
        _ => {
            // keep running prompt
            println!("Usage: jlox [script]");
        }
    }
}
