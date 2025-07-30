use std::collections::HashMap;

use crate::parser::ast::Expr;
use crate::runtime::environment::environment_value::EnvironmentValue;

pub mod environment_value;
pub mod value;
pub mod builtin_function;

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
            "len".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Len),
        );
        values.insert(
            "toString".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::ToString),
        );
        values.insert(
            "vector".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Vector),
        );
        values.insert(
            "push".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Push),
        );
        values.insert(
            "pop".to_string(),
            EnvironmentValue::Builtin(BuiltinFunction::Pop),
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
