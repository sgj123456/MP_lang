use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    parser::Expr,
    runtime::environment::{function::Function, value::EnvironmentValue, value::StructDef},
    runtime::error::InterpreterError,
};

pub mod function;
pub mod value;

pub use function::{BuiltinFunction, UserFunction};
pub use value::Value;

/// The execution environment storing variables and functions
#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    locals: HashMap<String, EnvironmentValue>,
}

impl Environment {
    pub fn new_root() -> Self {
        let mut locals = HashMap::new();

        locals.insert(
            "print".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Print)),
        );
        locals.insert(
            "push".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Push)),
        );
        locals.insert(
            "pop".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Pop)),
        );
        locals.insert(
            "input".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Input)),
        );
        locals.insert(
            "int".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Int)),
        );
        locals.insert(
            "float".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Float)),
        );
        locals.insert(
            "str".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::String)),
        );
        locals.insert(
            "len".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Len)),
        );
        locals.insert(
            "type".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Type)),
        );
        locals.insert(
            "random".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Random)),
        );
        locals.insert(
            "time".to_string(),
            EnvironmentValue::Function(Function::Builtin(BuiltinFunction::Time)),
        );
        locals.insert("nil".to_string(), EnvironmentValue::Variable(Value::Nil));

        Self {
            locals,
            parent: None,
        }
    }

    pub fn new_child(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            locals: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: String, value: Value) -> Result<(), InterpreterError> {
        if self.locals.contains_key(&name) {
            return Err(InterpreterError::RedefinedVariable(name));
        }
        self.locals.insert(name, EnvironmentValue::Variable(value));
        Ok(())
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), InterpreterError> {
        if self.locals.contains_key(name) {
            self.locals
                .insert(name.to_string(), EnvironmentValue::Variable(value));
            Ok(())
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign(name, value)
        } else {
            self.locals
                .insert(name.to_string(), EnvironmentValue::Variable(value));
            Ok(())
        }
    }

    pub fn define_function(
        &mut self,
        name: String,
        params: Vec<String>,
        body: Expr,
    ) -> Result<(), InterpreterError> {
        if self.locals.contains_key(&name) {
            return Err(InterpreterError::RedefinedVariable(name));
        }
        self.locals.insert(
            name,
            EnvironmentValue::Function(Function::User(UserFunction { params, body })),
        );
        Ok(())
    }

    pub fn define_struct(
        &mut self,
        name: String,
        fields: Vec<(String, Option<Value>)>,
    ) -> Result<(), InterpreterError> {
        if self.locals.contains_key(&name) {
            return Err(InterpreterError::RedefinedVariable(name));
        }
        self.locals.insert(
            name.clone(),
            EnvironmentValue::Struct(StructDef { name, fields }),
        );
        Ok(())
    }

    pub fn get_struct(&self, name: &str) -> Option<StructDef> {
        match self.locals.get(name) {
            Some(EnvironmentValue::Struct(def)) => Some(def.clone()),
            _ => self
                .parent
                .as_ref()
                .and_then(|parent| parent.borrow().get_struct(name)),
        }
    }

    pub fn get_value(&self, name: &str) -> Option<Value> {
        match self.locals.get(name) {
            Some(EnvironmentValue::Variable(value)) => Some(value.clone()),
            _ => self
                .parent
                .as_ref()
                .and_then(|parent| parent.borrow().get_value(name)),
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        match self.locals.get(name) {
            Some(EnvironmentValue::Function(function)) => Some(function),
            _ => None,
        }
    }

    pub fn get_function_recursive(&self, name: &str) -> Option<Function> {
        match self.locals.get(name) {
            Some(EnvironmentValue::Function(function)) => Some(function.clone()),
            _ => self
                .parent
                .as_ref()
                .and_then(|parent| parent.borrow().get_function_recursive(name)),
        }
    }
}
