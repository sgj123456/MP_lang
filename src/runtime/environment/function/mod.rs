mod builtin;
mod user;
pub use crate::runtime::environment::function::builtin::BuiltinFunction;
pub use crate::runtime::environment::function::user::UserFunction;

use crate::runtime::{environment::value::Value, error::InterpreterError};

pub trait Fun {
    fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError>;
}

#[derive(Debug, Clone)]
pub enum Function {
    Builtin(BuiltinFunction),
    User(UserFunction),
}
impl Fun for Function {
    fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        match self {
            Function::Builtin(f) => f.call(args),
            Function::User(f) => f.call(args),
        }
    }
}
