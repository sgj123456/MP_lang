use crate::lexer::TokenKind;
use crate::lexer::tokenize;
use crate::parser::{Expr, ExprKind, Stmt, StmtKind, parse};
use tower_lsp::lsp_types::*;

#[derive(Debug)]
pub struct MpSymbols;

impl Default for MpSymbols {
    fn default() -> Self {
        Self::new()
    }
}

impl MpSymbols {
    pub fn new() -> Self {
        Self
    }

    pub fn symbols(&self, content: &str) -> Vec<DocumentSymbol> {
        let mut symbols = Vec::new();

        if let Ok(tokens) = tokenize(content) {
            let ast = parse(tokens.clone());
            for stmt in ast {
                self.extract_symbol_from_stmt(&stmt, &tokens, &mut symbols);
            }
        }

        symbols
    }

    fn extract_symbol_from_stmt(
        &self,
        stmt: &Stmt,
        tokens: &[crate::lexer::Token],
        symbols: &mut Vec<DocumentSymbol>,
    ) {
        match &stmt.kind {
            StmtKind::Function { name, params, .. } => {
                let range = self.find_token_range(name, tokens);
                #[allow(deprecated)]
                let symbol = DocumentSymbol {
                    name: name.clone(),
                    detail: Some(format!("fn({})", params.join(", "))),
                    kind: SymbolKind::FUNCTION,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                };
                symbols.push(symbol);
            }
            StmtKind::Let { name, value } => {
                let range = self.find_token_range(name, tokens);
                let kind = self.infer_variable_kind(value);
                #[allow(deprecated)]
                let symbol = DocumentSymbol {
                    name: name.clone(),
                    detail: Some("let".to_string()),
                    kind,
                    tags: None,
                    deprecated: None,
                    range,
                    selection_range: range,
                    children: None,
                };
                symbols.push(symbol);
            }
            _ => {}
        }
    }

    fn find_token_range(&self, name: &str, tokens: &[crate::lexer::Token]) -> Range {
        for token in tokens.iter() {
            if let TokenKind::Identifier(id) = &token.kind
                && id == name
            {
                let end_col = token.span.column + name.len();
                return Range {
                    start: Position {
                        line: (token.span.line - 1) as u32,
                        character: (token.span.column - 1) as u32,
                    },
                    end: Position {
                        line: (token.span.line - 1) as u32,
                        character: end_col as u32,
                    },
                };
            }
        }
        Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: Position {
                line: 0,
                character: 0,
            },
        }
    }

    fn infer_variable_kind(&self, expr: &Expr) -> SymbolKind {
        match &expr.kind {
            ExprKind::FunctionCall { .. } => SymbolKind::FUNCTION,
            ExprKind::Array(_) => SymbolKind::ARRAY,
            ExprKind::Object(_) => SymbolKind::OBJECT,
            ExprKind::Boolean(_) => SymbolKind::BOOLEAN,
            ExprKind::String(_) => SymbolKind::STRING,
            ExprKind::Number(_) => SymbolKind::NUMBER,
            _ => SymbolKind::VARIABLE,
        }
    }
}
