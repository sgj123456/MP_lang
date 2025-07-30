use crate::runtime::{environment::value::Value, error::InterpreterError};

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
    pub fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
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
