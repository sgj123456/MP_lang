use std::cell::RefCell;
use std::rc::Rc;

use crate::{
    parser::Expr,
    runtime::{
        environment::{Environment, function::Fun, value::Value},
        error::InterpreterError,
        eval::eval_expr_rc,
    },
};

#[derive(Debug, Clone)]
pub struct UserFunction {
    pub params: Vec<String>,
    pub body: Expr,
}

impl Fun for UserFunction {
    fn call(&self, args: Vec<Value>, parent: &mut Environment) -> Result<Value, InterpreterError> {
        let env = Environment::new_child(Rc::new(RefCell::new(parent.clone())));
        let env_rc = Rc::new(RefCell::new(env));

        for (param, arg) in self.params.iter().zip(args) {
            env_rc.borrow_mut().define(param.to_string(), arg);
        }

        match eval_expr_rc(&self.body, &env_rc) {
            Err(InterpreterError::Return(value)) => Ok(value),
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    fn call_rc(
        &self,
        args: Vec<Value>,
        parent: &Rc<RefCell<Environment>>,
    ) -> Result<Value, InterpreterError> {
        let env = Environment::new_child(parent.clone());
        let env_rc = Rc::new(RefCell::new(env));

        for (param, arg) in self.params.iter().zip(args) {
            env_rc.borrow_mut().define(param.to_string(), arg);
        }

        match eval_expr_rc(&self.body, &env_rc) {
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
