pub mod ast;
pub mod error;
use crate::{
    lexer::token::{Token, TokenKind},
    parser::{
        ast::{Expr, Stmt},
        error::ParserError,
    },
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = Vec::new();
        self.tokens = self
            .tokens
            .iter()
            .filter(|token| !matches!(token.kind, TokenKind::Comment(_)))
            .cloned()
            .collect();
        while !self.is_at_end() {
            let stmt = self.statement()?;
            statements.push(stmt);
        }
        Ok(statements)
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
    fn statement(&mut self) -> Result<Stmt, ParserError> {
        self.delete_empty_statements();
        let stmt = if self.match_token(&TokenKind::Let) {
            self.let_statement()?
        } else if self.match_token(&TokenKind::Fn) {
            self.function_statement()?
        } else if self.match_token(&TokenKind::Return) {
            let value = if !self.check(&TokenKind::Semicolon) && !self.check(&TokenKind::Newline) {
                Some(self.expression()?)
            } else {
                None
            };
            Stmt::Return(value)
        } else {
            let expr = self.expression()?;
            if self.check(&TokenKind::Semicolon)
                || (self.check(&TokenKind::Newline)
                    && !self.is_at_block_last_not_empty_line()
                    && !self.is_at_last_not_empty_line())
            {
                Stmt::Expr(expr)
            } else if self.is_at_last_not_empty_line() || self.is_at_block_last_not_empty_line() {
                Stmt::Result(expr)
            } else {
                return Err(ParserError::new(
                    error::ParserErrorKind::UnexpectedToken(self.current().clone()),
                    "Unexpected token: {:?}. Expected a statement.",
                ));
            }
        };
        if !self.match_token(&TokenKind::Semicolon)
            && !self.match_token(&TokenKind::Newline)
            && !self.is_at_last_not_empty_line()
            && !self.is_at_block_last_not_empty_line()
            && !matches!(stmt, Stmt::Expr(_) | Stmt::Result(_))
        {
            panic!("Unexpected token: {:?}", self.current())
        }
        self.delete_empty_statements();
        Ok(stmt)
    }

    fn while_expression(&mut self) -> Result<Expr, ParserError> {
        let condition = self.expression()?;
        self.consume(&TokenKind::LeftBrace, "Expect '{' after while condition")?;

        let mut body = Vec::new();
        loop {
            if self.check(&TokenKind::RightBrace) || self.is_at_end() {
                break;
            }

            body.push(self.statement()?);
        }

        self.consume(&TokenKind::RightBrace, "Expect '}' after while body")?;
        Ok(Expr::While {
            condition: Box::new(condition),
            body,
        })
    }

    fn let_statement(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume_identifier()?;
        self.consume(&TokenKind::Equal, "Expect '=' after variable name")?;
        let value = self.expression()?;
        Ok(Stmt::Let { name, value })
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(&TokenKind::If) {
            self.if_expression()
        } else if self.match_token(&TokenKind::While) {
            self.while_expression()
        } else {
            self.assignment()
        }
    }

    fn assignment(&mut self) -> Result<Expr, ParserError> {
        let expr = self.equality()?;

        if self.match_token(&TokenKind::Equal) {
            let value = self.assignment()?;
            if let Expr::Variable(name) = expr {
                return Ok(Expr::BinaryOp {
                    left: Box::new(Expr::Variable(name)),
                    op: TokenKind::Equal,
                    right: Box::new(value),
                });
            }
            panic!("Invalid assignment target");
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while self.match_token(&TokenKind::Equal) || self.match_token(&TokenKind::NotEqual) {
            let op = self.previous().to_owned().kind;
            let right = self.comparison()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.match_token(&TokenKind::GreaterThan)
            || self.match_token(&TokenKind::GreaterThanOrEqual)
            || self.match_token(&TokenKind::LessThan)
            || self.match_token(&TokenKind::LessThanOrEqual)
        {
            let op = self.previous().to_owned().kind;
            let right = self.term()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.match_token(&TokenKind::Plus) || self.match_token(&TokenKind::Minus) {
            let op = self.previous().to_owned().kind;
            let right = self.factor()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.match_token(&TokenKind::Multiply) || self.match_token(&TokenKind::Divide) {
            let op = self.previous().to_owned().kind;
            let right = self.unary()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.match_token(&TokenKind::Minus) {
            let op = self.previous().to_owned().kind;
            let expr = self.unary()?;
            Ok(Expr::UnaryOp {
                op,
                expr: Box::new(expr),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.is_at_end() {
            return Err(ParserError::new(
                error::ParserErrorKind::UnexpectedEOF,
                "Unexpected end of file. Expected expression.",
            ));
        }
        let expr = match &self.current().kind {
            TokenKind::Number(n) => {
                let num = *n;
                self.advance();
                Expr::Number(num)
            }
            TokenKind::Boolean(b) => {
                let val = *b;
                self.advance();
                Expr::Boolean(val)
            }
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                Expr::String(s)
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();

                if self.match_token(&TokenKind::LeftParen) {
                    let mut args = Vec::new();
                    if !self.match_token(&TokenKind::RightParen) {
                        loop {
                            args.push(self.expression()?);
                            if !self.match_token(&TokenKind::Comma) {
                                break;
                            }
                        }
                        self.consume(&TokenKind::RightParen, "Expect ')' after arguments")?;
                    }
                    return Ok(Expr::FunctionCall { name, args });
                }
                Expr::Variable(name)
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(&TokenKind::RightParen, "Expect ')' after expression")?;
                expr
            }
            TokenKind::LeftBrace => {
                self.advance();
                let mut statements = Vec::new();
                while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                    statements.push(self.statement()?);
                }
                self.consume(&TokenKind::RightBrace, "Expect '}' after block")?;
                Expr::Block(statements)
            }
            _ => {
                let token = self.current();
                panic!(
                    "Unexpected token {:?} at {}:{}",
                    token.kind, token.span.line, token.span.column
                );
            }
        };
        Ok(expr)
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.current().kind == kind
        }
    }

    fn consume(&mut self, kind: &TokenKind, message: &'static str) -> Result<(), ParserError> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            let token = self.current();
            Err(ParserError::new(
                error::ParserErrorKind::UnexpectedToken(token.clone()),
                message,
            ))
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

    fn if_expression(&mut self) -> Result<Expr, ParserError> {
        let condition = Box::new(self.expression()?);
        let then_branch = Box::new(self.expression()?);

        let else_branch = if self.match_token(&TokenKind::Else) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };

        Ok(Expr::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn function_statement(&mut self) -> Result<Stmt, ParserError> {
        let name = self.consume_identifier()?;
        self.consume(&TokenKind::LeftParen, "Expect '(' after function name")?;

        let mut params = Vec::new();
        if !self.match_token(&TokenKind::RightParen) {
            loop {
                params.push(self.consume_identifier()?);
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
            self.consume(&TokenKind::RightParen, "Expect ')' after parameters")?;
        }

        let body = self.expression()?;

        Ok(Stmt::Function { name, params, body })
    }

    fn consume_identifier(&mut self) -> Result<String, ParserError> {
        if let TokenKind::Identifier(name) = &self.advance().kind {
            Ok(name.to_owned())
        } else {
            Err(ParserError::new(
                error::ParserErrorKind::UnexpectedToken(self.current().clone()),
                "Expect identifier",
            ))
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, ParserError> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
