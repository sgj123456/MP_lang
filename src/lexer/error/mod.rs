use std::error::Error;
use std::fmt;

use crate::lexer::Span;

impl Error for LexerError {}

#[derive(Debug, Clone)]
pub enum LexerError {
    UnknownOperator(char, Span),
    InvalidNumber(String, Span),
    UnexpectedChar(char, Span),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::UnknownOperator(c, span) => write!(f, "{span}: Unknown operator: '{c}'"),
            LexerError::InvalidNumber(s, span) => write!(f, "{span}: Invalid number: '{s}'"),
            LexerError::UnexpectedChar(c, span) => write!(f, "{span}: Unexpected character: '{c}'"),
        }
    }
}
