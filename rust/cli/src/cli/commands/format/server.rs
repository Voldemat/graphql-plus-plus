pub fn format_config(
    config_directory_path: &std::path::Path,
    is_check: bool,
    config: &crate::cli::config::ServerConfig,
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
    server_formatting_config: &crate::cli::config::GraphqlFormattingServerConfig,
) -> Vec<String> {
    super::shared::format_config::<super::shared::ServerASTNodeWrapper, _, _>(
        &crate::cli::utils::resolve_paths(
            config_directory_path,
            &config.inputs.graphql,
        ),
        crate::cli::shared::buffer_to_server_ast,
        |_, nodes| {
            libgql::formatter::server::nodes::format_nodes(
                shared_formatting_config,
                server_formatting_config,
                &nodes,
            )
            .to_vec()
        },
        shared_formatting_config,
        is_check,
    )
}
