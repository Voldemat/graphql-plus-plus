use crate::cli::shared::OpenBuffers;

pub struct ServerContext {
    pub config_directory_path: std::path::PathBuf,
    pub config: crate::cli::config::Config,
    pub open_buffers: tokio::sync::RwLock<OpenBuffers>,
}
