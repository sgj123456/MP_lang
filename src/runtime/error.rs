use std::{error::Error, fmt};

use crate::lexer::Span;
use crate::runtime::environment::value::Value;

impl Error for InterpreterError {}

#[derive(Debug)]
pub enum InterpreterError {
    UndefinedVariable(String),
    RedefinedVariable(String),
    InvalidOperation(String),
    TypeMismatch(String),
    UnsupportedExpression(String),
    Return(Value),
    Break,
    Continue,
    WithSpan {
        error: Box<InterpreterError>,
        span: Span,
    },
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::UndefinedVariable(name) => write!(f, "Undefined variable: {name}"),
            InterpreterError::RedefinedVariable(name) => write!(f, "Redefined variable: {name}"),
            InterpreterError::InvalidOperation(op) => write!(f, "Invalid operation: {op}"),
            InterpreterError::TypeMismatch(message) => write!(f, "Type mismatch: {message}"),
            InterpreterError::UnsupportedExpression(expression) => {
                write!(f, "Unsupported expression: {expression}")
            }
            InterpreterError::Return(value) => write!(f, "Function return value: {value}"),
            InterpreterError::Break => write!(f, "Break statement"),
            InterpreterError::Continue => write!(f, "Continue statement"),
            InterpreterError::WithSpan { error, span } => {
                write!(f, "Error at {}: {}", span, error)
            }
        }
    }
}

impl InterpreterError {
    pub fn with_span(self, span: Span) -> Self {
        InterpreterError::WithSpan {
            error: Box::new(self),
            span,
        }
    }
}
