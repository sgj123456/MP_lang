use std::error::Error;
use std::fmt;

use crate::lexer::Span;

impl Error for LexerError {}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub span: Span,
    pub kind: LexerErrorKind,
    pub message: String,
}
impl LexerError {
    pub fn new(span: Span, kind: LexerErrorKind, message: String) -> Self {
        Self {
            span,
            kind,
            message,
        }
    }
    pub fn span(&self) -> Span {
        self.span
    }
    pub fn kind(&self) -> &LexerErrorKind {
        &self.kind
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}
#[derive(Debug, Clone)]
pub enum LexerErrorKind {
    InvalidNumber(String),
    UnexpectedCharacter(char),
    UnclosedString,
    UnclosedComment,
    InvalidEscape(char),
}

impl fmt::Display for LexerErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerErrorKind::InvalidNumber(s) => write!(f, "Invalid number: '{s}'"),
            LexerErrorKind::UnexpectedCharacter(c) => {
                write!(f, "Unexpected character: '{c}'")
            }
            LexerErrorKind::UnclosedString => write!(f, "Unclosed string"),
            LexerErrorKind::UnclosedComment => write!(f, "Unclosed comment"),
            LexerErrorKind::InvalidEscape(c) => {
                write!(f, "Invalid escape sequence: '{c}'")
            }
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}
