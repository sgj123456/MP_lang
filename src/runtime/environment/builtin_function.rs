use crate::runtime::{environment::value::Value, error::InterpreterError};

#[derive(Debug, Clone)]
pub enum BuiltinFunction {
    Print,
    Push,
    Pop,
}

impl BuiltinFunction {
    pub fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        match self {
            BuiltinFunction::Print => {
                for arguments in args {
                    print!("{arguments} ");
                }
                println!();
                Ok(Value::Nil)
            }
            BuiltinFunction::Push => match args.as_slice() {
                [Value::Array(v), item] => {
                    let mut new_vec = v.clone();
                    new_vec.push(item.clone());
                    Ok(Value::Array(new_vec))
                }
                _ => Err(InterpreterError::TypeMismatch(
                    "push() expects a vector and an item".to_string(),
                )),
            },
            BuiltinFunction::Pop => match args.first() {
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
            },
        }
    }
}
