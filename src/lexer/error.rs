use std::error::Error;
use std::fmt;

use crate::lexer::Span;

impl Error for LexerError {}

#[derive(Debug, Clone)]
pub enum LexerError {
    InvalidNumber(String, Span),
    UnexpectedChar(char, Span),
    UnclosedString(Span),
    UnclosedComment(Span),
    InvalidEscape(char, Span),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::InvalidNumber(s, span) => write!(f, "{span}: Invalid number: '{s}'"),
            LexerError::UnexpectedChar(c, span) => write!(f, "{span}: Unexpected character: '{c}'"),
            LexerError::UnclosedString(span) => write!(f, "{span}: Unclosed string"),
            LexerError::UnclosedComment(span) => write!(f, "{span}: Unclosed comment"),
            LexerError::InvalidEscape(c, span) => {
                write!(f, "{span}: Invalid escape sequence: '{c}'")
            }
        }
    }
}
