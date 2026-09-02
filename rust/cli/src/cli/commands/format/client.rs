pub fn format_config(
    config_directory_path: &std::path::Path,
    is_check: bool,
    config: &crate::cli::config::ClientConfig,
    shared_formatter_config: &crate::cli::config::GraphqlFormattingSharedConfig,
    client_formatter_config: &crate::cli::config::GraphqlFormattingClientConfig,
) -> Vec<String> {
    super::shared::format_config::<super::shared::ClientASTNodeWrapper, _, _>(
        &crate::cli::utils::resolve_paths(
            config_directory_path,
            &config.inputs.graphql,
        ),
        crate::cli::shared::buffer_to_client_ast,
        |_, nodes| {
            libgql::formatter::client::nodes::format_nodes(
                shared_formatter_config,
                client_formatter_config,
                &nodes,
            )
            .to_vec()
        },
        shared_formatter_config,
        is_check,
    )
}
