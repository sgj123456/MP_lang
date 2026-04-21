use crate::lexer::{Span, Token};

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
    UnexpectedToken(Token),
    UnexpectedEOF,
}

impl std::fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserErrorKind::UnexpectedToken(token) => write!(f, "Unexpected token: {token}"),
            ParserErrorKind::UnexpectedEOF => write!(f, "Unexpected End of File"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub span: Span,
    pub kind: ParserErrorKind,
    pub message: String,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at {}: {}: {}", self.span, self.kind, self.message)
    }
}

impl ParserError {
    pub fn new(span: Span, kind: ParserErrorKind, message: String) -> Self {
        Self {
            span,
            kind,
            message,
        }
    }
    pub fn span(&self) -> Span {
        self.span
    }
    pub fn kind(&self) -> &ParserErrorKind {
        &self.kind
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::error::Error for ParserError {}
