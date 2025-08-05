use crate::runtime::{
    environment::{
        function::Fun,
        value::{Number, Value},
    },
    error::InterpreterError,
};

#[derive(Debug, Clone)]
pub enum BuiltinFunction {
    Print,
    Input,
    Int,
    Float,
    Random,
    Push,
    Pop,
}

fn print(args: Vec<Value>) -> Result<Value, InterpreterError> {
    for arguments in args {
        print!("{arguments} ");
    }
    println!();
    Ok(Value::Nil)
}

fn input() -> Result<Value, InterpreterError> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    Ok(Value::String(input.trim().to_string()))
}

fn push(args: Vec<Value>) -> Result<Value, InterpreterError> {
    match args.as_slice() {
        [Value::Array(v), item] => {
            let mut new_vec = v.clone();
            new_vec.push(item.clone());
            Ok(Value::Array(new_vec))
        }
        _ => Err(InterpreterError::TypeMismatch(
            "push() expects a vector and an item".to_string(),
        )),
    }
}

fn pop(args: Vec<Value>) -> Result<Value, InterpreterError> {
    match args.first() {
        Some(Value::Array(v)) if !v.is_empty() => {
            let mut new_vec = v.clone();
            let popped = new_vec.pop().unwrap();
            Ok(popped)
        }
        Some(Value::Array(_)) => Err(InterpreterError::InvalidOperation(
            "Cannot pop from empty vector".to_string(),
        )),
        _ => Err(InterpreterError::TypeMismatch(
            "pop() expects a vector".to_string(),
        )),
    }
}

fn int(args: Vec<Value>) -> Result<Value, InterpreterError> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(Number::Int(n.to_int()))),
        Some(Value::String(s)) => {
            Ok(Value::Number(Number::Int(s.parse().map_err(|e| {
                InterpreterError::InvalidOperation(format!("int() failed: {e}"))
            })?)))
        }
        _ => Err(InterpreterError::TypeMismatch(
            "int() expects a number or a string".to_string(),
        )),
    }
}

fn float(args: Vec<Value>) -> Result<Value, InterpreterError> {
    match args.first() {
        Some(Value::Number(n)) => Ok(Value::Number(Number::Float(n.to_float()))),
        Some(Value::String(s)) => {
            Ok(Value::Number(Number::Float(s.parse().map_err(|e| {
                InterpreterError::InvalidOperation(format!("float() failed: {e}"))
            })?)))
        }
        _ => Err(InterpreterError::TypeMismatch(
            "float() expects a number or a string".to_string(),
        )),
    }
}

fn random(args: Vec<Value>) -> Result<Value, InterpreterError> {
    match args.as_slice() {
        [] => Ok(Value::Number(Number::Int(rand::random()))),
        [Value::Number(n)] => match n {
            Number::Int(n) => Ok(Value::Number(Number::Int(rand::random_range(0..*n)))),
            Number::Float(n) => Ok(Value::Number(Number::Float(rand::random_range(0.0..*n)))),
        },
        [Value::Number(n1), Value::Number(n2)] => match (n1, n2) {
            (Number::Int(n1), Number::Int(n2)) => {
                Ok(Value::Number(Number::Int(rand::random_range(*n1..*n2))))
            }
            (Number::Float(n1), Number::Float(n2)) => {
                Ok(Value::Number(Number::Float(rand::random_range(*n1..*n2))))
            }
            _ => Err(InterpreterError::TypeMismatch(
                "random() expects two integers or two floats".to_string(),
            )),
        },
        _ => Err(InterpreterError::InvalidOperation(
            "random() expects 0, 1 or 2 arguments".to_string(),
        )),
    }
}

impl Fun for BuiltinFunction {
    fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        match self {
            BuiltinFunction::Print => print(args),
            BuiltinFunction::Input => input(),
            BuiltinFunction::Push => push(args),
            BuiltinFunction::Pop => pop(args),
            BuiltinFunction::Int => int(args),
            BuiltinFunction::Float => float(args),
            BuiltinFunction::Random => random(args),
        }
    }
}
