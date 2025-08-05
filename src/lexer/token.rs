use std::fmt;

use crate::runtime::environment::value::Number;

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.kind, self.span)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Number(Number),
    Boolean(bool),
    String(String),
    Comment(String),
    Comma,
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Semicolon,
    Colon,
    Newline,
    Identifier(String),
    Let,
    Fn,
    If,
    Else,
    While,
    Break,
    Continue,
    Return,
    Eof,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Number(n) => write!(f, "Number({n})"),
            TokenKind::Boolean(b) => write!(f, "Boolean({b})"),
            TokenKind::String(s) => write!(f, "String({s})"),
            TokenKind::Comment(s) => write!(f, "Comment({s})"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Multiply => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Equal => write!(f, "=="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::GreaterThan => write!(f, ">"),
            TokenKind::GreaterThanOrEqual => write!(f, ">="),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::LessThanOrEqual => write!(f, "<="),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Newline => write!(f, "Newline"),
            TokenKind::Identifier(s) => write!(f, "Identifier({s})"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::Fn => write!(f, "function"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Eof => write!(f, "End of file"),
        }
    }
}
