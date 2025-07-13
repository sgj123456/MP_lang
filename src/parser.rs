use crate::{
    ast::{Expr, Stmt},
    lexer::Token,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement());
        }
        statements
    }

    fn statement(&mut self) -> Stmt {
        if self.match_token(&Token::Keyword("let".to_string())) {
            self.let_statement()
        } else if self.match_token(&Token::Keyword("if".to_string())) {
            self.if_statement()
        } else {
            Stmt::Expr(self.expression())
        }
    }

    fn let_statement(&mut self) -> Stmt {
        let name = self.consume_identifier();
        self.consume(&Token::Equal, "Expect '=' after variable name");
        let value = self.expression();
        Stmt::Let { name, value }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(&Token::Keyword("==".to_string()))
            || self.match_token(&Token::Keyword("!=".to_string()))
        {
            let op = self.previous().to_owned();
            let right = self.comparison();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token(&Token::Keyword(">".to_string()))
            || self.match_token(&Token::Keyword(">=".to_string()))
            || self.match_token(&Token::Keyword("<".to_string()))
            || self.match_token(&Token::Keyword("<=".to_string()))
        {
            let op = self.previous().to_owned();
            let right = self.term();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(&Token::Plus) || self.match_token(&Token::Minus) {
            let op = self.previous().to_owned();
            let right = self.factor();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(&Token::Multiply) || self.match_token(&Token::Divide) {
            let op = self.previous().to_owned();
            let right = self.unary();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&Token::Minus) {
            let op = self.previous().to_owned();
            let expr = self.unary();
            Expr::UnaryOp {
                op,
                expr: Box::new(expr),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        if !self.is_at_end() {
            match &self.tokens[self.current] {
                Token::Number(n) => {
                    let num = *n;
                    self.advance();
                    return Expr::Number(num);
                }
                Token::Boolean(b) => {
                    let val = *b;
                    self.advance();
                    return Expr::Boolean(val);
                }
                Token::Identifier(name) => {
                    let name = name.clone();
                    self.advance();
                    return Expr::Variable(name);
                }
                _ => {}
            }
        }

        if self.match_token(&Token::LeftParen) {
            let expr = self.expression();
            self.consume(&Token::RightParen, "Expect ')' after expression");
            return expr;
        }

        panic!("Expect expression");
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.tokens[self.current] == token
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.tokens[self.current] == Token::Eof
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token: &Token, message: &str) {
        if self.check(token) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }

    fn if_statement(&mut self) -> Stmt {
        let condition = self.expression();
        self.consume(
            &Token::Keyword("then".to_string()),
            "Expect 'then' after if condition",
        );
        let then_branch = vec![self.statement()];

        let else_branch = if self.match_token(&Token::Keyword("else".to_string())) {
            Some(vec![self.statement()])
        } else {
            None
        };

        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn consume_identifier(&mut self) -> String {
        if let Token::Identifier(name) = self.advance() {
            name.to_owned()
        } else {
            panic!("Expect identifier");
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_number_expr() {
        let tokens = tokenize("123").unwrap();
        let ast = parse(tokens);
        assert_eq!(ast, vec![Stmt::Expr(Expr::Number(123.0))]);
    }

    #[test]
    fn test_binary_op() {
        let tokens = tokenize("1 + 2").unwrap();
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![Stmt::Expr(Expr::BinaryOp {
                left: Box::new(Expr::Number(1.0)),
                op: Token::Plus,
                right: Box::new(Expr::Number(2.0))
            })]
        );
    }

    #[test]
    fn test_variable_decl() {
        let tokens = tokenize("let x = 5").unwrap();
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![Stmt::Let {
                name: "x".to_string(),
                value: Expr::Number(5.0)
            }]
        );
    }

    #[test]
    fn test_if_expr() {
        let tokens = tokenize("if 1 < 2 then 3 else 4").unwrap();
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![Stmt::If {
                condition: Expr::BinaryOp {
                    left: Box::new(Expr::Number(1.0)),
                    op: Token::Keyword("<".to_string()),
                    right: Box::new(Expr::Number(2.0))
                },
                then_branch: vec![Stmt::Expr(Expr::Number(3.0))],
                else_branch: Some(vec![Stmt::Expr(Expr::Number(4.0))])
            }]
        );
    }

    #[test]
    fn test_operator_precedence() {
        let tokens = tokenize("1 + 2 * 3").unwrap();
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![Stmt::Expr(Expr::BinaryOp {
                left: Box::new(Expr::Number(1.0)),
                op: Token::Plus,
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(2.0)),
                    op: Token::Multiply,
                    right: Box::new(Expr::Number(3.0))
                })
            })]
        );
    }
}
