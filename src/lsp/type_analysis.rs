use crate::lexer::{TokenKind, tokenize};
use crate::parser::{Expr, ExprKind, Stmt, StmtKind, parse};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeInfo {
    Number,
    String,
    Boolean,
    Array(Box<TypeInfo>),
    Object(HashMap<String, TypeInfo>),
    Function {
        params: Vec<String>,
        return_type: Box<TypeInfo>,
    },
    Unknown,
    Nil,
}

impl TypeInfo {
    pub fn to_string(&self) -> String {
        match self {
            TypeInfo::Number => "Number".to_string(),
            TypeInfo::String => "String".to_string(),
            TypeInfo::Boolean => "Boolean".to_string(),
            TypeInfo::Array(elem) => format!("Array<{}>", elem.to_string()),
            TypeInfo::Object(fields) => {
                let fields_str: Vec<String> = fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("Object {{ {} }}", fields_str.join(", "))
            }
            TypeInfo::Function {
                params,
                return_type,
            } => {
                let params_str = params.join(", ");
                format!("fn({}) -> {}", params_str, return_type.to_string())
            }
            TypeInfo::Unknown => "Unknown".to_string(),
            TypeInfo::Nil => "Nil".to_string(),
        }
    }

    pub fn from_expr(expr: &Expr) -> Self {
        match &expr.kind {
            ExprKind::Number(_) => TypeInfo::Number,
            ExprKind::Boolean(_) => TypeInfo::Boolean,
            ExprKind::String(_) => TypeInfo::String,
            ExprKind::Array(items) => {
                if let Some(first) = items.first() {
                    TypeInfo::Array(Box::new(TypeInfo::from_expr(first)))
                } else {
                    TypeInfo::Array(Box::new(TypeInfo::Unknown))
                }
            }
            ExprKind::Object(fields) => {
                let mut type_fields = HashMap::new();
                for (name, value) in fields {
                    type_fields.insert(name.clone(), TypeInfo::from_expr(value));
                }
                TypeInfo::Object(type_fields)
            }
            ExprKind::FunctionCall { name, args } => {
                if crate::lsp::definition::MpDefinition::new().is_builtin(name) {
                    return Self::builtin_function_return_type(name, args);
                }
                TypeInfo::Unknown
            }
            ExprKind::Variable(_) => TypeInfo::Unknown,
            ExprKind::If {
                then_branch,
                else_branch,
                ..
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
                    then_type
                }
            }
            ExprKind::BinaryOp { left, op, right } => match op {
                TokenKind::Plus => {
                    let l = Self::from_expr(left);
                    let r = Self::from_expr(right);
                    if matches!(l, TypeInfo::String) || matches!(r, TypeInfo::String) {
                        TypeInfo::String
                    } else {
                        TypeInfo::Number
                    }
                }
                TokenKind::Minus
                | TokenKind::Multiply
                | TokenKind::Divide
                | TokenKind::GreaterThan
                | TokenKind::LessThan
                | TokenKind::GreaterThanOrEqual
                | TokenKind::LessThanOrEqual => TypeInfo::Number,
                TokenKind::Equal | TokenKind::NotEqual => TypeInfo::Boolean,
                _ => TypeInfo::Unknown,
            },
            ExprKind::Index { .. } => TypeInfo::Unknown,
            ExprKind::GetProperty { .. } => TypeInfo::Unknown,
            ExprKind::While { .. } => TypeInfo::Nil,
            ExprKind::Parenthesized(expr) => Self::from_expr(expr),
            ExprKind::Block(_) => TypeInfo::Unknown,
            ExprKind::UnaryOp { expr, .. } => Self::from_expr(expr),
        }
    }

    fn builtin_function_return_type(name: &str, args: &[Expr]) -> Self {
        match name {
            "len" => TypeInfo::Number,
            "type" => TypeInfo::String,
            "str" => TypeInfo::String,
            "int" => TypeInfo::Number,
            "float" => TypeInfo::Number,
            "input" => TypeInfo::String,
            "random" => TypeInfo::Number,
            "push" | "pop" => {
                if let Some(ExprKind::Array(items)) = args.first().map(|e| &e.kind) {
                    if let Some(first) = items.first() {
                        return TypeInfo::from_expr(first);
                    }
                }
                TypeInfo::Unknown
            }
            "print" => TypeInfo::Nil,
            _ => TypeInfo::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct TypeAnalyzer {
    variables: HashMap<String, TypeInfo>,
    functions: HashMap<String, (Vec<String>, TypeInfo)>,
}

impl TypeAnalyzer {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, content: &str) {
        if let Ok(tokens) = tokenize(content) {
            if let Ok(ast) = parse(tokens) {
                for stmt in ast {
                    self.analyze_stmt(&stmt);
                }
            }
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Let { name, value } => {
                let var_type = TypeInfo::from_expr(value);
                self.variables.insert(name.clone(), var_type);
            }
            StmtKind::Function { name, params, body } => {
                let return_type = TypeInfo::from_expr(body);
                self.functions
                    .insert(name.clone(), (params.clone(), return_type));
            }
            StmtKind::Expr(expr) => {
                self.analyze_expr(expr);
            }
            _ => {}
        }
    }

    fn analyze_expr(&mut self, expr: &Expr) -> TypeInfo {
        TypeInfo::from_expr(expr)
    }

    pub fn get_variable_type(&self, name: &str) -> Option<TypeInfo> {
        self.variables.get(name).cloned()
    }

    pub fn get_function_info(&self, name: &str) -> Option<(Vec<String>, TypeInfo)> {
        self.functions.get(name).cloned()
    }

    pub fn get_all_variables(&self) -> Vec<(String, TypeInfo)> {
        self.variables
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    pub fn get_all_functions(&self) -> Vec<(String, Vec<String>, TypeInfo)> {
        self.functions
            .iter()
            .map(|(k, (params, ret))| (k.clone(), params.clone(), ret.clone()))
            .collect()
    }
}

impl Default for TypeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
