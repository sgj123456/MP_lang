use crate::lexer::TokenKind;
use crate::parser::ExprKind;

pub fn get_builtin_return_type(name: &str) -> String {
    match name {
        "print" | "push" | "pop" | "time" => "nil".to_string(),
        "input" => "string".to_string(),
        "len" => "int".to_string(),
        "type" | "str" => "string".to_string(),
        "int" | "float" => "number".to_string(),
        "random" => "int".to_string(),
        _ => "unknown".to_string(),
    }
}

pub fn infer_type(expr: &crate::parser::Expr) -> String {
    match &expr.kind {
        ExprKind::Number(n) => match n {
            crate::runtime::environment::value::Number::Int(_) => "int".to_string(),
            crate::runtime::environment::value::Number::Float(_) => "float".to_string(),
        },
        ExprKind::Boolean(_) => "bool".to_string(),
        ExprKind::String(_) => "string".to_string(),
        ExprKind::Array(_) => "array".to_string(),
        ExprKind::Object(_) => "object".to_string(),
        ExprKind::FunctionCall { name, .. } => {
            if is_builtin_function(name) {
                get_builtin_return_type(name)
            } else {
                "function".to_string()
            }
        }
        ExprKind::BinaryOp { op, .. } => match op {
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Multiply
            | TokenKind::Divide
            | TokenKind::Modulo => "number".to_string(),
            TokenKind::Equal
            | TokenKind::NotEqual
            | TokenKind::GreaterThan
            | TokenKind::LessThan
            | TokenKind::GreaterThanOrEqual
            | TokenKind::LessThanOrEqual => "bool".to_string(),
            _ => "unknown".to_string(),
        },
        ExprKind::Variable(_) => "unknown".to_string(),
        ExprKind::Parenthesized(expr) => infer_type(expr),
        ExprKind::If { .. } => "unknown".to_string(),
        ExprKind::While { .. } => "array".to_string(),
        ExprKind::Block(_) => "unknown".to_string(),
        ExprKind::Index { .. } => "unknown".to_string(),
        ExprKind::GetProperty { .. } => "unknown".to_string(),
        ExprKind::UnaryOp { .. } => "unknown".to_string(),
        ExprKind::StructInstance { .. } => "unknown".to_string(),
    }
}

pub fn is_builtin_function(name: &str) -> bool {
    matches!(
        name,
        "print"
            | "input"
            | "len"
            | "type"
            | "str"
            | "int"
            | "float"
            | "random"
            | "push"
            | "pop"
            | "time"
    )
}
