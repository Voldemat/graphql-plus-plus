pub fn format_config(
    config_directory_path: &std::path::Path,
    is_check: bool,
    config: &crate::cli::config::ServerConfig,
    shared_formatter_config: &libgql::formatter::shared::config::Config,
    hir_to_lir_config: &codeform::hir_to_lir::config::Config,
    lir_printer_config: &codeform::lir_printer::Config,
    formatting_config: &crate::cli::config::GraphqlFormattingServerConfig,
) -> Vec<String> {
    let libgql_formatting_config = libgql::formatter::server::config::Config {
        shared: &shared_formatter_config,
    };
    super::shared::format_config::<super::shared::ServerASTNodeWrapper, _, _>(
        &crate::cli::utils::resolve_paths(
            config_directory_path,
            &config.inputs.graphql,
        ),
        crate::cli::shared::buffer_to_server_ast,
        |_, nodes| {
            libgql::formatter::server::nodes::format_nodes(
                &libgql_formatting_config,
                &nodes,
            )
            .to_vec()
        },
        hir_to_lir_config,
        lir_printer_config,
        is_check,
    )
}
