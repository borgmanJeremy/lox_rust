use crate::scanner;

enum Expression {
    Assign {
        name: scanner::Token,
        value: Box<Option<Expression>>,
    },
    Binary {
        left: Box<Option<Expression>>,
        operator: scanner::Token,
        right: Box<Option<Expression>>,
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

struct Parser {
    tokens: Vec<scanner::Token>,
    current: usize,
}

impl Parser {
    fn expression() {}

    fn equality() {}

    fn comparison() {}

    // helper
    fn matcher() {}
    fn check() {}
    fn advance() {}
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
            value: Box::new(Some(Expression::Literal {
                value: scanner::Literal::Numeric(1.0),
            })),
        };
    }
}
