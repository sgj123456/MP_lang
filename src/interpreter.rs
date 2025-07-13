// 解释器

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
            InterpreterError::UndefinedVariable(name) => write!(f, "未定义变量: {}", name),
            InterpreterError::InvalidOperation(op) => write!(f, "无效操作: {}", op),
            InterpreterError::TypeMismatch(msg) => write!(f, "类型不匹配: {}", msg),
            InterpreterError::UnsupportedExpression(expr) => write!(f, "不支持的表达式: {}", expr),
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
}

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.values.get(name).cloned()
    }
}

pub fn eval(ast: Vec<Stmt>) -> Result<Value, InterpreterError> {
    let mut env = Environment::new();
    eval_with_env(ast, &mut env)
}

pub fn eval_with_env(ast: Vec<Stmt>, env: &mut Environment) -> Result<Value, InterpreterError> {
    let mut result = Value::Number(0.0);

    for stmt in ast {
        result = eval_stmt(&stmt, env)?;
    }

    Ok(result)
}

fn eval_stmt(stmt: &Stmt, env: &mut Environment) -> Result<Value, InterpreterError> {
    match stmt {
        Stmt::Expr(expr) => eval_expr(expr, env),
        Stmt::Let { name, value } => {
            let val = eval_expr(value, env)?;
            env.define(name.clone(), val.clone());
            Ok(val)
        }
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let cond = eval_expr(condition, env)?;
            match cond {
                Value::Boolean(true) => eval_with_env((*then_branch).clone(), env),
                Value::Boolean(false) => {
                    if let Some(else_branch) = else_branch.clone() {
                        eval_with_env((else_branch).clone(), env)
                    } else {
                        Ok(Value::Number(0.0))
                    }
                }
                _ => Err(InterpreterError::TypeMismatch(
                    "Condition must evaluate to boolean".to_string(),
                )),
            }
        }
        _ => Ok(Value::Number(0.0)), // 暂时不支持其他语句
    }
}

fn eval_expr(expr: &Expr, env: &Environment) -> Result<Value, InterpreterError> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::Boolean(b) => Ok(Value::Boolean(*b)),
        Expr::Variable(name) => match env.get(name.as_str()) {
            Some(value) => Ok(value.clone()),
            None => Err(InterpreterError::UndefinedVariable(name.clone())),
        },
        Expr::BinaryOp { left, op, right } => {
            let left_val = eval_expr(left, env)?;
            let right_val = eval_expr(right, env)?;

            match (left_val.clone(), op.clone(), right_val.clone()) {
                (Value::Number(l), Token::Plus, Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::Number(l), Token::Minus, Value::Number(r)) => Ok(Value::Number(l - r)),
                (Value::Number(l), Token::Multiply, Value::Number(r)) => Ok(Value::Number(l * r)),
                (Value::Number(l), Token::Divide, Value::Number(r)) => Ok(Value::Number(l / r)),
                (Value::Number(l), Token::Keyword(k), Value::Number(r)) if k == "==" => {
                    Ok(Value::Boolean(l == r))
                }
                (Value::Number(l), Token::Keyword(k), Value::Number(r)) if k == "!=" => {
                    Ok(Value::Boolean(l != r))
                }
                (Value::Number(l), Token::Keyword(k), Value::Number(r)) if k == ">" => {
                    Ok(Value::Boolean(l > r))
                }
                (Value::Number(l), Token::Keyword(k), Value::Number(r)) if k == ">=" => {
                    Ok(Value::Boolean(l >= r))
                }
                (Value::Number(l), Token::Keyword(k), Value::Number(r)) if k == "<" => {
                    Ok(Value::Boolean(l < r))
                }
                (Value::Number(l), Token::Keyword(k), Value::Number(r)) if k == "<=" => {
                    Ok(Value::Boolean(l <= r))
                }
                (Value::Boolean(l), Token::Keyword(k), Value::Boolean(r)) if k == "==" => {
                    Ok(Value::Boolean(l == r))
                }
                (Value::Boolean(l), Token::Keyword(k), Value::Boolean(r)) if k == "!=" => {
                    Ok(Value::Boolean(l != r))
                }
                _ => Err(InterpreterError::InvalidOperation(format!(
                    "{:?} {:?} {:?}",
                    left_val, op, right_val
                ))),
            }
        }
        Expr::UnaryOp { op, expr } => {
            let val = eval_expr(expr, env)?;
            match (op, val) {
                (Token::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                _ => Err(InterpreterError::InvalidOperation(format!(
                    "{:?} {}",
                    op, "on non-number value"
                ))),
            }
        }
        _ => Err(InterpreterError::UnsupportedExpression(format!(
            "{:?}",
            expr
        ))),
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
        let tokens = tokenize("if 1 < 2 then 3 else 4").unwrap();
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
        let tokens = tokenize("if 1 then 2 else 3").unwrap();
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
        // 假设我们有不支持的表达式类型
        // 这里需要根据实际AST结构调整测试
        let tokens = tokenize("unsupported").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }
}
