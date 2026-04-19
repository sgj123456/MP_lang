use crate::lexer::{Span, Token};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ParserError {
    kind: ParserErrorKind,
    message: String,
    span: Option<Span>,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(span) = self.span {
            write!(f, "Error at {}: {}: {}", span, self.kind, self.message)
        } else {
            write!(f, "{}: {}", self.kind, self.message)
        }
    }
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, message: String) -> Self {
        let span = match &kind {
            ParserErrorKind::UnexpectedToken(token) => Some(token.span),
            _ => None,
        };
        Self { kind, message, span }
    }
}

impl std::error::Error for ParserError {}
