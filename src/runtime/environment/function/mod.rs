mod builtin;
mod user;
pub use crate::runtime::environment::function::builtin::BuiltinFunction;
pub use crate::runtime::environment::function::user::UserFunction;

use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    Environment,
    runtime::{environment::value::Value, error::InterpreterError},
};

pub trait Fun {
    fn call(&self, args: Vec<Value>, env: &mut Environment) -> Result<Value, InterpreterError>;
    fn call_rc(
        &self,
        args: Vec<Value>,
        env: &Rc<RefCell<Environment>>,
    ) -> Result<Value, InterpreterError>;
}

#[derive(Debug, Clone)]
pub enum Function {
    Builtin(BuiltinFunction),
    User(UserFunction),
}
impl Fun for Function {
    fn call(&self, args: Vec<Value>, env: &mut Environment) -> Result<Value, InterpreterError> {
        match self {
            Function::Builtin(f) => f.call(args, env),
            Function::User(f) => f.call(args, env),
        }
    }

    fn call_rc(
        &self,
        args: Vec<Value>,
        env: &Rc<RefCell<Environment>>,
    ) -> Result<Value, InterpreterError> {
        match self {
            Function::Builtin(f) => f.call_rc(args, env),
            Function::User(f) => f.call_rc(args, env),
        }
    }
}
