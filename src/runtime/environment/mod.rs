use std::collections::HashMap;

use crate::parser::ast::Expr;
use crate::runtime::environment::environment_value::EnvironmentValue;

pub mod builtin_function;
pub mod environment_value;
pub mod value;

use crate::runtime::environment::builtin_function::BuiltinFunction;

use crate::runtime::environment::value::Value;

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
            EnvironmentValue::Builtin(BuiltinFunction::Print),
        );
        values.insert(
            "push".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Push),
        );
        values.insert(
            "pop".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Pop),
        );
        values.insert(
            "input".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Input),
        );

        Self { values }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, EnvironmentValue::Variable(value));
    }

    pub fn define_function(&mut self, name: String, params: Vec<String>, body: Expr) {
        self.values
            .insert(name, EnvironmentValue::Function { params, body });
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        match self.values.get(name) {
            Some(EnvironmentValue::Variable(value)) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get_function(&self, name: &str) -> Option<(Vec<String>, Expr)> {
        match self.values.get(name) {
            Some(EnvironmentValue::Function { params, body }) => {
                Some((params.clone(), body.clone()))
            }
            _ => None,
        }
    }
}
