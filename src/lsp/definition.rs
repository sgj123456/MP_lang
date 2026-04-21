use crate::lexer::{TokenKind, tokenize_with_errors};
use crate::parser::{Stmt, StmtKind, parse};
use std::str::FromStr;
use tower_lsp_server::ls_types::*;

#[derive(Debug)]
pub struct MpDefinition;

impl Default for MpDefinition {
    fn default() -> Self {
        Self::new()
    }
}

impl MpDefinition {
    pub fn new() -> Self {
        Self
    }

    pub fn goto_definition(
        &self,
        content: &str,
        position: Position,
        uri: &str,
    ) -> Option<GotoDefinitionResponse> {
        let (tokens, errors) = tokenize_with_errors(content);

        if !errors.is_empty() {
            return None;
        }

        let line = position.line as usize + 1;
        let col = position.character as usize + 1;

        let target_name = self.find_identifier_at(&tokens, line, col)?;

        if self.is_builtin(&target_name) {
            return None;
        }

        let symbols = self.build_symbol_table(&tokens, content);

        if let Some(symbol_infos) = symbols.get(&target_name) {
            let mut candidates: Vec<&SymbolInfo> = symbol_infos
                .iter()
                .filter(|s| s.line < line || (s.line == line && s.column <= col))
                .collect();

            if let Some(best) = candidates.pop() {
                let location = Location {
                    uri: Uri::from_str(uri).unwrap(),
                    range: Range {
                        start: Position {
                            line: (best.line - 1) as u32,
                            character: (best.column - 1) as u32,
                        },
                        end: Position {
                            line: (best.line - 1) as u32,
                            character: (best.column + target_name.len() - 1) as u32,
                        },
                    },
                };
                return Some(GotoDefinitionResponse::Scalar(location));
            }
        }

        self.find_function_call_definition(&tokens, &target_name, uri)
    }

    fn find_function_call_definition(
        &self,
        tokens: &[crate::lexer::Token],
        target_name: &str,
        uri: &str,
    ) -> Option<GotoDefinitionResponse> {
        let ast = parse(tokens.to_vec());

        for stmt in &ast {
            if let StmtKind::Function { name, .. } = &stmt.kind
                && name == target_name
                && let Some(token) = self.find_token_by_name(name, tokens)
            {
                let location = Location {
                    uri: Uri::from_str(uri).unwrap(),
                    range: Range {
                        start: Position {
                            line: (token.span.line - 1) as u32,
                            character: (token.span.column - 1) as u32,
                        },
                        end: Position {
                            line: (token.span.line - 1) as u32,
                            character: (token.span.column + name.len() - 1) as u32,
                        },
                    },
                };
                return Some(GotoDefinitionResponse::Scalar(location));
            }
        }

        None
    }

    pub fn references(
        &self,
        content: &str,
        position: Position,
        uri: &str,
    ) -> Option<Vec<Location>> {
        let (tokens, errors) = tokenize_with_errors(content);

        if !errors.is_empty() {
            return None;
        }

        let line = position.line as usize + 1;
        let col = position.character as usize + 1;

        let target_name = self.find_identifier_at(&tokens, line, col)?;

        if self.is_builtin(&target_name) {
            return None;
        }

        let mut locations = Vec::new();

        for token in &tokens {
            if let TokenKind::Identifier(name) = &token.kind
                && name == &target_name
            {
                locations.push(Location {
                    uri: Uri::from_str(uri).unwrap(),
                    range: Range {
                        start: Position {
                            line: (token.span.line - 1) as u32,
                            character: (token.span.column - 1) as u32,
                        },
                        end: Position {
                            line: (token.span.line - 1) as u32,
                            character: (token.span.column + name.len() - 1) as u32,
                        },
                    },
                });
            }
        }

        if locations.is_empty() {
            None
        } else {
            Some(locations)
        }
    }

    fn find_identifier_at(
        &self,
        tokens: &[crate::lexer::Token],
        line: usize,
        col: usize,
    ) -> Option<String> {
        for token in tokens {
            if token.span.line == line
                && token.span.column <= col
                && let TokenKind::Identifier(name) = &token.kind
            {
                let end_col = token.span.column + name.len();
                if col <= end_col {
                    return Some(name.clone());
                }
            }
        }
        None
    }

