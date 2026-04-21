use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    lexer::TokenKind,
    parser::{Expr, ExprKind, Stmt, StmtKind},
    runtime::{
        environment::{Environment, function::Fun, value::Value},
        error::InterpreterError,
    },
};

pub fn eval(ast: Vec<Stmt>) -> Result<Value, InterpreterError> {
    let env = Environment::new_root();
    let env = Rc::new(RefCell::new(env));
    eval_with_env(ast, &env)
}

pub fn eval_with_env(
    ast: Vec<Stmt>,
    env: &Rc<RefCell<Environment>>,
) -> Result<Value, InterpreterError> {
    let mut result = Value::Nil;

    for stmt in ast {
        result = eval_stmt(&stmt, env)?;
    }

    Ok(result)
}

pub fn eval_stmt(stmt: &Stmt, env: &Rc<RefCell<Environment>>) -> Result<Value, InterpreterError> {
    match &stmt.kind {
        StmtKind::Expr(expr) => {
            eval_expr(expr, env)?;
            Ok(Value::Nil)
        }
        StmtKind::Let { name, value, .. } => {
            let value = eval_expr(value, env)?;
            env.borrow_mut().define(name.clone(), value)?;
            Ok(Value::Nil)
        }
        StmtKind::Function { name, params, body } => {
            env.borrow_mut()
                .define_function(name.clone(), params.clone(), body.clone())?;
            Ok(Value::Nil)
        }
        StmtKind::Struct { name, fields } => {
            let mut evaluated_fields = Vec::new();
            for (field_name, default_value) in fields {
                let value = match default_value {
                    Some(expr) => Some(eval_expr(expr, env)?),
                    None => None,
                };
                evaluated_fields.push((field_name.clone(), value));
            }
            env.borrow_mut()
                .define_struct(name.clone(), evaluated_fields)?;
            Ok(Value::Nil)
        }
        StmtKind::Break => Err(InterpreterError::Break),
        StmtKind::Continue => Err(InterpreterError::Continue),
        StmtKind::Result(expr) => eval_expr(expr, env),
        StmtKind::Return(Some(expr)) => Err(InterpreterError::Return(eval_expr(expr, env)?)),
        StmtKind::Return(None) => Err(InterpreterError::Return(Value::Nil)),
    }
}

