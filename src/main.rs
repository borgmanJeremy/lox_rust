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

enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: i32,
}

struct Interpreter {
    had_error: bool,
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: i32,
    current: i32,
    line: i32,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            source: String::new(),
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scanTokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: "".to_string(),
            line: self.line,
        })
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as i32;
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match (c) {
            '('=> self.add_token(TokenType::LEFT_PAREN, "".to_string()),
            ')'=> self.add_token(TokenType::RIGHT_PAREN, "".to_string()),
            '{'=> self.add_token(TokenType::LEFT_BRACE, "".to_string()),
            '}'=> self.add_token(TokenType::RIGHT_BRACE, "".to_string()),

            ','=> self.add_token(TokenType::COMMA, "".to_string()),
            '.'=> self.add_token(TokenType::DOT, "".to_string()),
            
            '-'=> self.add_token(TokenType::MINUS, "".to_string()),
            '+'=> self.add_token(TokenType::PLUS, "".to_string()),
            '*'=> self.add_token(TokenType::STAR, "".to_string()),
            
            ';'=> self.add_token(TokenType::SEMICOLON, "".to_string()),
            _ => {
                // Throw error
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.source.as_bytes()[self.current as usize - 1] as char;
    }

    fn add_token(&mut self, ttype: TokenType, literal: String) {
       let text= &self.source[self.start as usize .. self.current as usize];
       self.tokens.push(Token {
           token_type: ttype,
           lexeme: String::from(text),
           literal: literal,
           line: self.line
       })
    }
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
    }
}

fn run(contents: &str) {}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // No args (run prompt)
            run_prompt();
        }
        2 => {
            // runfile with arg0
            let contents = read_from_file(Path::new("test.lox"));
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
