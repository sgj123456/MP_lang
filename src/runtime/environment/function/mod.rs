mod builtin;
mod user;
pub use crate::runtime::environment::function::builtin::BuiltinFunction;
pub use crate::runtime::environment::function::user::UserFunction;

use crate::runtime::{environment::value::Value, error::InterpreterError};

pub trait Fun {
    fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError>;
}
