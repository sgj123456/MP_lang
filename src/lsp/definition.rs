use crate::lexer::{TokenKind, tokenize};
use crate::parser::{Stmt, StmtKind, parse};
use tower_lsp::lsp_types::*;

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
        let tokens = tokenize(content).ok()?;

        let line = position.line as usize + 1;
        let col = position.character as usize + 1;

        let target_name = self.find_identifier_at(&tokens, line, col)?;

        if self.is_builtin(&target_name) {
            return None;
        }

        let symbols = self.build_symbol_table(&tokens, content);

        if let Some(symbol_infos) = symbols.get(&target_name)
            && let Some(first) = symbol_infos.first()
        {
            let location = Location {
                uri: url::Url::parse(uri).unwrap(),
                range: Range {
                    start: Position {
                        line: (first.line - 1) as u32,
                        character: (first.column - 1) as u32,
                    },
                    end: Position {
                        line: (first.line - 1) as u32,
                        character: (first.column + target_name.len() - 1) as u32,
                    },
                },
            };
            return Some(GotoDefinitionResponse::Scalar(location));
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
                    && let Some(token) = self.find_token_by_name(&name, tokens) {
                        let location = Location {
                            uri: url::Url::parse(uri).unwrap(),
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
        let tokens = tokenize(content).ok()?;

        let line = position.line as usize + 1;
        let col = position.character as usize + 1;

        let target_name = self.find_identifier_at(&tokens, line, col)?;

        if self.is_builtin(&target_name) {
            return None;
        }

        let mut locations = Vec::new();
        let uri = url::Url::parse(uri).ok()?;

        for token in &tokens {
            if let TokenKind::Identifier(name) = &token.kind
                && name == &target_name
            {
                locations.push(Location {
                    uri: uri.clone(),
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
            StmtKind::Function { name, .. } => {
                if let Some(token) = self.find_token_by_name(name, tokens) {
                    symbols.entry(name.clone()).or_default().push(SymbolInfo {
                        _name: name.clone(),
                        line: token.span.line,
                        column: token.span.column,
                        _kind: SymbolKind::FUNCTION,
                    });
                }
            }
            StmtKind::Let { name, .. } => {
                if let Some(token) = self.find_token_by_name(name, tokens) {
                    symbols.entry(name.clone()).or_default().push(SymbolInfo {
                        _name: name.clone(),
                        line: token.span.line,
                        column: token.span.column,
                        _kind: SymbolKind::VARIABLE,
                    });
                }
            }
            _ => {}
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
