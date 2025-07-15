use crate::{
    ast::Expr,
    runtime::environment::{builtin_function::BuiltinFunction, value::Value},
};

#[derive(Debug, Clone)]
pub enum EnvironmentValue {
    Variable(Value),
    Function { params: Vec<String>, body: Expr },
    Builtin(BuiltinFunction),
}
