use crate::lexer::{Span, tokenize};
use crate::parser::{Expr, ExprKind, Stmt, StmtKind, parse_with_errors};
use std::collections::HashMap;
use std::str::FromStr;
use tower_lsp_server::{Client, ls_types::*};

#[derive(Debug)]
pub struct VariableType {
    pub name: String,
    pub span: Span,
    pub var_type: String,
}

#[derive(Debug)]
pub struct MpDiagnostics;

impl Default for MpDiagnostics {
    fn default() -> Self {
        Self::new()
    }
}

impl MpDiagnostics {
    pub fn new() -> Self {
        Self
    }

    pub async fn publish(&self, client: &Client, uri: &str, content: &str) {
        let (diagnostics, _) = self.analyze(content);
        let uri = Uri::from_str(uri).unwrap();
        client.publish_diagnostics(uri, diagnostics, None).await;
    }

    pub fn analyze(&self, content: &str) -> (Vec<Diagnostic>, Vec<VariableType>) {
        let mut diagnostics = Vec::new();

        let tokens = match tokenize(content) {
            Ok(tokens) => tokens,
            Err(e) => {
                diagnostics.push(Diagnostic {
                    range: self.span_to_range(&e.span()),
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String("MP001".to_string())),
                    source: Some("mp-lang".to_string()),
                    message: format!("Lexer error: {}", e.message()),
                    ..Default::default()
                });
                return (diagnostics, Vec::new());
            }
        };