    fn build_symbol_table(
        &self,
        tokens: &[crate::lexer::Token],
        _content: &str,
    ) -> std::collections::HashMap<String, Vec<SymbolInfo>> {
        use std::collections::HashMap;
        let mut symbols: HashMap<String, Vec<SymbolInfo>> = HashMap::new();

        let ast = parse(tokens.to_vec());
        for stmt in ast {
            self.extract_symbols_from_stmt(&stmt, tokens, &mut symbols);
        }

        symbols
    }

    fn extract_symbols_from_stmt(
        &self,
        stmt: &Stmt,
        tokens: &[crate::lexer::Token],
        symbols: &mut std::collections::HashMap<String, Vec<SymbolInfo>>,
    ) {
        match &stmt.kind {
            StmtKind::Function {
                name,
                params: _,
                body,
                ..
            } => {
                symbols.entry(name.clone()).or_default().push(SymbolInfo {
                    _name: name.clone(),
                    line: stmt.span.line,
                    column: stmt.span.column,
                    _kind: SymbolKind::FUNCTION,
                });
                self.extract_symbols_from_expr(body, tokens, symbols);
            }
            StmtKind::Let {
                name, name_span, ..
            } => {
                symbols.entry(name.clone()).or_default().push(SymbolInfo {
                    _name: name.clone(),
                    line: name_span.line,
                    column: name_span.column,
                    _kind: SymbolKind::VARIABLE,
                });
            }
            StmtKind::Expr(expr) | StmtKind::Result(expr) => {
                self.extract_symbols_from_expr(expr, tokens, symbols);
            }
            StmtKind::Return(Some(expr)) => {
                self.extract_symbols_from_expr(expr, tokens, symbols);
            }
            StmtKind::Break
            | StmtKind::Continue
            | StmtKind::Return(None)
            | StmtKind::Struct { .. } => {}
        }
    }

    fn extract_symbols_from_expr(
        &self,
        expr: &crate::parser::Expr,
        tokens: &[crate::lexer::Token],
        symbols: &mut std::collections::HashMap<String, Vec<SymbolInfo>>,
    ) {
        use crate::parser::ExprKind::*;

        match &expr.kind {
            If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.extract_symbols_from_expr(condition, tokens, symbols);
                self.extract_symbols_from_expr(then_branch, tokens, symbols);
                if let Some(else_b) = else_branch {
                    self.extract_symbols_from_expr(else_b, tokens, symbols);
                }
            }
            While { condition, body } => {
                self.extract_symbols_from_expr(condition, tokens, symbols);
                self.extract_symbols_from_expr(body, tokens, symbols);
            }
            Block(stmts) => {
                for stmt in stmts {
                    let dummy_span = expr.span;
                    let stmt = crate::parser::Stmt {
                        kind: stmt.clone(),
                        span: dummy_span,
                    };
                    self.extract_symbols_from_stmt(&stmt, tokens, symbols);
                }
            }
            BinaryOp { left, right, .. } => {
                self.extract_symbols_from_expr(left, tokens, symbols);
                self.extract_symbols_from_expr(right, tokens, symbols);
            }
            UnaryOp { expr, .. } => {
                self.extract_symbols_from_expr(expr, tokens, symbols);
            }
            FunctionCall { args, .. } => {
                for arg in args {
                    self.extract_symbols_from_expr(arg, tokens, symbols);
                }
            }
            Array(items) => {
                for item in items {
                    self.extract_symbols_from_expr(item, tokens, symbols);
                }
            }
            Object(fields) => {
                for (_, value) in fields {
                    self.extract_symbols_from_expr(value, tokens, symbols);
                }
            }
            Index { object, index } => {
                self.extract_symbols_from_expr(object, tokens, symbols);
                self.extract_symbols_from_expr(index, tokens, symbols);
            }
            GetProperty { object, .. } => {
                self.extract_symbols_from_expr(object, tokens, symbols);
            }
            Parenthesized(e) => {
                self.extract_symbols_from_expr(e, tokens, symbols);
            }
            Number(_) | Boolean(_) | String(_) | Variable(_) | StructInstance { .. } => {}
        }
    }

    fn find_token_by_name(
        &self,
        name: &str,
        tokens: &[crate::lexer::Token],
    ) -> Option<crate::lexer::Token> {
        for token in tokens {
            if let TokenKind::Identifier(id) = &token.kind
                && id == name
            {
                return Some(token.clone());
            }
        }
        None
    }

    pub fn is_builtin(&self, name: &str) -> bool {
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
}

#[derive(Debug, Clone)]
struct SymbolInfo {
    _name: String,
    line: usize,
    column: usize,
    _kind: SymbolKind,
}
