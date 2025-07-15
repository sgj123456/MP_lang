use crate::{
    ast::{Expr, Stmt},
    lexer::{Token, TokenKind},
    runtime::{
        environment::{Environment, environment_value::EnvironmentValue, value::Value},
        error::interpreter_error::InterpreterError,
    },
};

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
        Stmt::Expr(expr) => {
            eval_expr(expr, env)?;
            Ok(Value::Nil)
        }
        Stmt::Let { name, value } => {
            let value = eval_expr(value, env)?;
            env.define(name.clone(), value);
            Ok(Value::Nil)
        }
        Stmt::Function { name, params, body } => {
            env.define_function(name.clone(), params.clone(), body.clone());
            Ok(Value::Nil)
        }
        Stmt::Result(expr) => eval_expr(expr, env),
        Stmt::Return(Some(expr)) => Err(InterpreterError::Return(eval_expr(expr, env)?)),
        Stmt::Return(None) => Err(InterpreterError::Return(Value::Nil)),
    }
}

fn eval_expr(expr: &Expr, env: &mut Environment) -> Result<Value, InterpreterError> {
    match expr {
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::Boolean(b) => Ok(Value::Boolean(*b)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Variable(name) => match env.get(name.as_str()) {
            Some(value) => Ok(value),
            None => Err(InterpreterError::UndefinedVariable(name.clone())),
        },
        Expr::BinaryOp { left, op, right } => {
            if let TokenKind::Equal = op {
                if let Expr::Variable(name) = left.as_ref() {
                    let right_value = eval_expr(right, env)?;
                    env.define(name.clone(), right_value.clone());
                    return Ok(right_value);
                } else {
                    return Err(InterpreterError::InvalidOperation(
                        "Invalid assignment target".to_string(),
                    ));
                }
            }

            let left_value = eval_expr(left, env)?;
            let right_value = eval_expr(right, env)?;

            match (left_value, right_value) {
                (Value::Number(l), Value::Number(r)) => match op {
                    TokenKind::Plus => Ok(Value::Number(l + r)),
                    TokenKind::Minus => Ok(Value::Number(l - r)),
                    TokenKind::Multiply => Ok(Value::Number(l * r)),
                    TokenKind::Divide => Ok(Value::Number(l / r)),
                    TokenKind::GreaterThan => Ok(Value::Boolean(l > r)),
                    TokenKind::GreaterThanOrEqual => Ok(Value::Boolean(l >= r)),
                    TokenKind::LessThan => Ok(Value::Boolean(l < r)),
                    TokenKind::LessThanOrEqual => Ok(Value::Boolean(l <= r)),
                    TokenKind::Equal => Ok(Value::Boolean(l == r)),
                    TokenKind::NotEqual => Ok(Value::Boolean(l != r)),
                    _ => Err(InterpreterError::InvalidOperation(format!("{op:?}"))),
                },
                (Value::Boolean(l), Value::Boolean(r)) => match op {
                    TokenKind::Equal => Ok(Value::Boolean(l == r)),
                    TokenKind::NotEqual => Ok(Value::Boolean(l != r)),
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
                (TokenKind::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                _ => Err(InterpreterError::InvalidOperation(format!("{op:?}"))),
            }
        }
        Expr::FunctionCall { name, args } => match env.get_function(name.as_str()) {
            Some((params, body)) => {
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
                match eval_expr(&body, &mut call_env) {
                    Ok(value) | Err(InterpreterError::Return(value)) => Ok(value),
                    Err(err) => Err(err),
                }
            }
            None => {
                let evaluated_args = args
                    .iter()
                    .map(|arg| eval_expr(arg, &mut env.clone()))
                    .collect::<Result<Vec<_>, _>>()?;
                match env.values.get(name.as_str()) {
                    Some(EnvironmentValue::Builtin(builtin)) => builtin.call(evaluated_args),
                    _ => Err(InterpreterError::UndefinedVariable(name.clone())),
                }
            }
        },
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition_value = eval_expr(condition, env)?;
            if let Value::Boolean(b) = condition_value {
                if b {
                    eval_expr(then_branch, env)
                } else if let Some(else_branch) = else_branch {
                    eval_expr(else_branch, env)
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
            let mut block_env = env.clone();
            let mut result = Value::Nil;
            for stmt in statements {
                result = eval_stmt(stmt, &mut block_env)?;
            }
            Ok(result)
        }
        Expr::While { condition, body } => {
            let mut result = Vec::new();
            loop {
                let condition_value = eval_expr(condition, env)?;
                if let Value::Boolean(b) = condition_value {
                    if !b {
                        break;
                    }
                } else {
                    return Err(InterpreterError::TypeMismatch(
                        "While condition must be boolean".to_string(),
                    ));
                }
                let (last, body) = body.split_last().unwrap();
                for stmt in body {
                    eval_stmt(stmt, env)?;
                }
                result.push(eval_stmt(last, env)?);
            }
            if result.is_empty() {
                Ok(Value::Nil)
            } else {
                Ok(Value::Vector(result))
            }
        }
    }
}
