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
            let stmt = self.statement();
            statements.push(stmt);
        }
        statements
    }

    fn statement(&mut self) -> Stmt {
        while self.match_token(&Token::Semicolon) || self.match_token(&Token::Newline) {}
        let stmt = if self.match_token(&Token::Let) {
            self.let_statement()
        } else if self.match_token(&Token::Fn) {
            self.function_statement()
        } else {
            let expr = self.expression();
            if !self.check(&Token::Semicolon) && self.check(&Token::RightBrace) {
                Stmt::Result(expr)
            } else {
                Stmt::Expr(expr)
            }
        };
        if !self.is_at_end()
            && !self.is_at_block_end()
            && !self.check(&Token::Semicolon)
            && !self.check(&Token::Newline)
        {
            panic!("Expect ';' or newline after expression");
        }
        while self.match_token(&Token::Semicolon) || self.match_token(&Token::Newline) {}
        stmt
    }

    fn while_expression(&mut self) -> Expr {
        let condition = self.expression();
        self.consume(&Token::LeftBrace, "Expect '{' after while condition");

        let mut body = Vec::new();
        loop {
            if self.check(&Token::RightBrace) || self.is_at_end() {
                break;
            }

            // 解析语句并添加到循环体
            body.push(self.statement());
        }

        self.consume(&Token::RightBrace, "Expect '}' after while body");
        Expr::While {
            condition: Box::new(condition),
            body,
        }
    }

    fn let_statement(&mut self) -> Stmt {
        let name = self.consume_identifier();
        self.consume(&Token::Equal, "Expect '=' after variable name");
        let value = self.expression();
        Stmt::Let { name, value }
    }

    fn expression(&mut self) -> Expr {
        let expr = if self.match_token(&Token::If) {
            self.if_expression()
        } else if self.match_token(&Token::While) {
            self.while_expression()
        } else {
            self.assignment()
        };
        expr
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.equality();

        if self.match_token(&Token::Equal) {
            let value = self.assignment();
            if let Expr::Variable(name) = expr {
                return Expr::BinaryOp {
                    left: Box::new(Expr::Variable(name)),
                    op: Token::Equal,
                    right: Box::new(value),
                };
            }
            panic!("Invalid assignment target");
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(&Token::Equal) || self.match_token(&Token::NotEqual) {
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

        while self.match_token(&Token::GreaterThan)
            || self.match_token(&Token::GreaterThanOrEqual)
            || self.match_token(&Token::LessThan)
            || self.match_token(&Token::LessThanOrEqual)
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
        if let Token::Identifier(name) = &self.tokens[self.current] {
            let name = name.clone();
            self.advance();

            if self.match_token(&Token::LeftParen) {
                let mut args = Vec::new();
                if !self.match_token(&Token::RightParen) {
                    loop {
                        args.push(self.expression());
                        if !self.match_token(&Token::Comma) {
                            break;
                        }
                    }
                    self.consume(&Token::RightParen, "Expect ')' after arguments");
                }
                return Expr::FunctionCall { name, args };
            }
            return Expr::Variable(name);
        }

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

        if self.match_token(&Token::LeftBrace) {
            let mut statements = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                statements.push(self.statement());
            }
            self.consume(&Token::RightBrace, "Expect '}' after block");
            return Expr::Block(statements);
        }

        let current_token = if self.is_at_end() {
            "end of input".to_string()
        } else {
            format!("{:?}", self.tokens[self.current])
        };
        panic!("Expect expression, found {current_token}");
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

    fn is_at_block_end(&self) -> bool {
        self.check(&Token::RightBrace)
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

    fn if_expression(&mut self) -> Expr {
        let condition = Box::new(self.expression());
        let then_branch = vec![self.statement()];

        let else_branch = if self.match_token(&Token::Else) {
            Some(vec![self.statement()])
        } else {
            None
        };

        Expr::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn function_statement(&mut self) -> Stmt {
        let name = self.consume_identifier();
        self.consume(&Token::LeftParen, "Expect '(' after function name");

        let mut params = Vec::new();
        if !self.match_token(&Token::RightParen) {
            loop {
                params.push(self.consume_identifier());
                if !self.match_token(&Token::Comma) {
                    break;
                }
            }
            self.consume(&Token::RightParen, "Expect ')' after parameters");
        }

        self.consume(&Token::LeftBrace, "Expect '{' before function body");
        let mut body = Vec::new();
        while !self.match_token(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.statement());
        }

        Stmt::Function { name, params, body }
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
        let tokens = tokenize("if 1 < 2 {3}; else {4};").unwrap();
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![Stmt::Expr(Expr::If {
                condition: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Number(1.0)),
                    op: Token::LessThan,
                    right: Box::new(Expr::Number(2.0))
                }),
                then_branch: vec![Stmt::Expr(Expr::Block(vec![Stmt::Result(Expr::Number(
                    3.0
                ))]))],
                else_branch: Some(vec![Stmt::Expr(Expr::Block(vec![Stmt::Result(
                    Expr::Number(4.0)
                )]))])
            })]
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

    #[test]
    fn test_function_decl() {
        let tokens = tokenize("fn add(a, b) { a + b }").unwrap();
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![Stmt::Function {
                name: "add".to_string(),
                params: vec!["a".to_string(), "b".to_string()],
                body: vec![Stmt::Result(Expr::BinaryOp {
                    left: Box::new(Expr::Variable("a".to_string())),
                    op: Token::Plus,
                    right: Box::new(Expr::Variable("b".to_string()))
                })]
            }]
        );
    }

    #[test]
    fn test_function_call() {
        let tokens = tokenize("add(1, 2)").unwrap();
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![Stmt::Expr(Expr::FunctionCall {
                name: "add".to_string(),
                args: vec![Expr::Number(1.0), Expr::Number(2.0)]
            })]
        );
    }

    #[test]
    fn test_nested_function_call() {
        let tokens = tokenize("add(1, mul(2, 3))").unwrap();
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![Stmt::Expr(Expr::FunctionCall {
                name: "add".to_string(),
                args: vec![
                    Expr::Number(1.0),
                    Expr::FunctionCall {
                        name: "mul".to_string(),
                        args: vec![Expr::Number(2.0), Expr::Number(3.0)]
                    }
                ]
            })]
        );
    }

    #[test]
    fn test_semicolon_separator() {
        let tokens = tokenize("let x = 1; let y = 2").unwrap();
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![
                Stmt::Let {
                    name: "x".to_string(),
                    value: Expr::Number(1.0)
                },
                Stmt::Let {
                    name: "y".to_string(),
                    value: Expr::Number(2.0)
                }
            ]
        );
    }

    #[test]
    fn test_multiple_semicolons() {
        let tokens = tokenize("let x = 1;;; let y = 2").unwrap();
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![
                Stmt::Let {
                    name: "x".to_string(),
                    value: Expr::Number(1.0)
                },
                Stmt::Let {
                    name: "y".to_string(),
                    value: Expr::Number(2.0)
                }
            ]
        );
    }

    #[test]
    fn test_semicolon_after_expr() {
        let tokens = tokenize("1 + 2; 3 * 4").unwrap();
        let ast = parse(tokens);
        assert_eq!(
            ast,
            vec![
                Stmt::Expr(Expr::BinaryOp {
                    left: Box::new(Expr::Number(1.0)),
                    op: Token::Plus,
                    right: Box::new(Expr::Number(2.0))
                }),
                Stmt::Expr(Expr::BinaryOp {
                    left: Box::new(Expr::Number(3.0)),
                    op: Token::Multiply,
                    right: Box::new(Expr::Number(4.0))
                })
            ]
        );
    }
}
