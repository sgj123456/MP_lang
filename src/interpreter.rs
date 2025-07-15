use std::{collections::HashMap, error::Error, fmt};

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

use crate::{
    ast::{Expr, Stmt},
    lexer::Token,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Vector(Vec<Value>),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Nil => write!(f, "nil"),
            Value::Vector(v) => {
                write!(f, "[")?;
                for (i, item) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BuiltinFunction {
    Print,
    Len,
    ToString,
    Vector,
    Push,
    Pop,
}

impl BuiltinFunction {
    fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        match self {
            BuiltinFunction::Print => {
                for arg in args {
                    print!("{arg} ");
                }
                println!();
                Ok(Value::Nil)
            }
            BuiltinFunction::Len => match args.first() {
                Some(Value::Number(n)) => Ok(Value::Number(n.abs())),
                Some(Value::Boolean(b)) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
                Some(Value::Nil) => Ok(Value::Number(0.0)),
                Some(Value::Vector(v)) => Ok(Value::Number(v.len() as f64)),
                _ => Err(InterpreterError::TypeMismatch(
                    "len() expects a number, boolean, nil or vector".to_string(),
                )),
            },
            BuiltinFunction::ToString => match args.first() {
                Some(value) => Ok(Value::Number(value.to_string().len() as f64)),
                None => Err(InterpreterError::TypeMismatch(
                    "toString() expects one argument".to_string(),
                )),
            },
            BuiltinFunction::Vector => Ok(Value::Vector(args)),
            BuiltinFunction::Push => match args.as_slice() {
                [Value::Vector(v), item] => {
                    let mut new_vec = v.clone();
                    new_vec.push(item.clone());
                    Ok(Value::Vector(new_vec))
                }
                _ => Err(InterpreterError::TypeMismatch(
                    "push() expects a vector and an item".to_string(),
                )),
            },
            BuiltinFunction::Pop => match args.first() {
                Some(Value::Vector(v)) if !v.is_empty() => {
                    let mut new_vec = v.clone();
                    let popped = new_vec.pop().unwrap();
                    Ok(popped)
                }
                Some(Value::Vector(_)) => Err(InterpreterError::InvalidOperation(
                    "Cannot pop from empty vector".to_string(),
                )),
                _ => Err(InterpreterError::TypeMismatch(
                    "pop() expects a vector".to_string(),
                )),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum EnvironmentValue {
    Variable(Value),
    Function { params: Vec<String>, body: Expr },
    Builtin(BuiltinFunction),
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
        let mut values = HashMap::new();

        values.insert(
            "print".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Print),
        );
        values.insert(
            "len".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Len),
        );
        values.insert(
            "toString".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::ToString),
        );
        values.insert(
            "vector".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Vector),
        );
        values.insert(
            "push".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Push),
        );
        values.insert(
            "pop".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Pop),
        );

        Self { values }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, EnvironmentValue::Variable(value));
    }

    pub fn define_function(&mut self, name: String, params: Vec<String>, body: Expr) {
        self.values
            .insert(name, EnvironmentValue::Function { params, body });
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        match self.values.get(name) {
            Some(EnvironmentValue::Variable(value)) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_function(&self, name: &str) -> Option<(Vec<String>, Expr)> {
        match self.values.get(name) {
            Some(EnvironmentValue::Function { params, body }) => {
                Some((params.clone(), body.clone()))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
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
            if let Token::Equal = op {
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
                    Token::Plus => Ok(Value::Number(l + r)),
                    Token::Minus => Ok(Value::Number(l - r)),
                    Token::Multiply => Ok(Value::Number(l * r)),
                    Token::Divide => Ok(Value::Number(l / r)),
                    Token::GreaterThan => Ok(Value::Boolean(l > r)),
                    Token::GreaterThanOrEqual => Ok(Value::Boolean(l >= r)),
                    Token::LessThan => Ok(Value::Boolean(l < r)),
                    Token::LessThanOrEqual => Ok(Value::Boolean(l <= r)),
                    Token::Equal => Ok(Value::Boolean(l == r)),
                    Token::NotEqual => Ok(Value::Boolean(l != r)),
                    _ => Err(InterpreterError::InvalidOperation(format!("{op:?}"))),
                },
                (Value::Boolean(l), Value::Boolean(r)) => match op {
                    Token::Equal => Ok(Value::Boolean(l == r)),
                    Token::NotEqual => Ok(Value::Boolean(l != r)),
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
        let tokens = tokenize("if 1 < 2 {3} else {4}").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_undefined_variable() {
        let tokens = tokenize("x;").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_invalid_operation() {
        let tokens = tokenize("true + 1;").unwrap();
        let ast = parse(tokens);
        assert!(eval(ast).is_err());
    }

    #[test]
    fn test_type_mismatch() {
        let tokens = tokenize("if 1 + true {2} else {3}").unwrap();
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

    #[test]
    fn test_while_loop() {
        let tokens = tokenize("{ let x = 0; while x < 3 { x = x + 1 } }").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(
            result,
            Value::Vector(Vec::from([
                Value::Number(1.0),
                Value::Number(2.0),
                Value::Number(3.0)
            ]))
        );
    }

    #[test]
    fn test_while_with_condition_false() {
        let tokens = tokenize("while false { 1 };").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_nested_while_loops() {
        let tokens = tokenize(
            "{
            let x = 0;
            let y = 0;
            while x < 2 {
                x = x + 1;
                while y < 3 {
                    y = y + 1;
                }
            };
            y
        }",
        )
        .unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(3.0))
    }

    #[test]
    fn test_vector_operations() {
        let tokens = tokenize("let v = vector(1, 2, 3); push(v, 4); pop(v)").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn test_function_return() {
        let tokens = tokenize("fn add(a, b) { return a + b; }; add(2, 3)").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    fn test_early_return() {
        let tokens = tokenize("fn test() { return 10; 20; }; test()").unwrap();
        let ast = parse(tokens);
        let result = eval(ast).unwrap();
        assert_eq!(result, Value::Number(10.0));
    }
}
