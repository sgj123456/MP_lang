use crate::lexer::{Span, tokenize};
use crate::parser::parse;
use tower_lsp::{Client, lsp_types::*};

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
        let uri = url::Url::parse(uri).unwrap();
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

        if let Err(e) = parse(tokens.clone()) {
            diagnostics.push(Diagnostic {
                range: self.span_to_range(&e.span()),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("MP002".to_string())),
                source: Some("mp-lang".to_string()),
                message: format!("Parser error: {}", e),
                ..Default::default()
            });
        }

        diagnostics
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
