use crate::runtime::{
    environment::{function::Fun, value::Value},
    error::InterpreterError,
};

#[derive(Debug, Clone)]
pub enum BuiltinFunction {
    Print,
    Input,
    Push,
    Pop,
}

impl Fun for BuiltinFunction {
    fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        match self {
            BuiltinFunction::Print => {
                for arguments in args {
                    print!("{arguments} ");
                }
                println!();
                Ok(Value::Nil)
            }
            BuiltinFunction::Input => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                Ok(Value::String(input.trim().to_string()))
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
