use crate::lexer::{Span, tokenize};
use crate::parser::{Expr, ExprKind, Stmt, StmtKind, parse_with_errors};
use tower_lsp_server::{Client, ls_types::*};
use std::collections::HashMap;
use std::str::FromStr;

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
        let diagnostics = self.analyze(content);
        let uri = Uri::from_str(uri).unwrap();
        client.publish_diagnostics(uri, diagnostics, None).await;
    }

    pub fn analyze(&self, content: &str) -> Vec<Diagnostic> {
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
                return diagnostics;
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

        let static_diagnostics = self.static_analysis(content);
        diagnostics.extend(static_diagnostics);

        diagnostics
    }

    fn static_analysis(&self, content: &str) -> Vec<Diagnostic> {
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
    fn from_name(name: &str) -> Option<(Self, usize)> {
        match name {
            "len" => Some((Self::Len, 1)),
            "type" => Some((Self::Type, 1)),
            "str" => Some((Self::Str, 1)),
            "int" => Some((Self::Int, 1)),
            "float" => Some((Self::Float, 1)),
            "input" => Some((Self::Input, 0)),
            "random" => Some((Self::Random, 0)),
            "push" => Some((Self::Push, 2)),
            "pop" => Some((Self::Pop, 1)),
            "print" => Some((Self::Print, 1)),
            "println" => Some((Self::Println, 1)),
            "readline" => Some((Self::ReadLine, 0)),
            _ => None,
        }
    }
}

struct StaticAnalyzer {
    variables: HashMap<String, Span>,
    functions: HashMap<String, (Span, Vec<String>)>,
}

impl StaticAnalyzer {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn analyze(&mut self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        let tokens = match tokenize(content) {
            Ok(tokens) => tokens,
            Err(_) => return diagnostics,
        };

        let (ast, _) = parse_with_errors(tokens);

        self.collect_definitions(&ast, &mut diagnostics);

        self.check_usages(&ast, &mut diagnostics);

        diagnostics
    }

    fn collect_definitions(&mut self, ast: &[Stmt], diagnostics: &mut Vec<Diagnostic>) {
        for stmt in ast {
            match &stmt.kind {
                StmtKind::Let { name, .. } => {
                    if self.variables.contains_key(name) {
                        diagnostics.push(Diagnostic {
                            range: self.span_to_range(&stmt.span),
                            severity: Some(DiagnosticSeverity::WARNING),
                            code: Some(NumberOrString::String("MP003".to_string())),
                            source: Some("mp-lang".to_string()),
                            message: format!("Variable '{}' is already defined", name),
                            ..Default::default()
                        });
                    }
                    self.variables.insert(name.clone(), stmt.span);
                }
                StmtKind::Function { name, params, .. } => {
                    if self.functions.contains_key(name)
                        && let Some((_first_span, _)) = self.functions.get(name) {
                            diagnostics.push(Diagnostic {
                                range: self.span_to_range(&stmt.span),
                                severity: Some(DiagnosticSeverity::WARNING),
                                code: Some(NumberOrString::String("MP004".to_string())),
                                source: Some("mp-lang".to_string()),
                                message: format!("Function '{}' is already defined", name),
                                ..Default::default()
                            });
                        }
                    self.functions.insert(name.clone(), (stmt.span, params.clone()));
                }
                _ => {}
            }
        }
    }

    fn check_usages(&self, ast: &[Stmt], diagnostics: &mut Vec<Diagnostic>) {
        for stmt in ast {
            self.check_stmt(stmt, diagnostics);
        }
    }

    fn check_stmt(&self, stmt: &Stmt, diagnostics: &mut Vec<Diagnostic>) {
        match &stmt.kind {
            StmtKind::Let { name: _, value } => {
                self.check_expr(value, diagnostics);
            }
            StmtKind::Function { name: _, params: _, body } => {
                self.check_expr(body, diagnostics);
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
            StmtKind::Break | StmtKind::Continue | StmtKind::Return(None) => {}
        }
    }

    fn check_expr(&self, expr: &Expr, diagnostics: &mut Vec<Diagnostic>) {
        match &expr.kind {
            ExprKind::Variable(name) => {
                if !self.variables.contains_key(name) && !self.functions.contains_key(name)
                    && BuiltinFunction::from_name(name).is_none() {
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
                    if args.len() != expected_args {
                        let severity = if args.len() < expected_args {
                            DiagnosticSeverity::ERROR
                        } else {
                            DiagnosticSeverity::WARNING
                        };
                        diagnostics.push(Diagnostic {
                            range: self.span_to_range(&expr.span),
                            severity: Some(severity),
                            code: Some(NumberOrString::String("MP006".to_string())),
                            source: Some("mp-lang".to_string()),
                            message: format!(
                                "Builtin function '{}' expects {} argument(s), got {}",
                                name, expected_args, args.len()
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
                    && args.len() != params.len() {
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
                                name, params.len(), args.len()
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
            ExprKind::If { condition, then_branch, else_branch } => {
                self.check_expr(condition, diagnostics);
                self.check_expr(then_branch, diagnostics);
                if let Some(else_b) = else_branch {
                    self.check_expr(else_b, diagnostics);
                }
            }
            ExprKind::While { condition, body } => {
                self.check_expr(condition, diagnostics);
                self.check_expr(body, diagnostics);
            }
            ExprKind::Block(stmts) => {
                for stmt_kind in stmts {
                    let dummy_span = expr.span;
                    let stmt = Stmt {
                        kind: stmt_kind.clone(),
                        span: dummy_span,
                    };
                    self.check_stmt(&stmt, diagnostics);
                }
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
            ExprKind::Number(_) | ExprKind::Boolean(_) | ExprKind::String(_) => {}
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
