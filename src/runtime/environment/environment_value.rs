use crate::runtime::{
    environment::{
        function::{BuiltinFunction, Fun, UserFunction},
        value::Value,
    },
    error::InterpreterError,
};

#[derive(Debug, Clone)]
pub enum EnvironmentValue {
    Variable(Value),
    Function(Function),
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
