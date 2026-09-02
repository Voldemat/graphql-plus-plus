#[derive(Debug)]
pub enum FileType {
    Server,
    Client,
}

pub fn get_file_type(
    config: &crate::cli::config::Config,
    local_path: &std::path::Path,
) -> Option<FileType> {
    if let Some(config_server) = config.server.as_ref() {
        for server_graphql_pattern in &config_server.inputs.graphql {
            let pattern =
                glob::Pattern::new(&server_graphql_pattern.to_string_lossy())
                    .unwrap();
            if pattern.matches_path(local_path) {
                return Some(FileType::Server);
            }
        }
    }
    if let Some(config_client) = config.client.as_ref() {
        for client_graphql_pattern in &config_client.inputs.graphql {
            let pattern =
                glob::Pattern::new(&client_graphql_pattern.to_string_lossy())
                    .unwrap();
            if pattern.matches_path(local_path) {
                return Some(FileType::Client);
            }
        }
    }
    return None;
}
