use std::{error::Error, fmt};

use crate::runtime::environment::value::Value;

impl Error for InterpreterError {}

#[derive(Debug)]
pub enum InterpreterError {
    UndefinedVariable(String),
    InvalidOperation(String),
    TypeMismatch(String),
    UnsupportedExpression(String),
    Return(Value),
    Break,
    Continue,
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::UndefinedVariable(name) => write!(f, "Undefined variable: {name}"),
            InterpreterError::InvalidOperation(op) => write!(f, "Invalid operation: {op}"),
            InterpreterError::TypeMismatch(message) => write!(f, "Type mismatch: {message}"),
            InterpreterError::UnsupportedExpression(expression) => {
                write!(f, "Unsupported expression: {expression}")
            }
            InterpreterError::Return(value) => write!(f, "Function return value: {value}"),
            InterpreterError::Break => write!(f, "Break statement"),
            InterpreterError::Continue => write!(f, "Continue statement"),
        }
    }
}
