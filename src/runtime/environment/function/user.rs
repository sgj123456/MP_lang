use crate::{
    parser::ast::Expr,
    runtime::{
        environment::{Environment, function::Fun, value::Value},
        error::InterpreterError,
        eval::eval_expr,
    },
};

#[derive(Debug, Clone)]
pub struct UserFunction {
    pub params: Vec<String>,
    pub body: Expr,
}

impl Fun for UserFunction {
    fn call(&self, args: Vec<Value>) -> Result<Value, InterpreterError> {
        let mut env = Environment::new();

        for (i, arg) in args.into_iter().zip(self.params.iter()) {
            env.define(arg.to_string(), i);
        }
        match eval_expr(&self.body, &mut env) {
            Err(InterpreterError::Return(value)) => Ok(value),
            n => n,
        }
    }
}

impl UserFunction {
    pub fn new(params: Vec<String>, body: Expr) -> Self {
        Self { params, body }
    }
}
