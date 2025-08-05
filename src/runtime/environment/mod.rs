use std::collections::HashMap;

use crate::{
    parser::Expr,
    runtime::environment::{function::Function, value::EnvironmentValue},
};

pub mod function;
pub mod value;

pub use function::{BuiltinFunction, UserFunction};
pub use value::Value;

/// The execution environment storing variables and functions
#[derive(Debug, Clone)]
pub struct Environment {
    pub(crate) values: HashMap<String, EnvironmentValue>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        let mut values = HashMap::new();

        values.insert(
            "print".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Print)),
        );
        values.insert(
            "push".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Push)),
        );
        values.insert(
            "pop".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Pop)),
        );
        values.insert(
            "input".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Input)),
        );

        Self { values }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, EnvironmentValue::Variable(value));
    }

    pub fn define_function(&mut self, name: String, params: Vec<String>, body: Expr) {
        self.values.insert(
            name,
            EnvironmentValue::Function(Function::User(UserFunction { params, body })),
        );
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        match self.values.get(name) {
            Some(EnvironmentValue::Variable(value)) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        match self.values.get(name) {
            Some(EnvironmentValue::Function(function)) => Some(function),
            _ => None,
        }
    }
}
