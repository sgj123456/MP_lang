use crate::parser::{Expr, ExprKind, Stmt, StmtKind};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeInfo {
    Number,
    String,
    Boolean,
    Array(Box<TypeInfo>),
    Object(HashMap<String, TypeInfo>),
    Function,
    Nil,
    Unknown,
    Custom(String),
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeInfo::Number => write!(f, "Number"),
            TypeInfo::String => write!(f, "String"),
            TypeInfo::Boolean => write!(f, "Boolean"),
            TypeInfo::Array(elem) => write!(f, "Array< {}>", elem),
            TypeInfo::Object(fields) => {
                let fields_str: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "Object {{ {} }}", fields_str.join(", "))
            }
            TypeInfo::Function => write!(f, "Function"),
            TypeInfo::Nil => write!(f, "Nil"),
            TypeInfo::Unknown => write!(f, "Unknown"),
            TypeInfo::Custom(name) => write!(f, "{}", name),
        }
    }
}

impl TypeInfo {
    pub fn from_expr(expr: &Expr) -> Self {
        match &expr.kind {
            ExprKind::Number(_) => TypeInfo::Number,
            ExprKind::Boolean(_) => TypeInfo::Boolean,
            ExprKind::String(_) => TypeInfo::String,
            ExprKind::Array(_) => TypeInfo::Array(Box::new(TypeInfo::Unknown)),
            ExprKind::Object(fields) => {
                let mut type_fields = HashMap::new();
                for (key, value) in fields {
                    type_fields.insert(key.clone(), Self::from_expr(value));
                }
                TypeInfo::Object(type_fields)
            }
            ExprKind::Variable(_) => TypeInfo::Unknown,
            ExprKind::Parenthesized(expr) => Self::from_expr(expr),
            ExprKind::If {
                condition: _,
                then_branch,
                else_branch,
            } => {
                let then_type = Self::from_expr(then_branch);
                if let Some(else_b) = else_branch {
                    let else_type = Self::from_expr(else_b);
                    if then_type == else_type {
                        then_type
                    } else {
                        TypeInfo::Unknown
                    }
                } else {
                    TypeInfo::Nil
                }
            }
            ExprKind::BinaryOp { left, right, .. } => {
                let left_type = Self::from_expr(left);
                let right_type = Self::from_expr(right);
                if left_type == right_type {
                    left_type
                } else {
                    TypeInfo::Unknown
                }
            }
            ExprKind::UnaryOp { expr, .. } => Self::from_expr(expr),
            ExprKind::FunctionCall { .. } => TypeInfo::Function,
            ExprKind::While { .. } => TypeInfo::Nil,
            ExprKind::GetProperty { .. } => TypeInfo::Unknown,
            ExprKind::Index { .. } => TypeInfo::Unknown,
            ExprKind::Block(_) => TypeInfo::Unknown,
            ExprKind::StructInstance { name, .. } => TypeInfo::Custom(name.clone()),
        }
    }
}

pub struct TypeAnalyzer {
    pub variables: HashMap<String, TypeInfo>,
    pub functions: HashMap<String, (Vec<String>, TypeInfo)>,
    pub structs: HashMap<String, Vec<(String, TypeInfo)>>,
}

impl Default for TypeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeAnalyzer {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, ast: Vec<Stmt>) {
        for stmt in ast {
            self.analyze_stmt(&stmt);
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Let { name, value, .. } => {
                let var_type = TypeInfo::from_expr(value);
                self.variables.insert(name.clone(), var_type);
            }
            StmtKind::Function { name, params, body } => {
                let return_type = TypeInfo::from_expr(body);
                self.functions
                    .insert(name.clone(), (params.clone(), return_type));
            }
            StmtKind::Struct { name, fields } => {
                let mut type_fields = Vec::new();
                for (field_name, default_value) in fields {
                    let field_type = match default_value {
                        Some(expr) => TypeInfo::from_expr(expr),
                        None => TypeInfo::Unknown,
                    };
                    type_fields.push((field_name.clone(), field_type));
                }
                self.structs.insert(name.clone(), type_fields);
            }
            StmtKind::Expr(expr) => {
                self.analyze_expr(expr);
            }
            _ => {}
        }
    }

    fn analyze_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Variable(name) => {
                if let Some(var_type) = self.variables.get(name) {
                    let _ = var_type;
                }
            }
            ExprKind::FunctionCall { name, args } => {
                for arg in args {
                    self.analyze_expr(arg);
                }
                if let Some((params, return_type)) = self.functions.get(name) {
                    let _ = (params, return_type);
                }
            }
            ExprKind::BinaryOp { left, right, .. } => {
                self.analyze_expr(left);
                self.analyze_expr(right);
            }
            ExprKind::UnaryOp { expr, .. } => {
                self.analyze_expr(expr);
            }
            ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.analyze_expr(condition);
                self.analyze_expr(then_branch);
                if let Some(else_b) = else_branch {
                    self.analyze_expr(else_b);
                }
            }
            ExprKind::While { condition, body } => {
                self.analyze_expr(condition);
                self.analyze_expr(body);
            }
            ExprKind::Array(elements) => {
                for element in elements {
                    self.analyze_expr(element);
                }
            }
            ExprKind::Object(fields) => {
                for (_, value) in fields {
                    self.analyze_expr(value);
                }
            }
            ExprKind::Index { object, index } => {
                self.analyze_expr(object);
                self.analyze_expr(index);
            }
            ExprKind::GetProperty { object, .. } => {
                self.analyze_expr(object);
            }
            ExprKind::StructInstance { name, args } => {
                for arg in args {
                    self.analyze_expr(arg);
                }
                if let Some(struct_fields) = self.structs.get(name) {
                    let _ = struct_fields;
                }
            }
            _ => {}
        }
    }
}
