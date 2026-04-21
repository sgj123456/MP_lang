mod ast;
mod error;

pub use ast::{Expr, ExprKind, Stmt, StmtKind};

use crate::runtime::environment::value::Number;
use crate::{
    lexer::{Token, TokenKind},
    parser::error::ParserError,
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<ParserError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn get_errors(&self) -> &[ParserError] {
        &self.errors
    }

    fn report_error(&mut self, error: ParserError) {
        self.errors.push(error);
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.current + 1)
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        self.tokens = self
            .tokens
            .iter()
            .filter(|token| !matches!(token.kind, TokenKind::Comment(_)))
            .cloned()
            .collect();
        while !self.is_at_end() {
            statements.push(self.statement());
        }
        statements
    }
    fn delete_empty_lines(&mut self) {
        self.delete_continuous_tokens(&TokenKind::Newline);
    }
    fn delete_empty_statements(&mut self) {
        self.delete_continuous_tokens(&TokenKind::Semicolon);
        self.delete_continuous_tokens(&TokenKind::Newline);
    }
    fn delete_continuous_tokens(&mut self, kind: &TokenKind) {
        while self.match_token(kind) {}
    }
    fn statement(&mut self) -> Stmt {
        self.delete_empty_statements();
        let stmt = if self.match_token(&TokenKind::Let) {
            self.let_statement()
        } else if self.match_token(&TokenKind::Fn) {
            self.function_statement()
        } else if self.match_token(&TokenKind::Struct) {
            self.struct_statement()
        } else if self.match_token(&TokenKind::Continue) {
            Stmt {
                kind: StmtKind::Continue,
                span: self.previous().span,
            }
        } else if self.match_token(&TokenKind::Break) {
            Stmt {
                kind: StmtKind::Break,
                span: self.previous().span,
            }
        } else if self.match_token(&TokenKind::Return) {
            let value = if !self.check(&TokenKind::Semicolon) && !self.check(&TokenKind::Newline) {
                Some(self.expression())
            } else {
                None
            };
            Stmt {
                kind: StmtKind::Return(value),
                span: self.previous().span,
            }
        } else {
            let expr = self.expression();
            if self.check(&TokenKind::Semicolon)
                || (self.check(&TokenKind::Newline)
                    && !self.is_at_block_last_not_empty_line()
                    && !self.is_at_last_not_empty_line())
            {
                Stmt {
                    kind: StmtKind::Expr(expr),
                    span: self.previous().span,
                }
            } else if self.is_at_last_not_empty_line() || self.is_at_block_last_not_empty_line() {
                Stmt {
                    kind: StmtKind::Result(expr),
                    span: self.previous().span,
                }
            } else {
                self.report_error(ParserError::new(
                    self.peek().span,
                    error::ParserErrorKind::UnexpectedToken(self.peek().clone()),
                    "Unexpected token. Expected a statement.".into(),
                ));
                Stmt {
                    kind: StmtKind::Expr(expr),
                    span: self.previous().span,
                }
            }
        };
        if !self.match_token(&TokenKind::Semicolon)
            && !self.match_token(&TokenKind::Newline)
            && !self.is_at_last_not_empty_line()
            && !self.is_at_block_last_not_empty_line()
            && !matches!(stmt.kind, StmtKind::Expr(_) | StmtKind::Result(_))
        {
            self.report_error(ParserError::new(
                self.peek().span,
                error::ParserErrorKind::UnexpectedToken(self.peek().clone()),
                "Unexpected token. Expected ';' or newline".into(),
            ));
        }
        self.delete_empty_statements();
        stmt
    }

    fn while_expression(&mut self) -> Expr {
        let condition = self.expression();
        let body = self.expression();
        Expr {
            kind: ExprKind::While {
                condition: Box::new(condition),
                body: Box::new(body),
            },
            span: self.previous().span,
        }
    }

    fn let_statement(&mut self) -> Stmt {
        let name = self.consume_identifier();
        let name_span = self.previous().span;
        self.consume(&TokenKind::Assign, "Expect '=' after variable name");
        let value = self.expression();
        Stmt {
            kind: StmtKind::Let {
                name,
                name_span,
                value,
            },
            span: self.previous().span,
        }
    }

    fn expression(&mut self) -> Expr {
        if self.match_token(&TokenKind::If) {
            self.if_expression()
        } else if self.match_token(&TokenKind::While) {
            self.while_expression()
        } else {
            self.assignment()
        }
    }

    fn assignment(&mut self) -> Expr {
        let expr = self.equality();

        if self.match_token(&TokenKind::Assign) {
            let value = self.assignment();
            match expr.kind.clone() {
                ExprKind::Variable(_) | ExprKind::Index { .. } => {
                    return Expr {
                        kind: ExprKind::BinaryOp {
                            left: Box::new(expr),
                            op: TokenKind::Assign,
                            right: Box::new(value),
                        },
                        span: self.previous().span,
                    };
                }
                _ => {
                    self.report_error(ParserError::new(
                        self.previous().span,
                        error::ParserErrorKind::UnexpectedToken(self.previous().clone()),
                        "Invalid assignment target: expected a variable name".into(),
                    ));
                }
            }
        }

        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_token(&TokenKind::Equal) || self.match_token(&TokenKind::NotEqual) {
            let op = self.previous().to_owned().kind;
            let right = self.comparison();
            expr = Expr {
                kind: ExprKind::BinaryOp {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                },
                span: self.previous().span,
            };
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.logical_or();

        while self.match_token(&TokenKind::GreaterThan)
            || self.match_token(&TokenKind::GreaterThanOrEqual)
            || self.match_token(&TokenKind::LessThan)
            || self.match_token(&TokenKind::LessThanOrEqual)
        {
            let op = self.previous().to_owned().kind;
            let right = self.logical_or();
            expr = Expr {
                kind: ExprKind::BinaryOp {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                },
                span: self.previous().span,
            };
        }

        expr
    }

    fn logical_or(&mut self) -> Expr {
        let mut expr = self.logical_and();

        while self.match_token(&TokenKind::LogicalOr) {
            let op = self.previous().to_owned().kind;
            let right = self.logical_and();
            expr = Expr {
                kind: ExprKind::BinaryOp {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                },
                span: self.previous().span,
            };
        }

        expr
    }

    fn logical_and(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_token(&TokenKind::LogicalAnd) {
            let op = self.previous().to_owned().kind;
            let right = self.term();
            expr = Expr {
                kind: ExprKind::BinaryOp {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                },
                span: self.previous().span,
            };
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_token(&TokenKind::Plus) || self.match_token(&TokenKind::Minus) {
            let op = self.previous().to_owned().kind;
            let right = self.factor();
            expr = Expr {
                kind: ExprKind::BinaryOp {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                },
                span: self.previous().span,
            };
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_token(&TokenKind::Multiply)
            || self.match_token(&TokenKind::Divide)
            || self.match_token(&TokenKind::Modulo)
        {
            let op = self.previous().to_owned().kind;
            let right = self.unary();
            expr = Expr {
                kind: ExprKind::BinaryOp {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                },
                span: self.previous().span,
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(&TokenKind::Minus) || self.match_token(&TokenKind::Not) {
            let op = self.previous().to_owned().kind;
            let expr = self.unary();
            return Expr {
                kind: ExprKind::UnaryOp {
                    op,
                    expr: Box::new(expr),
                },
                span: self.previous().span,
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.is_at_end() {
            self.report_error(ParserError::new(
                self.previous().span,
                error::ParserErrorKind::UnexpectedEOF,
                "Unexpected end of file. Expected expression.".into(),
            ));
            return Expr {
                kind: ExprKind::Number(Number::Float(0.0)),
                span: self.previous().span,
            };
        }
        let expr = match &self.peek().kind {
            TokenKind::Number(n) => {
                let num = n.clone();
                self.advance();
                Expr {
                    kind: ExprKind::Number(num),
                    span: self.previous().span,
                }
            }
            TokenKind::Boolean(b) => {
                let val = *b;
                self.advance();
                Expr {
                    kind: ExprKind::Boolean(val),
                    span: self.previous().span,
                }
            }
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                Expr {
                    kind: ExprKind::String(s),
                    span: self.previous().span,
                }
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();

                if self.match_token(&TokenKind::LeftParen) {
                    let mut args = Vec::new();
                    if !self.match_token(&TokenKind::RightParen) {
                        loop {
                            args.push(self.expression());
                            if !self.match_token(&TokenKind::Comma) {
                                break;
                            }
                        }
                        self.consume(&TokenKind::RightParen, "Expect ')' after arguments");
                    }
                    return Expr {
                        kind: ExprKind::FunctionCall { name, args },
                        span: self.previous().span,
                    };
                }
                Expr {
                    kind: ExprKind::Variable(name),
                    span: self.previous().span,
                }
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.expression();
                self.consume(&TokenKind::RightParen, "Expect ')' after expression");
                Expr {
                    kind: ExprKind::Parenthesized(Box::new(expr)),
                    span: self.previous().span,
                }
            }
            TokenKind::LeftBrace => {
                self.advance();
                self.delete_empty_lines();
                let is_object = if let TokenKind::String(_) = &self.peek().kind {
                    matches!(
                        self.peek_next(),
                        Some(Token {
                            kind: TokenKind::Colon,
                            ..
                        })
                    )
                } else {
                    false
                };

                if is_object {
                    let mut properties = Vec::new();
                    while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                        self.delete_empty_lines();
                        let key = if let TokenKind::String(name) = &self.peek().kind {
                            name.clone()
                        } else {
                            self.report_error(ParserError::new(
                                self.peek().span,
                                error::ParserErrorKind::UnexpectedToken(self.peek().clone()),
                                "Expect property name".into(),
                            ));
                            "".to_string()
                        };
                        self.advance();
                        self.consume(&TokenKind::Colon, "Expect ':' after property name");
                        let value = self.expression();
                        properties.push((key, value));

                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                        self.delete_empty_lines();
                    }
                    self.delete_empty_lines();
                    self.consume(&TokenKind::RightBrace, "Expect '}' after object properties");
                    return Expr {
                        kind: ExprKind::Object(properties),
                        span: self.previous().span,
                    };
                }

                let mut statements = Vec::new();
                while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                    statements.push(self.statement().kind);
                }
                self.consume(&TokenKind::RightBrace, "Expect '}' after block");
                Expr {
                    kind: ExprKind::Block(statements),
                    span: self.previous().span,
                }
            }
            TokenKind::LeftBracket => {
                self.advance();
                self.delete_empty_lines();
                let mut elements = Vec::new();
                while !self.check(&TokenKind::RightBracket) && !self.is_at_end() {
                    elements.push(self.expression());
                    self.delete_empty_lines();
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                    self.delete_empty_lines();
                }
                self.consume(&TokenKind::RightBracket, "Expect ']' after array elements");
                Expr {
                    kind: ExprKind::Array(elements),
                    span: self.previous().span,
                }
            }

            _ => {
                let token = self.peek();
                let span = token.span;
                self.report_error(ParserError::new(
                    span,
                    error::ParserErrorKind::UnexpectedToken(token.clone()),
                    "in primary parser".into(),
                ));
                Expr {
                    kind: ExprKind::Number(Number::Float(0.0)),
                    span,
                }
            }
        };

        self.postfix_expression(expr)
    }

    fn postfix_expression(&mut self, mut expr: Expr) -> Expr {
        loop {
            if self.match_token(&TokenKind::LeftBracket) {
                let index = self.expression();
                self.consume(&TokenKind::RightBracket, "Expect ']' after index");
                expr = Expr {
                    kind: ExprKind::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    },
                    span: self.previous().span,
                };
            } else if self.match_token(&TokenKind::Colon) {
                if let TokenKind::Identifier(property) = &self.peek().kind {
                    let prop_name = property.clone();
                    self.advance();
                    expr = Expr {
                        kind: ExprKind::GetProperty {
                            object: Box::new(expr),
                            property: prop_name,
                        },
                        span: self.previous().span,
                    };
                } else {
                    self.report_error(ParserError::new(
                        self.peek().span,
                        error::ParserErrorKind::UnexpectedToken(self.peek().clone()),
                        "Expect property name after ':'".into(),
                    ));
                }
            } else {
                break;
            }
        }
        expr
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().kind == kind
        }
    }

    fn consume(&mut self, kind: &TokenKind, message: &'static str) {
        if self.check(kind) {
            self.advance();
        } else {
            let token = self.peek();
            self.report_error(ParserError::new(
                token.span,
                error::ParserErrorKind::UnexpectedToken(token.clone()),
                message.into(),
            ));
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.tokens[self.current].kind == TokenKind::Eof
    }

    fn is_at_last_not_empty_line(&mut self) -> bool {
        self.delete_empty_lines();
        self.is_at_end()
    }
    fn is_at_block_last_not_empty_line(&mut self) -> bool {
        self.delete_empty_lines();
        self.check(&TokenKind::RightBrace)
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn if_expression(&mut self) -> Expr {
        let condition = Box::new(self.expression());
        let then_branch = Box::new(self.expression());

        let else_branch = if self.match_token(&TokenKind::Else) {
            Some(Box::new(self.expression()))
        } else {
            None
        };

        Expr {
            kind: ExprKind::If {
                condition,
                then_branch,
                else_branch,
            },
            span: self.previous().span,
        }
    }

    fn function_statement(&mut self) -> Stmt {
        let name = self.consume_identifier();
        self.consume(&TokenKind::LeftParen, "Expect '(' after function name");

        let mut params = Vec::new();
        if !self.match_token(&TokenKind::RightParen) {
            loop {
                params.push(self.consume_identifier());
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
            self.consume(&TokenKind::RightParen, "Expect ')' after parameters");
        }

        let body = self.expression();

        Stmt {
            kind: StmtKind::Function { name, params, body },
            span: self.previous().span,
        }
    }

    fn struct_statement(&mut self) -> Stmt {
        let name = self.consume_identifier();
        self.consume(&TokenKind::LeftBrace, "Expect '{' after struct name");

        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.delete_empty_lines();
            let field_name = self.consume_identifier();
            self.delete_empty_lines();
            let default_value = if self.match_token(&TokenKind::Assign) {
                let expr = Some(self.expression());
                self.delete_empty_lines();
                expr
            } else {
                None
            };
            fields.push((field_name, default_value));

            if !self.match_token(&TokenKind::Comma) {
                break;
            }
            self.delete_empty_lines();
        }

        self.consume(&TokenKind::RightBrace, "Expect '}' after struct fields");

        Stmt {
            kind: StmtKind::Struct { name, fields },
            span: self.previous().span,
        }
    }

    fn consume_identifier(&mut self) -> String {
        if let TokenKind::Identifier(name) = &self.advance().kind {
            name.to_owned()
        } else {
            self.report_error(ParserError::new(
                self.peek().span,
                error::ParserErrorKind::UnexpectedToken(self.peek().clone()),
                "Expect identifier".into(),
            ));
            "".to_owned()
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

pub fn parse_with_errors(tokens: Vec<Token>) -> (Vec<Stmt>, Vec<ParserError>) {
    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();
    (stmts, parser.get_errors().to_vec())
}
