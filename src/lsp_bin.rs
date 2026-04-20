use tokio::io::{stdin, stdout};
use tower_lsp::{LspService, Server};
use mp_lang::lsp::MpLanguageServer;

#[tokio::main]
async fn main() {
    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::build(|client| MpLanguageServer::new(client))
        .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
