use std::collections::HashMap;
// TODO: it would be fun to replace this with a functional iterator based approach
#[derive(Debug, Clone,PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Numeric(f64),
    Text(String),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: i32,
}

fn init_keywords() -> HashMap<String, TokenType> {
    let mut keyword_map = HashMap::new();
    keyword_map.insert(String::from("and"), TokenType::And);
    keyword_map.insert(String::from("class"), TokenType::Class);
    keyword_map.insert(String::from("else"), TokenType::Else);
    keyword_map.insert(String::from("false"), TokenType::False);
    keyword_map.insert(String::from("for"), TokenType::For);
    keyword_map.insert(String::from("fun"), TokenType::Fun);
    keyword_map.insert(String::from("if"), TokenType::If);
    keyword_map.insert(String::from("nil"), TokenType::Nil);
    keyword_map.insert(String::from("or"), TokenType::Or);
    keyword_map.insert(String::from("print"), TokenType::Print);
    keyword_map.insert(String::from("return"), TokenType::Return);
    keyword_map.insert(String::from("super"), TokenType::Super);
    keyword_map.insert(String::from("this"), TokenType::This);
    keyword_map.insert(String::from("true"), TokenType::True);
    keyword_map.insert(String::from("var"), TokenType::Var);
    keyword_map.insert(String::from("while"), TokenType::While);
    return keyword_map;
}

pub struct Scanner {
    // source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: i32,

    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            // source: String::from(contents),
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1,

            keywords: init_keywords(),
        }
    }

    pub fn scan_tokens(&mut self, source: &str) -> Vec<Token> {
        while !self.is_at_end(source) {
            self.start = self.current;
            self.scan_token(source);
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: Literal::Nil,
            line: self.line,
        });
        return self.tokens.clone();
    }

    fn is_at_end(&self, source: &str) -> bool {
        return self.current >= source.len();
    }

    fn scan_token(&mut self, source: &str) {
        let c = self.advance(source);

        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::Nil, source),
            ')' => self.add_token(TokenType::RightParen, Literal::Nil, source),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Nil, source),
            '}' => self.add_token(TokenType::RightBrace, Literal::Nil, source),

            ',' => self.add_token(TokenType::Comma, Literal::Nil, source),
            '.' => self.add_token(TokenType::Dot, Literal::Nil, source),

            '-' => self.add_token(TokenType::Minus, Literal::Nil, source),
            '+' => self.add_token(TokenType::Plus, Literal::Nil, source),
            '*' => self.add_token(TokenType::Star, Literal::Nil, source),

            ';' => self.add_token(TokenType::Semicolon, Literal::Nil, source),

            // Might be 1 or 2 size tokens
            '!' => {
                if self.match_next('=', source) {
                    self.add_token(TokenType::BangEqual, Literal::Nil, source);
                } else {
                    self.add_token(TokenType::Bang, Literal::Nil, source);
                }
            }
            '=' => {
                if self.match_next('=', source) {
                    self.add_token(TokenType::EqualEqual, Literal::Nil, source);
                } else {
                    self.add_token(TokenType::Equal, Literal::Nil, source);
                }
            }
            '<' => {
                if self.match_next('=', source) {
                    self.add_token(TokenType::LessEqual, Literal::Nil, source);
                } else {
                    self.add_token(TokenType::Less, Literal::Nil, source);
                }
            }
            '>' => {
                if self.match_next('=', source) {
                    self.add_token(TokenType::GreaterEqual, Literal::Nil, source);
                } else {
                    self.add_token(TokenType::Greater, Literal::Nil, source);
                }
            }
            '/' => {
                if self.match_next('/', source) {
                    // Comment
                    while self.peek(source) != '\n' && !self.is_at_end(source) {
                        self.advance(source);
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::Nil, source);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            // Literals
            '"' => {
                self.string(source);
            }
            _ => {
                if c.is_digit(10) {
                    self.number(source);
                }

                if c.is_alphabetic() {
                    self.identifier(source);
                }
                // Throw error
            }
        }
    }
    fn identifier(&mut self, source: &str) {
        while is_alphanumeric_underscore(self.peek(source)) {
            self.advance(source);
        }
        let val = match self.keywords.get(&source[self.start..self.current]) {
            Some(val) => val.clone(),
            None => TokenType::Identifier,
        };

        self.add_token(val, Literal::Nil, source);
    }

    fn number(&mut self, source: &str) {
        while self.peek(source).is_digit(10) {
            self.advance(source);
        }
        if self.peek(source) == '.' && self.peek_next(source).is_digit(10) {
            self.advance(source);
            while self.peek(source).is_digit(10) {
                self.advance(source);
            }
        }
        let value = &source[self.start..self.current];
        let num = double_from_string(value);
        self.add_token(TokenType::Number, Literal::Numeric(num), source);
    }

    fn string(&mut self, source: &str) {
        while self.peek(source) != '"' && !self.is_at_end(source) {
            if self.peek(source) == '\n' {
                self.line += 1;
            }
            self.advance(source);
        }

        if self.is_at_end(source) {
            // ERROR unterminated string
            return;
        }
        self.advance(source);
        let value = String::from(&source[self.start + 1..self.current - 1]);
        &self.add_token(TokenType::String, Literal::Text(value), source);
    }

    fn peek(&self, source: &str) -> char {
        if self.is_at_end(source) {
            return '\0';
        } else {
            //TODO: Is this really the best way to index this if I KNOW its ascii?
            return source.as_bytes()[self.current] as char;
        }
    }

    fn peek_next(&self, source: &str) -> char {
        if self.current + 1 >= source.len() {
            return '\0';
        } else {
            //TODO: Is this really the best way to index this if I KNOW its ascii?
            return source.as_bytes()[self.current + 1] as char;
        }
    }
    fn match_next(&mut self, expected: char, source: &str) -> bool {
        if self.is_at_end(source) {
            return false;
        }
        if source.as_bytes()[self.current] as char != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn advance(&mut self, source: &str) -> char {
        self.current += 1;
        return source.as_bytes()[self.current - 1] as char;
    }

    fn add_token(&mut self, ttype: TokenType, literal: Literal, source: &str) {
        let text = &source[self.start..self.current];
        self.tokens.push(Token {
            token_type: ttype,
            lexeme: String::from(text),
            literal: literal,
            line: self.line,
        })
    }
}

fn is_alphanumeric_underscore(c: char) -> bool {
    if c.is_digit(10) || c.is_ascii_alphabetic() || c == '_' {
        return true;
    }
    return false;
}

fn double_from_string(input_str: &str) -> f64 {
    //println!("{}", input_str);
    let val: f64 = input_str.parse().unwrap();
    return val;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_alphanumeric_underscore() {
        assert_eq!(is_alphanumeric_underscore('a'), true);
        assert_eq!(is_alphanumeric_underscore('Z'), true);
        assert_eq!(is_alphanumeric_underscore('1'), true);
        assert_eq!(is_alphanumeric_underscore('0'), true);
        assert_eq!(is_alphanumeric_underscore(','), false);
        assert_eq!(is_alphanumeric_underscore('\0'), false);
        assert_eq!(is_alphanumeric_underscore('\n'), false);
    }

    #[test]
    fn test_double_from_string() {
        assert_eq!(double_from_string("1.234"), 1.234);
        assert_eq!(double_from_string("4"), 4.0);
        assert_eq!(double_from_string("0.5"), 0.5);
    }
}
