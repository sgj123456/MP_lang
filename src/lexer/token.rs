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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.kind, self.span)
    }
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
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Multiply => write!(f, "Multiply"),
            TokenKind::Divide => write!(f, "Divide"),
            TokenKind::Equal => write!(f, "Equal"),
            TokenKind::NotEqual => write!(f, "NotEqual"),
            TokenKind::GreaterThan => write!(f, "GreaterThan"),
            TokenKind::GreaterThanOrEqual => write!(f, "GreaterThanOrEqual"),
            TokenKind::LessThan => write!(f, "LessThan"),
            TokenKind::LessThanOrEqual => write!(f, "LessThanOrEqual"),
            TokenKind::LeftParen => write!(f, "LeftParen"),
            TokenKind::RightParen => write!(f, "RightParen"),
            TokenKind::LeftBracket => write!(f, "LeftBracket"),
            TokenKind::RightBracket => write!(f, "RightBracket"),
            TokenKind::LeftBrace => write!(f, "LeftBrace"),
            TokenKind::RightBrace => write!(f, "RightBrace"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Colon => write!(f, "Colon"),
            TokenKind::Newline => write!(f, "Newline"),
            TokenKind::Identifier(s) => write!(f, "Identifier({s})"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::Fn => write!(f, "Fn"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::While => write!(f, "While"),
            TokenKind::Return => write!(f, "Return"),
            TokenKind::Eof => write!(f, "Eof"),
        }
    }
}
