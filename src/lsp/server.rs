use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use crate::lsp::completion::MpCompleter;
use crate::lsp::definition::MpDefinition;
use crate::lsp::diagnostics::MpDiagnostics;
use crate::lsp::hover::MpHover;
use crate::lsp::symbols::MpSymbols;

#[derive(Debug)]
pub struct MpLanguageServer {
    client: Client,
    documents: Arc<RwLock<HashMap<String, String>>>,
    completer: MpCompleter,
    diagnostics: MpDiagnostics,
    hover: MpHover,
    symbols: MpSymbols,
    definition: MpDefinition,
}

impl MpLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(HashMap::new())),
            completer: MpCompleter::new(),
            diagnostics: MpDiagnostics::new(),
            hover: MpHover::new(),
            symbols: MpSymbols::new(),
            definition: MpDefinition::new(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for MpLanguageServer {
    async fn initialize(&self, _params: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![
                        ".".to_string(),
                        ":".to_string(),
                        "(".to_string(),
                        ",".to_string(),
                    ]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                diagnostic_provider: Some(
                    DiagnosticServerCapabilities::Options(Default::default()),
                ),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _params: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "MP Language Server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let content = params.text_document.text;

        {
            let mut docs = self.documents.write().await;
            docs.insert(uri.clone(), content.clone());
        }

        self.diagnostics.publish(&self.client, &uri, &content).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let content = params.content_changes[0].text.clone();

        {
            let mut docs = self.documents.write().await;
            docs.insert(uri.clone(), content.clone());
        }

        self.diagnostics.publish(&self.client, &uri, &content).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let mut docs = self.documents.write().await;
        docs.remove(&uri);
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let position = params.text_document_position.position;

        let docs = self.documents.read().await;
        let content = docs.get(&uri).map(|s| s.as_str()).unwrap_or("");

        let items = self.completer.complete(content, position);
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let position = params.text_document_position_params.position;

        let docs = self.documents.read().await;
        let content = docs.get(&uri).map(|s| s.as_str()).unwrap_or("");

        Ok(self.hover.hover(content, position))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.to_string();

        let docs = self.documents.read().await;
        let content = docs.get(&uri).map(|s| s.as_str()).unwrap_or("");

        Ok(Some(DocumentSymbolResponse::Nested(
            self.symbols.symbols(content),
        )))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let position = params.text_document_position_params.position;

        let docs = self.documents.read().await;
        let content = docs.get(&uri).map(|s| s.as_str()).unwrap_or("");

        Ok(self.definition.goto_definition(content, position, &uri))
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri.to_string();
        let position = params.text_document_position.position;

        let docs = self.documents.read().await;
        let content = docs.get(&uri).map(|s| s.as_str()).unwrap_or("");

        Ok(self.definition.references(content, position, &uri))
    }
}
