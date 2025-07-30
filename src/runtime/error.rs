use std::{error::Error, fmt};

use crate::runtime::environment::value::Value;

impl Error for InterpreterError {}

#[derive(Debug)]
pub enum InterpreterError {
    UndefinedVariable(String),
    InvalidOperation(String),
    TypeMismatch(String),
    #[allow(dead_code)]
    UnsupportedExpression(String),
    Return(Value),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::UndefinedVariable(name) => write!(f, "未定义变量: {name}"),
            InterpreterError::InvalidOperation(op) => write!(f, "无效操作: {op}"),
            InterpreterError::TypeMismatch(msg) => write!(f, "类型不匹配: {msg}"),
            InterpreterError::UnsupportedExpression(expr) => write!(f, "不支持的表达式: {expr}"),
            InterpreterError::Return(value) => write!(f, "函数返回值: {value}"),
        }
    }
}
