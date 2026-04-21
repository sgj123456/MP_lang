use crate::lexer::tokenize;
use crate::parser::{Stmt, StmtKind, parse};
use tower_lsp::lsp_types::*;

#[derive(Debug)]
#[allow(dead_code)]
pub struct MpWorkspaceSymbols;

impl Default for MpWorkspaceSymbols {
    fn default() -> Self {
        Self::new()
    }
}

impl MpWorkspaceSymbols {
    pub fn new() -> Self {
        Self
    }

    pub fn workspace_symbols(&self, query: &str, uri: &str) -> Vec<SymbolInformation> {
        let mut symbols = Vec::new();
        let query_lower = query.to_lowercase();

        if let Ok(tokens) = tokenize(query) {
            let ast = parse(tokens);
            for stmt in ast {
                if let Some(info) = self.extract_symbol(&stmt, uri)
                    && (query.is_empty() || info.name.to_lowercase().contains(&query_lower)) {
                        symbols.push(info);
                    }
            }
        }

        symbols
    }

    #[allow(deprecated)]
    fn extract_symbol(&self, stmt: &Stmt, uri: &str) -> Option<SymbolInformation> {
        match &stmt.kind {
            StmtKind::Function {
                name, params: _, ..
            } => Some(SymbolInformation {
                name: name.clone(),
                kind: SymbolKind::FUNCTION,
                location: Location {
                    uri: url::Url::parse(uri).ok()?,
                    range: Range {
                        start: Position {
                            line: (stmt.span.line - 1) as u32,
                            character: 0,
                        },
                        end: Position {
                            line: (stmt.span.line - 1) as u32,
                            character: 10,
                        },
                    },
                },
                container_name: None,
                tags: None,
                deprecated: None,
            }),
            StmtKind::Let { name, .. } => Some(SymbolInformation {
                name: name.clone(),
                kind: SymbolKind::VARIABLE,
                location: Location {
                    uri: url::Url::parse(uri).ok()?,
                    range: Range {
                        start: Position {
                            line: (stmt.span.line - 1) as u32,
                            character: 0,
                        },
                        end: Position {
                            line: (stmt.span.line - 1) as u32,
                            character: 10,
                        },
                    },
                },
                container_name: None,
                tags: None,
                deprecated: None,
            }),
            _ => None,
        }
    }
}
