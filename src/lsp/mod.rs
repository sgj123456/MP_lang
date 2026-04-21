pub mod completion;
pub mod definition;
pub mod diagnostics;
pub mod hover;
pub mod inlay_hint;
mod server;
pub mod symbols;
pub mod type_analysis;
pub mod workspace_symbols;

pub use server::MpLanguageServer;
