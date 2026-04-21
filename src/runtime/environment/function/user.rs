use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    parser::Expr,
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
    fn call(
        &self,
        args: Vec<Value>,
        parent: &Rc<RefCell<Environment>>,
    ) -> Result<Value, InterpreterError> {
        let env = Rc::new(RefCell::new(Environment::new_child(parent.clone())));

        for (param, arg) in self.params.iter().zip(args) {
            env.borrow_mut().define(param.to_string(), arg)?;
        }

        match eval_expr(&self.body, &env) {
            Err(InterpreterError::Return(value)) => Ok(value),
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

impl UserFunction {
    pub fn new(params: Vec<String>, body: Expr) -> Self {
        Self { params, body }
    }
}