pub fn eval_expr(expr: &Expr, env: &Rc<RefCell<Environment>>) -> Result<Value, InterpreterError> {
    match &expr.kind {
        ExprKind::Number(n) => Ok(Value::Number(n.clone())),
        ExprKind::Boolean(b) => Ok(Value::Boolean(*b)),
        ExprKind::String(s) => Ok(Value::String(s.clone())),
        ExprKind::Parenthesized(expr) => eval_expr(expr, env),
        ExprKind::Variable(name) => match env.borrow().get_value(name.as_str()) {
            Some(value) => Ok(value),
            None => Err(InterpreterError::UndefinedVariable(name.clone())),
        },
        ExprKind::BinaryOp { left, op, right } => {
            if let TokenKind::Assign = op {
                if let ExprKind::Variable(name) = &left.as_ref().kind {
                    let right_value = eval_expr(right, env)?;
                    env.borrow_mut()
                        .assign(name.as_str(), right_value.clone())?;
                    return Ok(right_value);
                } else if let ExprKind::Index { object, index } = &left.as_ref().kind {
                    let obj_value = eval_expr(object, env)?;
                    let index_value = eval_expr(index, env)?;
                    let right_value = eval_expr(right, env)?;

                    return match (obj_value, index_value) {
                        (Value::Array(arr), Value::Number(num)) => {
                            let idx = num.to_int() as usize;
                            let mut arr_mut = arr.borrow_mut();
                            if idx < arr_mut.len() {
                                arr_mut[idx] = right_value.clone();
                                Ok(right_value)
                            } else {
                                Err(InterpreterError::InvalidOperation(format!(
                                    "Array index out of bounds: {} (length: {})",
                                    idx,
                                    arr_mut.len()
                                )))
                            }
                        }
                        (Value::String(s), Value::Number(num)) => {
                            let idx = num.to_int() as isize;
                            let len = s.len() as isize;
                            let actual_idx = if idx < 0 { len + idx } else { idx };
                            if actual_idx >= 0 && actual_idx < len {
                                if let ExprKind::Variable(var_name) = &object.as_ref().kind {
                                    let mut new_chars: Vec<char> = s.chars().collect();
                                    let new_char = right_value.to_string();
                                    if let Some(c) = new_char.chars().next() {
                                        new_chars[actual_idx as usize] = c;
                                        let new_string: String = new_chars.into_iter().collect();
                                        let new_value = Value::String(new_string);
                                        env.borrow_mut()
                                            .assign(var_name.as_str(), new_value.clone())?;
                                        Ok(right_value)
                                    } else {
                                        Err(InterpreterError::InvalidOperation(
                                            "Cannot assign empty value to string index".to_string(),
                                        ))
                                    }
                                } else {
                                    Err(InterpreterError::InvalidOperation(
                                        "Cannot assign to string index directly, use variable"
                                            .to_string(),
                                    ))
                                }
                            } else {
                                Err(InterpreterError::InvalidOperation(format!(
                                    "String index out of bounds: {} (length: {})",
                                    idx, len
                                )))
                            }
                        }
                        _ => Err(InterpreterError::TypeMismatch(
                            "Index assignment requires array or string".to_string(),
                        )),
                    };
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
                    TokenKind::Modulo => Ok(Value::Number(l % r)),
                    TokenKind::GreaterThan => Ok(Value::Boolean(l > r)),
                    TokenKind::GreaterThanOrEqual => Ok(Value::Boolean(l >= r)),
                    TokenKind::LessThan => Ok(Value::Boolean(l < r)),
                    TokenKind::LessThanOrEqual => Ok(Value::Boolean(l <= r)),
                    TokenKind::Equal => Ok(Value::Boolean(l == r)),
                    TokenKind::NotEqual => Ok(Value::Boolean(l != r)),
                    TokenKind::LogicalAnd => Ok(Value::Boolean(l.to_bool() && r.to_bool())),
                    TokenKind::LogicalOr => Ok(Value::Boolean(l.to_bool() || r.to_bool())),
                    _ => Err(InterpreterError::InvalidOperation(format!("{op:?}"))),
                },
                (Value::Boolean(l), Value::Boolean(r)) => match op {
                    TokenKind::Equal => Ok(Value::Boolean(l == r)),
                    TokenKind::NotEqual => Ok(Value::Boolean(l != r)),
                    TokenKind::LogicalAnd => Ok(Value::Boolean(l && r)),
                    TokenKind::LogicalOr => Ok(Value::Boolean(l || r)),
                    _ => Err(InterpreterError::InvalidOperation(format!("{op:?}"))),
                },
                (Value::String(l), Value::String(r)) => match op {
                    TokenKind::Plus => Ok(Value::String(l + &r)),
                    TokenKind::Equal => Ok(Value::Boolean(l == r)),
                    TokenKind::NotEqual => Ok(Value::Boolean(l != r)),
                    TokenKind::LogicalAnd | TokenKind::LogicalOr => {
                        let bool_l = !l.is_empty();
                        let bool_r = !r.is_empty();
                        match op {
                            TokenKind::LogicalAnd => Ok(Value::Boolean(bool_l && bool_r)),
                            TokenKind::LogicalOr => Ok(Value::Boolean(bool_l || bool_r)),
                            _ => Err(InterpreterError::InvalidOperation(format!("{op:?}"))),
                        }
                    }
                    _ => Err(InterpreterError::InvalidOperation(format!("{op:?}"))),
                },
                _ => Err(InterpreterError::TypeMismatch(
                    "Invalid operands for binary operation".to_string(),
                )),
            }
        }
        ExprKind::UnaryOp { op, expr } => {
            let value = eval_expr(expr, env)?;
            match (op, value) {
                (TokenKind::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                (TokenKind::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
                (TokenKind::Not, Value::Nil) => Ok(Value::Boolean(true)),
                _ => Err(InterpreterError::InvalidOperation(format!("{op:?}"))),
            }
        }
        ExprKind::StructInstance { name, args } => {
            let mut args_values = Vec::new();
            for arg in args {
                args_values.push(eval_expr(arg, env)?);
            }
            let struct_def = match env.borrow().get_struct(name.as_str()) {
                Some(def) => def,
                None => return Err(InterpreterError::UndefinedVariable(name.clone())),
            };
            let mut fields = HashMap::new();
            for (i, (field_name, default_value)) in struct_def.fields.iter().enumerate() {
                let value = if i < args_values.len() {
                    args_values[i].clone()
                } else if let Some(default) = default_value {
                    default.clone()
                } else {
                    Value::Nil
                };
                fields.insert(field_name.clone(), value);
            }
            Ok(Value::StructInstance {
                name: name.clone(),
                fields,
            })
        }
        ExprKind::FunctionCall { name, args } => {
            let mut args_values = Vec::new();
            for arg in args {
                args_values.push(eval_expr(arg, env)?);
            }
            if let Some(struct_def) = env.borrow().get_struct(name.as_str()) {
                let mut fields = HashMap::new();
                for (i, (field_name, default_value)) in struct_def.fields.iter().enumerate() {
                    let value = if i < args_values.len() {
                        args_values[i].clone()
                    } else if let Some(default) = default_value {
                        default.clone()
                    } else {
                        Value::Nil
                    };
                    fields.insert(field_name.clone(), value);
                }
                return Ok(Value::StructInstance {
                    name: name.clone(),
                    fields,
                });
            }
            let fn_value = match env.borrow().get_function_recursive(name.as_str()) {
                Some(value) => value,
                None => return Err(InterpreterError::UndefinedVariable(name.clone())),
            };
            fn_value.call(args_values, env)
        }
        ExprKind::If {
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
        ExprKind::Block(statements) => {
            let block_env = Rc::new(RefCell::new(Environment::new_child(env.clone())));
            let mut result = Value::Nil;
            for stmt in statements {
                let stmt = Stmt {
                    kind: stmt.clone(),
                    span: crate::lexer::Span { line: 0, column: 0 },
                };
                result = eval_stmt(&stmt, &block_env)?;
            }
            Ok(result)
        }
        ExprKind::While { condition, body } => {
            let mut result = Vec::new();
            loop {
                let condition_value = eval_expr(condition, env)?;
                match condition_value {
                    Value::Boolean(false) => break,
                    Value::Boolean(true) => {}
                    _ => {
                        return Err(InterpreterError::TypeMismatch(
                            "While condition must be boolean".to_string(),
                        ));
                    }
                }
                let value = match eval_expr(body, env) {
                    Ok(value) => value,
                    Err(InterpreterError::Break) => break,
                    Err(InterpreterError::Continue) => continue,
                    err @ Err(_) => return err,
                };
                result.push(value);
            }
            if result.is_empty() {
                Ok(Value::Nil)
            } else {
                Ok(Value::Array(Rc::new(RefCell::new(result))))
            }
        }
        ExprKind::Array(values) => {
            let evaluated_values = values
                .iter()
                .map(|value| eval_expr(value, env))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Array(Rc::new(RefCell::new(evaluated_values))))
        }
        ExprKind::Object(vec) => {
            let mut object = HashMap::new();
            for (key, value) in vec {
                let value = eval_expr(value, env)?;
                object.insert(key.clone(), value);
            }
            Ok(Value::Object(object))
        }
        ExprKind::Index { object, index } => {
            let obj_value = eval_expr(object, env)?;
            let index_value = eval_expr(index, env)?;

            match (obj_value, index_value) {
                (Value::Array(arr), Value::Number(num)) => {
                    let idx = num.to_int() as usize;
                    let arr = arr.borrow();
                    if idx < arr.len() {
                        Ok(arr[idx].clone())
                    } else {
                        Err(InterpreterError::InvalidOperation(format!(
                            "Array index out of bounds: {} (length: {})",
                            idx,
                            arr.len()
                        )))
                    }
                }
                (Value::String(s), Value::Number(num)) => {
                    let idx = num.to_int() as isize;
                    let len = s.len() as isize;
                    let actual_idx = if idx < 0 { len + idx } else { idx };
                    if actual_idx >= 0 && actual_idx < len {
                        let ch = s.chars().nth(actual_idx as usize).unwrap();
                        Ok(Value::String(ch.to_string()))
                    } else {
                        Err(InterpreterError::InvalidOperation(format!(
                            "String index out of bounds: {} (length: {})",
                            idx, len
                        )))
                    }
                }
                (Value::Object(obj), Value::String(key)) => {
                    if let Some(value) = obj.get(&key) {
                        Ok(value.clone())
                    } else {
                        Err(InterpreterError::InvalidOperation(format!(
                            "Object property not found: {}",
                            key
                        )))
                    }
                }
                (Value::StructInstance { fields, .. }, Value::String(key)) => {
                    if let Some(value) = fields.get(&key) {
                        Ok(value.clone())
                    } else {
                        Err(InterpreterError::InvalidOperation(format!(
                            "Struct property not found: {}",
                            key
                        )))
                    }
                }
                _ => Err(InterpreterError::TypeMismatch(
                    "Index access requires array/string index or object/string property"
                        .to_string(),
                )),
            }
        }
        ExprKind::GetProperty { object, property } => {
            let obj_value = eval_expr(object, env)?;

            match obj_value {
                Value::Object(obj) => {
                    if let Some(value) = obj.get(property.as_str()) {
                        Ok(value.clone())
                    } else {
                        Err(InterpreterError::InvalidOperation(format!(
                            "Object property not found: {}",
                            property
                        )))
                    }
                }
                Value::StructInstance { fields, .. } => {
                    if let Some(value) = fields.get(property.as_str()) {
                        Ok(value.clone())
                    } else {
                        Err(InterpreterError::InvalidOperation(format!(
                            "Struct property not found: {}",
                            property
                        )))
                    }
                }
                _ => Err(InterpreterError::TypeMismatch(
                    "Property access requires an object".to_string(),
                )),
            }
        }
    }
}
