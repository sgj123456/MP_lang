use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number(f64),
    Boolean(bool),
    String(String),
    Comment(String),
    Comma,
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    LeftParen,
    RightParen,
    #[allow(dead_code)]
    LeftBracket,
    #[allow(dead_code)]
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Newline,
    Identifier(String),
    Let,
    Fn,
    If,
    Else,
    While,
    Return,
    Eof,
}
