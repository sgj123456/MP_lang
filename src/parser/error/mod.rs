use crate::lexer::Token;

#[derive(Debug)]
pub enum ParserErrorKind {
    UnexpectedToken(Token),
    UnexpectedEOF,
    InvalidSyntax,
}
impl std::fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserErrorKind::UnexpectedToken(token) => write!(f, "Unexpected token: {token}"),
            ParserErrorKind::UnexpectedEOF => write!(f, "Unexpected EOF"),
            ParserErrorKind::InvalidSyntax => write!(f, "Invalid syntax"),
        }
    }
}

#[derive(Debug)]
pub struct ParserError {
    kind: ParserErrorKind,
    message: &'static str,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, message: &'static str) -> Self {
        Self { kind, message }
    }
}

impl std::error::Error for ParserError {}
