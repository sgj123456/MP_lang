use std::{collections::HashMap, error::Error, fmt};

impl Error for InterpreterError {}

#[derive(Debug)]
pub enum InterpreterError {
    UndefinedVariable(String),
    InvalidOperation(String),
    TypeMismatch(String),
    UnsupportedExpression(String),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::UndefinedVariable(name) => write!(f, "未定义变量: {name}"),
            InterpreterError::InvalidOperation(op) => write!(f, "无效操作: {op}"),
            InterpreterError::TypeMismatch(msg) => write!(f, "类型不匹配: {msg}"),
            InterpreterError::UnsupportedExpression(expr) => write!(f, "不支持的表达式: {expr}"),
        }
    }
}

use crate::{
    ast::{Expr, Stmt},
    lexer::Token,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
}

#[derive(Debug, Clone)]
pub enum EnvironmentValue {
    Variable(Value),
    Function {
        params: Vec<String>,
        body: Vec<Stmt>,
    },
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, EnvironmentValue>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, EnvironmentValue::Variable(value));
    }

    pub fn define_function(&mut self, name: String, params: Vec<String>, body: Vec<Stmt>) {
        self.values
            .insert(name, EnvironmentValue::Function { params, body });
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        match self.values.get(name) {
            Some(EnvironmentValue::Variable(value)) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_function(&self, name: &str) -> Option<(Vec<String>, Vec<Stmt>)> {
        match self.values.get(name) {
            Some(EnvironmentValue::Function { params, body }) => {
                Some((params.clone(), body.clone()))
            }
            _ => None,
        }
    }
}

pub fn eval(ast: Vec<Stmt>) -> Result<Value, InterpreterError> {
    let mut env = Environment::new();
    eval_with_env(ast, &mut env)
}

pub fn eval_with_env(ast: Vec<Stmt>, env: &mut Environment) -> Result<Value, InterpreterError> {
    let mut result = Value::Nil;

    for stmt in ast {
        result = eval_stmt(&stmt, env)?;
    }

    Ok(result)
}

fn eval_stmt(stmt: &Stmt, env: &mut Environment) -> Result<Value, InterpreterError> {
    match stmt {
        Stmt::Expr(expr) => eval_expr(expr, env),
        Stmt::Let { name, value } => {
            let value = eval_expr(value, env)?;
            env.define(name.clone(), value);
            Ok(Value::Nil)
        }
        Stmt::Function { name, params, body } => {
            env.define_function(name.clone(), params.clone(), body.clone());
            Ok(Value::Nil)
        }
        _ => Ok(Value::Nil), // 其他不支持语句
    }
}

fn eval_expr(expr: &Expr, env: &mut Environment) -> Result<Value, InterpreterError> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::Boolean(b) => Ok(Value::Boolean(*b)),
        Expr::Variable(name) => match env.get(name.as_str()) {
            Some(value) => Ok(value),
            None => Err(InterpreterError::UndefinedVariable(name.clone())),
        },
        Expr::BinaryOp { left, op, right } => {
            let left_value = eval_expr(left, env)?;
            let right_value = eval_expr(right, env)?;

            match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => match op {
                    Token::Plus => Ok(Value::Number(l + r)),
                    Token::Minus => Ok(Value::Number(l - r)),
                    Token::Multiply => Ok(Value::Number(l * r)),
                    Token::Divide => Ok(Value::Number(l / r)),
                    Token::Keyword(op) if op == ">" => Ok(Value::Boolean(l > r)),
                    Token::Keyword(op) if op == ">=" => Ok(Value::Boolean(l >= r)),
                    Token::Keyword(op) if op == "<" => Ok(Value::Boolean(l < r)),
                    Token::Keyword(op) if op == "<=" => Ok(Value::Boolean(l <= r)),
                    Token::Keyword(op) if op == "==" => Ok(Value::Boolean(l == r)),
                    Token::Keyword(op) if op == "!=" => Ok(Value::Boolean(l != r)),
                    _ => Err(InterpreterError::InvalidOperation(format!("{op:?}"))),
                },
                (Value::Boolean(l), Value::Boolean(r)) => match op {
                    Token::Keyword(op) if op == "==" => Ok(Value::Boolean(l == r)),
                    Token::Keyword(op) if op == "!=" => Ok(Value::Boolean(l != r)),
                    _ => Err(InterpreterError::InvalidOperation(format!("{op:?}"))),
                },
                _ => Err(InterpreterError::TypeMismatch(
                    "操作数类型不匹配".to_string(),
                )),
            }
        }
        Expr::UnaryOp { op, expr } => {
            let value = eval_expr(expr, env)?;
            match (op, value) {
                (Token::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                _ => Err(InterpreterError::InvalidOperation(format!("{op:?}"))),
            }
        }
        Expr::FunctionCall { name, args } => {
            let (params, body) = match env.get_function(name.as_str()) {
                Some(func) => func,
                None => return Err(InterpreterError::UndefinedVariable(name.clone())),
            };

            if params.len() != args.len() {
                return Err(InterpreterError::InvalidOperation(
                    "参数数量不匹配".to_string(),
                ));
            }

            let mut call_env = env.clone();
            for (param, arg) in params.iter().zip(args.iter()) {
                let value = eval_expr(arg, env)?;
                call_env.define(param.clone(), value);
            }

            eval_with_env(body, &mut call_env)
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition_value = eval_expr(condition, env)?;
            if let Value::Boolean(b) = condition_value {
                if b {
                    eval_with_env(then_branch.clone(), env)
                } else if let Some(else_branch) = else_branch {
                    eval_with_env(else_branch.clone(), env)
                } else {
                    Ok(Value::Nil)
                }
            } else {
                Err(InterpreterError::TypeMismatch(
                    "If condition must be boolean".to_string(),
                ))
            }
        }
        Expr::Block(statements) => {
            let mut block_env = env.clone(); // 创建新的作用域
            let mut result = Value::Nil;
            for stmt in statements {
                result = eval_stmt(stmt, &mut block_env)?;
            }
            Ok(result)
        }
        _ => Err(InterpreterError::UnsupportedExpression(format!("{expr:?}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::tokenize, parser::parse};

    #[test]
    fn test_number_eval() {
        let tokens = tokenize("123").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(123.0));
    }

    #[test]
    fn test_binary_op_eval() {
        let tokens = tokenize("1 + 2 * 3").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(7.0));
    }

    #[test]
    fn test_variable_eval() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(5.0));

        let tokens = tokenize("x + 3").unwrap();
        let ast = parse(tokens);
        let result = eval_with_env(ast, &mut env).unwrap();
        assert_eq!(result, Value::Number(8.0));
    }

    #[test]
    fn test_if_expr_eval() {
        let tokens = tokenize("if 1 < 2 3 else 4").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_undefined_variable() {
        let tokens = tokenize("x").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_invalid_operation() {
        let tokens = tokenize("true + 1").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_type_mismatch() {
        let tokens = tokenize("if 1 + true 2 else 3").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_invalid_unary_op() {
        let tokens = tokenize("-true").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_unsupported_expression() {
        let tokens = tokenize("unsupported").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_block_expr_eval() {
        let tokens = tokenize("{ let x = 1; x + 2 }").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_nested_block_scope() {
        let tokens = tokenize("{ let x = 1; { let x = 2; x } }").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(2.0));
    }
}
