use mp_lang::lsp::MpLanguageServer;
use tokio::io::{stdin, stdout};
use tower_lsp_server::{LspService, Server};

#[tokio::main]
async fn main() {
    let stdin = stdin();
    let stdout = stdout();

    let (service, socket) = LspService::build(MpLanguageServer::new).finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
