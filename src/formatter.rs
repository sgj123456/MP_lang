use crate::lexer;
use crate::lexer::{Span, TokenKind};
use crate::parser;
use crate::parser::{Expr, ExprKind, Stmt, StmtKind};

pub struct Formatter {
    indent: usize,
    output: String,
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter {
    pub fn new() -> Self {
        Self {
            indent: 0,
            output: String::new(),
        }
    }

    pub fn format(&mut self, source: &str) -> Result<String, String> {
        let tokens = lexer::tokenize(source).map_err(|e| e.to_string())?;
        let ast = parser::parse(tokens);
        self.format_statements(&ast);
        Ok(self.output.clone())
    }

    fn format_statements(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.format_statement(stmt);
            if !self.output.ends_with('\n') {
                self.output.push('\n');
            }
        }
    }

    fn format_statement(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Let { name, value, .. } => {
                self.add_indent();
                self.output.push_str("let ");
                self.output.push_str(name);
                self.output.push_str(" = ");
                self.format_expr(value);
                self.output.push(';');
            }
            StmtKind::Function { name, params, body } => {
                self.add_indent();
                self.output.push_str("fn ");
                self.output.push_str(name);
                self.output.push('(');
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.output.push_str(param);
                }
                self.output.push_str(") ");
                self.format_expr(body);
            }
            StmtKind::Expr(expr) => {
                self.add_indent();
                self.format_expr(expr);
            }
            StmtKind::Result(expr) => {
                self.add_indent();
                self.format_expr(expr);
            }
            StmtKind::Return(value) => {
                self.add_indent();
                self.output.push_str("return");
                if let Some(expr) = value {
                    self.output.push(' ');
                    self.format_expr(expr);
                }
            }
            StmtKind::Break => {
                self.add_indent();
                self.output.push_str("break");
            }
            StmtKind::Continue => {
                self.add_indent();
                self.output.push_str("continue");
            }
            StmtKind::Struct { name, fields } => {
                self.add_indent();
                self.output.push_str("struct ");
                self.output.push_str(name);
                self.output.push_str(" { ");
                for (i, (field_name, default_value)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.output.push_str(field_name);
                    if let Some(value) = default_value {
                        self.output.push_str(" = ");
                        self.format_expr(value);
                    }
                }
                self.output.push_str(" }");
            }
        }
    }

    fn format_expr(&mut self, expr: &Expr) {
        match &expr.kind {
            ExprKind::Number(n) => {
                self.output.push_str(&n.to_string());
            }
            ExprKind::Boolean(b) => {
                self.output.push_str(if *b { "true" } else { "false" });
            }
            ExprKind::String(s) => {
                self.output.push('"');
                self.output.push_str(s);
                self.output.push('"');
            }
            ExprKind::Variable(name) => {
                self.output.push_str(name);
            }
            ExprKind::Array(elements) => {
                self.output.push('[');
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.format_expr(elem);
                }
                self.output.push(']');
            }
            ExprKind::Object(properties) => {
                self.output.push('{');
                for (i, (key, value)) in properties.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.output.push('"');
                    self.output.push_str(key);
                    self.output.push_str("\": ");
                    self.format_expr(value);
                }
                self.output.push('}');
            }
            ExprKind::Parenthesized(expr) => {
                self.output.push('(');
                self.format_expr(expr);
                self.output.push(')');
            }
            ExprKind::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.output.push_str("if ");
                self.format_expr(condition);
                self.output.push(' ');
                self.format_expr(then_branch);
                if let Some(else_expr) = else_branch {
                    self.output.push_str(" else ");
                    self.format_expr(else_expr);
                }
            }
            ExprKind::While { condition, body } => {
                self.output.push_str("while ");
                self.format_expr(condition);
                self.output.push(' ');
                self.format_expr(body);
            }
            ExprKind::Block(statements) => {
                self.output.push_str("{\n");
                self.indent += 1;
                for stmt in statements {
                    self.format_statement(&Stmt {
                        kind: stmt.clone(),
                        span: Span { line: 0, column: 0 },
                    });
                    if !self.output.ends_with('\n') {
                        self.output.push('\n');
                    }
                }
                self.indent -= 1;
                self.add_indent();
                self.output.push('}');
            }
            ExprKind::BinaryOp { left, op, right } => {
                self.format_expr(left);
                self.output.push(' ');
                self.output.push_str(&token_kind_to_string(op));
                self.output.push(' ');
                self.format_expr(right);
            }
            ExprKind::UnaryOp { op, expr } => {
                self.output.push_str(&token_kind_to_string(op));
                self.format_expr(expr);
            }
            ExprKind::FunctionCall { name, args } => {
                self.output.push_str(name);
                self.output.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.format_expr(arg);
                }
                self.output.push(')');
            }
            ExprKind::Index { object, index } => {
                self.format_expr(object);
                self.output.push('[');
                self.format_expr(index);
                self.output.push(']');
            }
            ExprKind::GetProperty { object, property } => {
                self.format_expr(object);
                self.output.push(':');
                self.output.push_str(property);
            }
            ExprKind::StructInstance { name, args } => {
                self.output.push_str(name);
                self.output.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.format_expr(arg);
                }
                self.output.push(')');
            }
        }
    }

    fn add_indent(&mut self) {
        for _ in 0..self.indent {
            self.output.push_str("    ");
        }
    }
}

fn token_kind_to_string(kind: &TokenKind) -> String {
    match kind {
        TokenKind::Plus => "+".to_string(),
        TokenKind::Minus => "-".to_string(),
        TokenKind::Multiply => "*".to_string(),
        TokenKind::Divide => "/".to_string(),
        TokenKind::Assign => "=".to_string(),
        TokenKind::Equal => "==".to_string(),
        TokenKind::NotEqual => "!=".to_string(),
        TokenKind::LogicalAnd => "&&".to_string(),
        TokenKind::LogicalOr => "||".to_string(),
        TokenKind::Not => "!".to_string(),
        TokenKind::GreaterThan => ">".to_string(),
        TokenKind::GreaterThanOrEqual => ">=".to_string(),
        TokenKind::LessThan => "<".to_string(),
        TokenKind::LessThanOrEqual => "<=".to_string(),
        _ => format!("{:?}", kind),
    }
}

pub fn format_code(source: &str) -> Result<String, String> {
    let mut formatter = Formatter::new();
    formatter.format(source)
}