        let (_, errors) = parse_with_errors(tokens);
        for e in &errors {
            diagnostics.push(Diagnostic {
                range: self.span_to_range(&e.span()),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("MP002".to_string())),
                source: Some("mp-lang".to_string()),
                message: format!("Parser error: {}", e),
                ..Default::default()
            });
        }

        let (static_diagnostics, variable_types) = self.static_analysis(content);
        diagnostics.extend(static_diagnostics);

        (diagnostics, variable_types)
    }

    fn static_analysis(&self, content: &str) -> (Vec<Diagnostic>, Vec<VariableType>) {
        let mut analyzer = StaticAnalyzer::new();
        analyzer.analyze(content)
    }

    fn span_to_range(&self, span: &Span) -> Range {
        Range {
            start: Position {
                line: (span.line.saturating_sub(1)) as u32,
                character: (span.column.saturating_sub(1)) as u32,
            },
            end: Position {
                line: (span.line.saturating_sub(1)) as u32,
                character: span.column as u32,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BuiltinFunction {
    Len,
    Type,
    Str,
    Int,
    Float,
    Input,
    Random,
    Push,
    Pop,
    Print,
    Println,
    ReadLine,
}

impl BuiltinFunction {
    fn from_name(name: &str) -> Option<(Self, std::ops::RangeInclusive<usize>)> {
        match name {
            "len" => Some((Self::Len, 1..=1)),
            "type" => Some((Self::Type, 1..=1)),
            "str" => Some((Self::Str, 1..=1)),
            "int" => Some((Self::Int, 1..=1)),
            "float" => Some((Self::Float, 1..=1)),
            "input" => Some((Self::Input, 0..=0)),
            "random" => Some((Self::Random, 0..=2)),
            "push" => Some((Self::Push, 2..=2)),
            "pop" => Some((Self::Pop, 1..=1)),
            "print" => Some((Self::Print, 1..=1)),
            "println" => Some((Self::Println, 1..=1)),
            "readline" => Some((Self::ReadLine, 0..=0)),
            _ => None,
        }
    }
}

struct VariableInfo {
    span: Span,
    var_type: String,
}

struct StaticAnalyzer {
    scopes: Vec<HashMap<String, VariableInfo>>,
    functions: HashMap<String, (Span, Vec<String>)>,
}

impl StaticAnalyzer {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            functions: HashMap::new(),
        }
    }

    fn analyze(&mut self, content: &str) -> (Vec<Diagnostic>, Vec<VariableType>) {
        let mut diagnostics = Vec::new();

        let tokens = match tokenize(content) {
            Ok(tokens) => tokens,
            Err(_) => return (diagnostics, Vec::new()),
        };

        let (ast, _) = parse_with_errors(tokens);

        self.collect_definitions(&ast, &mut diagnostics);

        self.check_usages(&ast, &mut diagnostics);

        let variable_types = self.collect_variable_types();

        (diagnostics, variable_types)
    }

    fn collect_variable_types(&self) -> Vec<VariableType> {
        let mut variable_types = Vec::new();

        for scope in &self.scopes {
            for (name, info) in scope {
                variable_types.push(VariableType {
                    name: name.clone(),
                    span: info.span,
                    var_type: info.var_type.clone(),
                });
            }
        }

        variable_types
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    fn add_variable(&mut self, name: &str, span: Span, var_type: String) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(name) {
                return;
            }
            scope.insert(name.to_string(), VariableInfo { span, var_type });
        }
    }

    fn get_variable_type(&self, name: &str) -> Option<&String> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(&info.var_type);
            }
        }
        None
    }

    fn contains_variable(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(name) {
                return true;
            }
        }
        false
    }

    fn collect_definitions(&mut self, ast: &[Stmt], diagnostics: &mut Vec<Diagnostic>) {
        for stmt in ast {
            self.collect_stmt_definitions(stmt, diagnostics);
        }
    }

    fn infer_type(&self, expr: &Expr) -> String {
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
                if let Some((_builtin, _)) = BuiltinFunction::from_name(name) {
                    self.get_builtin_return_type(name)
                } else {
                    "function".to_string()
                }
            }
            BinaryOp { op, .. } => match op {
                crate::lexer::TokenKind::Plus
                | crate::lexer::TokenKind::Minus
                | crate::lexer::TokenKind::Multiply
                | crate::lexer::TokenKind::Divide => "number".to_string(),
                crate::lexer::TokenKind::Equal
                | crate::lexer::TokenKind::NotEqual
                | crate::lexer::TokenKind::GreaterThan
                | crate::lexer::TokenKind::LessThan
                | crate::lexer::TokenKind::GreaterThanOrEqual
                | crate::lexer::TokenKind::LessThanOrEqual => "bool".to_string(),
                _ => "unknown".to_string(),
            },
            Variable(name) => self
                .get_variable_type(name)
                .cloned()
                .unwrap_or_else(|| "unknown".to_string()),
            Parenthesized(expr) => self.infer_type(expr),
            If { .. } => "unknown".to_string(),
            While { .. } => "array".to_string(),
            Block(_) => "unknown".to_string(),
            Index { .. } => "unknown".to_string(),
            GetProperty { .. } => "unknown".to_string(),
            UnaryOp { .. } => "unknown".to_string(),
            StructInstance { .. } => "unknown".to_string(),
        }
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

    fn collect_stmt_definitions(&mut self, stmt: &Stmt, diagnostics: &mut Vec<Diagnostic>) {
        match &stmt.kind {
            StmtKind::Let {
                name,
                name_span,
                value,
            } => {
                if self
                    .scopes
                    .last()
                    .map(|s| s.contains_key(name))
                    .unwrap_or(false)
                {
                    diagnostics.push(Diagnostic {
                        range: self.span_to_range(name_span),
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: Some(NumberOrString::String("MP003".to_string())),
                        source: Some("mp-lang".to_string()),
                        message: format!("Variable '{}' is already defined", name),
                        ..Default::default()
                    });
                }
                let var_type = self.infer_type(value);
                self.add_variable(name, *name_span, var_type);
            }
            StmtKind::Function { name, params, body } => {
                if self.functions.contains_key(name)
                    && let Some((_first_span, _)) = self.functions.get(name)
                {
                    diagnostics.push(Diagnostic {
                        range: self.span_to_range(&stmt.span),
                        severity: Some(DiagnosticSeverity::WARNING),
                        code: Some(NumberOrString::String("MP004".to_string())),
                        source: Some("mp-lang".to_string()),
                        message: format!("Function '{}' is already defined", name),
                        ..Default::default()
                    });
                }
                self.functions
                    .insert(name.clone(), (stmt.span, params.clone()));
                self.push_scope();
                for param in params {
                    self.add_variable(param, stmt.span, "unknown".to_string());
                }
                self.collect_expr_definitions(body);
                self.pop_scope();
            }
            StmtKind::Expr(expr) => {
                self.collect_expr_definitions(expr);
            }
            StmtKind::Result(expr) => {
                self.collect_expr_definitions(expr);
            }
            StmtKind::Return(Some(expr)) => {
                self.collect_expr_definitions(expr);
            }
            StmtKind::Break
            | StmtKind::Continue
            | StmtKind::Return(None)
            | StmtKind::Struct { .. } => {}
        }
    }

    fn collect_expr_definitions(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.collect_expr_definitions(condition);
                self.push_scope();
                self.collect_expr_definitions(then_branch);
                self.pop_scope();
                if let Some(else_b) = else_branch {
                    self.push_scope();
                    self.collect_expr_definitions(else_b);
                    self.pop_scope();
                }
            }
            ExprKind::While { condition, body } => {
                self.collect_expr_definitions(condition);
                self.push_scope();
                self.collect_expr_definitions(body);
                self.pop_scope();
            }
            ExprKind::Block(stmts) => {
                self.push_scope();
                for stmt_kind in stmts {
                    let dummy_span = expr.span;
                    let stmt = Stmt {
                        kind: stmt_kind.clone(),
                        span: dummy_span,
                    };
                    self.collect_stmt_definitions(&stmt, &mut Vec::new());
                }
                self.pop_scope();
            }
            _ => {
                for child in expr.children() {
                    self.collect_expr_definitions(child);
                }
            }
        }
    }

    fn check_usages(&mut self, ast: &[Stmt], diagnostics: &mut Vec<Diagnostic>) {
        for stmt in ast {
            self.check_stmt(stmt, diagnostics);
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt, diagnostics: &mut Vec<Diagnostic>) {
        match &stmt.kind {
            StmtKind::Let { name, value, .. } => {
                let var_type = self.infer_type(value);
                self.add_variable(name, stmt.span, var_type);
                self.check_expr(value, diagnostics);
            }
            StmtKind::Function {
                name: _,
                params,
                body,
            } => {
                self.push_scope();
                for param in params {
                    self.add_variable(param, stmt.span, "unknown".to_string());
                }
                self.check_expr(body, diagnostics);
                self.pop_scope();
            }
            StmtKind::Expr(expr) => {
                self.check_expr(expr, diagnostics);
            }
            StmtKind::Result(expr) => {
                self.check_expr(expr, diagnostics);
            }
            StmtKind::Return(Some(expr)) => {
                self.check_expr(expr, diagnostics);
            }
            StmtKind::Break
            | StmtKind::Continue
            | StmtKind::Return(None)
            | StmtKind::Struct { .. } => {}
        }
    }

    fn check_expr(&mut self, expr: &Expr, diagnostics: &mut Vec<Diagnostic>) {
        match &expr.kind {
            ExprKind::Variable(name) => {
                if !self.contains_variable(name)
                    && !self.functions.contains_key(name)
                    && BuiltinFunction::from_name(name).is_none()
                {
                    diagnostics.push(Diagnostic {
                        range: self.span_to_range(&expr.span),
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: Some(NumberOrString::String("MP005".to_string())),
                        source: Some("mp-lang".to_string()),
                        message: format!("Undefined variable or function: '{}'", name),
                        ..Default::default()
                    });
                }
            }
            ExprKind::FunctionCall { name, args } => {
                if let Some((_builtin, expected_args)) = BuiltinFunction::from_name(name) {
                    if !expected_args.contains(&args.len()) {
                        let severity = if args.len() < *expected_args.start() {
                            DiagnosticSeverity::ERROR
                        } else {
                            DiagnosticSeverity::WARNING
                        };
                        let expected_str = if expected_args.start() == expected_args.end() {
                            format!("{}", expected_args.start())
                        } else {
                            format!("{} to {}", expected_args.start(), expected_args.end())
                        };
                        diagnostics.push(Diagnostic {
                            range: self.span_to_range(&expr.span),
                            severity: Some(severity),
                            code: Some(NumberOrString::String("MP006".to_string())),
                            source: Some("mp-lang".to_string()),
                            message: format!(
                                "Builtin function '{}' expects {} argument(s), got {}",
                                name,
                                expected_str,
                                args.len()
                            ),
                            ..Default::default()
                        });
                    }
                } else if !self.functions.contains_key(name) {
                    diagnostics.push(Diagnostic {
                        range: self.span_to_range(&expr.span),
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: Some(NumberOrString::String("MP007".to_string())),
                        source: Some("mp-lang".to_string()),
                        message: format!("Undefined function: '{}'", name),
                        ..Default::default()
                    });
                } else if let Some((_, params)) = self.functions.get(name)
                    && args.len() != params.len()
                {
                    let severity = if args.len() < params.len() {
                        DiagnosticSeverity::ERROR
                    } else {
                        DiagnosticSeverity::WARNING
                    };
                    diagnostics.push(Diagnostic {
                        range: self.span_to_range(&expr.span),
                        severity: Some(severity),
                        code: Some(NumberOrString::String("MP008".to_string())),
                        source: Some("mp-lang".to_string()),
                        message: format!(
                            "Function '{}' expects {} argument(s), got {}",
                            name,
                            params.len(),
                            args.len()
                        ),
                        ..Default::default()
                    });
                }

                for arg in args {
                    self.check_expr(arg, diagnostics);
                }
            }
            ExprKind::BinaryOp { left, right, .. } => {
                self.check_expr(left, diagnostics);
                self.check_expr(right, diagnostics);
            }
            ExprKind::UnaryOp { expr, .. } => {
                self.check_expr(expr, diagnostics);
            }
            ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.check_expr(condition, diagnostics);
                self.push_scope();
                self.check_expr(then_branch, diagnostics);
                self.pop_scope();
                if let Some(else_b) = else_branch {
                    self.push_scope();
                    self.check_expr(else_b, diagnostics);
                    self.pop_scope();
                }
            }
            ExprKind::While { condition, body } => {
                self.check_expr(condition, diagnostics);
                self.push_scope();
                self.check_expr(body, diagnostics);
                self.pop_scope();
            }
            ExprKind::Block(stmts) => {
                self.push_scope();
                for stmt_kind in stmts {
                    let dummy_span = expr.span;
                    let stmt = Stmt {
                        kind: stmt_kind.clone(),
                        span: dummy_span,
                    };
                    self.check_stmt(&stmt, diagnostics);
                }
                self.pop_scope();
            }
            ExprKind::Array(items) => {
                for item in items {
                    self.check_expr(item, diagnostics);
                }
            }
            ExprKind::Object(fields) => {
                for (_, value) in fields {
                    self.check_expr(value, diagnostics);
                }
            }
            ExprKind::Index { object, index } => {
                self.check_expr(object, diagnostics);
                self.check_expr(index, diagnostics);
            }
            ExprKind::GetProperty { object, .. } => {
                self.check_expr(object, diagnostics);
            }
            ExprKind::Parenthesized(expr) => {
                self.check_expr(expr, diagnostics);
            }
            ExprKind::Number(_)
            | ExprKind::Boolean(_)
            | ExprKind::String(_)
            | ExprKind::StructInstance { .. } => {}
        }
    }

    fn span_to_range(&self, span: &Span) -> Range {
        Range {
            start: Position {
                line: (span.line.saturating_sub(1)) as u32,
                character: (span.column.saturating_sub(1)) as u32,
            },
            end: Position {
                line: (span.line.saturating_sub(1)) as u32,
                character: span.column as u32,
            },
        }
    }
}
