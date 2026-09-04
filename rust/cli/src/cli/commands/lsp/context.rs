pub struct ServerContext {
    pub config_directory_path: std::path::PathBuf,
    pub config: crate::cli::config::Config,
    pub buffers: tokio::sync::RwLock<
        std::collections::HashMap<std::path::PathBuf, String>,
    >,
}
