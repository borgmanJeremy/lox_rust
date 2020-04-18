use crate::scanner;

enum Expression {
    Assign {
        name: scanner::Token,
        value: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: scanner::Token,
        right: Box<Expression>,
    },

    Literal {
        value: scanner::Literal,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Unary {
        operator: scanner::Token,
        right: Box<Expression>,
    },
}

pub struct Parser {
    tokens: Vec<scanner::Token>,
    current: usize,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            tokens: Vec::new(),
            current: 0,
        }
    }
    fn expression(&self) -> Expression {
        return self.equality();
    }

    fn equality(&self) -> Expression {
        let mut expr = self.comparison();

        while self.matcher(&vec![
            scanner::TokenType::BangEqual,
            scanner::TokenType::EqualEqual,
        ]) {
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: self.previous().clone(),
                right: Box::new(self.comparison()),
            }
        }
        return expr;
    }

    fn comparison(&self) -> Expression {
        let mut expr = self.addition();

        while self.matcher(&vec![
            scanner::TokenType::Greater,
            scanner::TokenType::GreaterEqual,
            scanner::TokenType::Less,
            scanner::TokenType::LessEqual,
        ]) {
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: self.previous().clone(),
                right: Box::new(self.addition()),
            }
        }
        return expr;
    }

    fn addition(&self) -> Expression {}
    // helper
    fn matcher(&mut self, ttypes: &Vec<scanner::TokenType>) -> bool {
        for ttype in ttypes.iter() {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, ttype: &scanner::TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == *ttype;
    }

    fn advance(&mut self) -> &scanner::Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    fn is_at_end(&self) -> bool {
        return self.peek().token_type == scanner::TokenType::Eof;
    }
    fn peek(&self) -> &scanner::Token {
        return &self.tokens[self.current];
    }
    fn previous(&self) -> &scanner::Token {
        return &self.tokens[self.current - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build() {
        let x = Expression::Assign {
            name: scanner::Token {
                token_type: scanner::TokenType::Equal,
                lexeme: String::from("="),
                literal: scanner::Literal::Nil,
                line: 1,
            },
            value: Box::new(Expression::Literal {
                value: scanner::Literal::Numeric(1.0),
            }),
        };
    }
}
