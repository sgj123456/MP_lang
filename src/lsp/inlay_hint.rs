use crate::lexer::{TokenKind, tokenize};
use crate::parser::{parse, StmtKind};
use tower_lsp_server::ls_types::*;

#[derive(Debug)]
pub struct MpInlayHints;

impl Default for MpInlayHints {
    fn default() -> Self {
        Self::new()
    }
}

impl MpInlayHints {
    pub fn new() -> Self {
        Self
    }

    pub fn provide(&self, content: &str) -> Vec<InlayHint> {
        let mut hints = Vec::new();

        let tokens = match tokenize(content) {
            Ok(tokens) => tokens,
            Err(_) => return hints,
        };

        let ast = parse(tokens);

        for stmt in &ast {
            self.extract_hints_from_stmt(stmt, content, &mut hints);
        }

        hints
    }

    fn extract_hints_from_stmt(
        &self,
        stmt: &crate::parser::Stmt,
        content: &str,
        hints: &mut Vec<InlayHint>,
    ) {
        match &stmt.kind {
            StmtKind::Let { name, value } => {
                if let Some(token) = self.find_token_in_content(name, content) {
                    let type_label = self.infer_type(value);
                    if !type_label.is_empty() {
                        hints.push(InlayHint {
                            position: Position {
                                line: (token.span.line - 1) as u32,
                                character: (token.span.column + name.len()) as u32,
                            },
                            label: InlayHintLabel::String(format!(" : {}", type_label)),
                            kind: Some(InlayHintKind::TYPE),
                            text_edits: None,
                            tooltip: Some(InlayHintTooltip::String(type_label.clone())),
                            padding_left: Some(false),
                            padding_right: Some(true),
                            data: None,
                        });
                    }
                }
            }
            StmtKind::Function { name, params, body } => {
                if let Some(token) = self.find_token_in_content(name, content) {
                    let param_types: Vec<String> = params.iter().map(|_| "_".to_string()).collect();
                    let return_type = self.infer_return_type(body);
                    
                    let type_label = if return_type.is_empty() {
                        format!("({})", param_types.join(", "))
                    } else {
                        format!("({}) -> {}", param_types.join(", "), return_type)
                    };

                    hints.push(InlayHint {
                        position: Position {
                            line: (token.span.line - 1) as u32,
                            character: (token.span.column + name.len()) as u32,
                        },
                        label: InlayHintLabel::String(type_label),
                        kind: Some(InlayHintKind::TYPE),
                        text_edits: None,
                        tooltip: Some(InlayHintTooltip::String("Function signature".to_string())),
                        padding_left: Some(false),
                        padding_right: Some(true),
                        data: None,
                    });
                }
            }
            _ => {}
        }
    }

    fn find_token_in_content(&self, name: &str, content: &str) -> Option<crate::lexer::Token> {
        let tokens = tokenize(content).ok()?;
        
        for token in &tokens {
            if let TokenKind::Identifier(id) = &token.kind
                && id == name {
                    return Some(token.clone());
                }
        }
        None
    }

    fn infer_type(&self, expr: &crate::parser::Expr) -> String {
        use crate::parser::ExprKind::*;
        
        match &expr.kind {
            Number(n) => match n {
                crate::runtime::environment::value::Number::Int(_) => "int".to_string(),
                crate::runtime::environment::value::Number::Float(_) => "float".to_string(),
            },
            Boolean(_) => "bool".to_string(),
            String(_) => "string".to_string(),
            Array(_) => "array".to_string(),
            Object(_) => "object".to_string(),
            FunctionCall { name, .. } => {
                if self.is_builtin_function(name) {
                    self.get_builtin_return_type(name)
                } else {
                    "function".to_string()
                }
            }
            BinaryOp { op, .. } => {
                match op {
                    TokenKind::Plus | TokenKind::Minus | TokenKind::Multiply | TokenKind::Divide => {
                        "number".to_string()
                    }
                    TokenKind::Equal | TokenKind::NotEqual 
                    | TokenKind::GreaterThan | TokenKind::LessThan
                    | TokenKind::GreaterThanOrEqual | TokenKind::LessThanOrEqual => {
                        "bool".to_string()
                    }
                    _ => "unknown".to_string()
                }
            }
            Variable(_) => "unknown".to_string(),
            Parenthesized(expr) => self.infer_type(expr),
            If { .. } => "unknown".to_string(),
            While { .. } => "array".to_string(),
            Block(_) => "unknown".to_string(),
            Index { .. } => "unknown".to_string(),
            GetProperty { .. } => "unknown".to_string(),
            UnaryOp { .. } => "unknown".to_string(),
        }
    }

    fn infer_return_type(&self, body: &crate::parser::Expr) -> String {
        use crate::parser::ExprKind::*;
        
        match &body.kind {
            Block(statements) => {
                for stmt in statements {
                    if let StmtKind::Return(Some(expr)) = stmt {
                        return self.infer_type(expr);
                    }
                }
                if let Some(last) = statements.last() {
                    if let StmtKind::Expr(expr) = last {
                        return self.infer_type(expr);
                    }
                    if let StmtKind::Result(expr) = last {
                        return self.infer_type(expr);
                    }
                }
            }
            _ => return self.infer_type(body),
        }
        
        "unknown".to_string()
    }

    fn is_builtin_function(&self, name: &str) -> bool {
        matches!(
            name,
            "print" | "input" | "len" | "type" | "str" 
            | "int" | "float" | "random" | "push" | "pop" | "time"
        )
    }

    fn get_builtin_return_type(&self, name: &str) -> String {
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
}
