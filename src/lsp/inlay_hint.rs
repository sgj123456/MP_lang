use crate::lexer::{TokenKind, tokenize};
use crate::lsp::diagnostics::MpDiagnostics;
use crate::parser::{StmtKind, parse};
use std::collections::HashMap;
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

        let diagnostics = MpDiagnostics::new();
        let (_, variable_types) = diagnostics.analyze(content);

        let var_types_map: HashMap<String, String> = variable_types
            .into_iter()
            .map(|vt| (vt.name, vt.var_type))
            .collect();

        let tokens = match tokenize(content) {
            Ok(tokens) => tokens,
            Err(_) => return hints,
        };

        let ast = parse(tokens);

        self.extract_hints_from_ast(&ast, content, &mut hints, &var_types_map);

        hints
    }

    fn extract_hints_from_ast(
        &self,
        ast: &[crate::parser::Stmt],
        content: &str,
        hints: &mut Vec<InlayHint>,
        var_types: &HashMap<String, String>,
    ) {
        for stmt in ast {
            self.extract_hints_from_stmt(stmt, content, hints, var_types);
        }
    }

    fn extract_hints_from_stmt(
        &self,
        stmt: &crate::parser::Stmt,
        content: &str,
        hints: &mut Vec<InlayHint>,
        var_types: &HashMap<String, String>,
    ) {
        match &stmt.kind {
            StmtKind::Let {
                name,
                name_span,
                value,
            } => {
                let type_label = self.infer_type(value, var_types);
                if !type_label.is_empty()
                    && let Some(token) = self.find_token_in_content(name, content, name_span.line)
                {
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
            StmtKind::Function { name, params, body } => {
                if let Some(token) = self.find_token_in_content(name, content, stmt.span.line) {
                    let param_types: Vec<String> = params.iter().map(|_| "_".to_string()).collect();
                    let return_type = self.infer_return_type(body, var_types);

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
            StmtKind::Expr(expr) => {
                self.extract_hints_from_expr(expr, content, hints, var_types);
            }
            StmtKind::Result(expr) => {
                self.extract_hints_from_expr(expr, content, hints, var_types);
            }
            StmtKind::Return(Some(expr)) => {
                self.extract_hints_from_expr(expr, content, hints, var_types);
            }
            StmtKind::Break
            | StmtKind::Continue
            | StmtKind::Return(None)
            | StmtKind::Struct { .. } => {}
        }
    }

    fn extract_hints_from_expr(
        &self,
        expr: &crate::parser::Expr,
        content: &str,
        hints: &mut Vec<InlayHint>,
        var_types: &HashMap<String, String>,
    ) {
        use crate::parser::ExprKind::*;

        match &expr.kind {
            If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.extract_hints_from_expr(condition, content, hints, var_types);
                self.extract_hints_from_expr(then_branch, content, hints, var_types);
                if let Some(else_b) = else_branch {
                    self.extract_hints_from_expr(else_b, content, hints, var_types);
                }
            }
            While { condition, body } => {
                self.extract_hints_from_expr(condition, content, hints, var_types);
                self.extract_hints_from_expr(body, content, hints, var_types);
            }
            Block(stmts) => {
                for stmt in stmts {
                    let dummy_span = expr.span;
                    let stmt = crate::parser::Stmt {
                        kind: stmt.clone(),
                        span: dummy_span,
                    };
                    self.extract_hints_from_stmt(&stmt, content, hints, var_types);
                }
            }
            BinaryOp { left, right, .. } => {
                self.extract_hints_from_expr(left, content, hints, var_types);
                self.extract_hints_from_expr(right, content, hints, var_types);
            }
            UnaryOp { expr, .. } => {
                self.extract_hints_from_expr(expr, content, hints, var_types);
            }
            FunctionCall { args, .. } => {
                for arg in args {
                    self.extract_hints_from_expr(arg, content, hints, var_types);
                }
            }
            Array(items) => {
                for item in items {
                    self.extract_hints_from_expr(item, content, hints, var_types);
                }
            }
            Object(fields) => {
                for (_, value) in fields {
                    self.extract_hints_from_expr(value, content, hints, var_types);
                }
            }
            Index { object, index } => {
                self.extract_hints_from_expr(object, content, hints, var_types);
                self.extract_hints_from_expr(index, content, hints, var_types);
            }
            GetProperty { object, .. } => {
                self.extract_hints_from_expr(object, content, hints, var_types);
            }
            Parenthesized(e) => {
                self.extract_hints_from_expr(e, content, hints, var_types);
            }
            Number(_) | Boolean(_) | String(_) | Variable(_) | StructInstance { .. } => {}
        }
    }

    fn find_token_in_content(
        &self,
        name: &str,
        content: &str,
        line: usize,
    ) -> Option<crate::lexer::Token> {
        let tokens = tokenize(content).ok()?;

        for token in &tokens {
            if let TokenKind::Identifier(id) = &token.kind
                && id == name
                && token.span.line == line
            {
                return Some(token.clone());
            }
        }
        None
    }

    fn infer_type(
        &self,
        expr: &crate::parser::Expr,
        var_types: &HashMap<String, String>,
    ) -> String {
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
            BinaryOp { op, .. } => match op {
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
            Variable(name) => var_types
                .get(name)
                .cloned()
                .unwrap_or_else(|| "unknown".to_string()),
            Parenthesized(expr) => self.infer_type(expr, var_types),
            If { .. } => "unknown".to_string(),
            While { .. } => "array".to_string(),
            Block(_) => "unknown".to_string(),
            Index { .. } => "unknown".to_string(),
            GetProperty { .. } => "unknown".to_string(),
            UnaryOp { .. } => "unknown".to_string(),
            StructInstance { .. } => "unknown".to_string(),
        }
    }

    fn infer_return_type(
        &self,
        body: &crate::parser::Expr,
        var_types: &HashMap<String, String>,
    ) -> String {
        use crate::parser::ExprKind::*;

        match &body.kind {
            Block(statements) => {
                for stmt in statements {
                    if let StmtKind::Return(Some(expr)) = stmt {
                        return self.infer_type(expr, var_types);
                    }
                }
                if let Some(last) = statements.last() {
                    if let StmtKind::Expr(expr) = last {
                        return self.infer_type(expr, var_types);
                    }
                    if let StmtKind::Result(expr) = last {
                        return self.infer_type(expr, var_types);
                    }
                }
            }
            _ => return self.infer_type(body, var_types),
        }

        "unknown".to_string()
    }

    fn is_builtin_function(&self, name: &str) -> bool {
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
