pub struct ServerContext {
    pub config_directory_path: std::path::PathBuf,
    pub config: crate::cli::config::Config,
}

#[derive(Clone)]
pub struct ServerMetadata(pub std::sync::Arc<ServerContext>);

impl jsonrpc_core::Metadata for ServerMetadata {}
