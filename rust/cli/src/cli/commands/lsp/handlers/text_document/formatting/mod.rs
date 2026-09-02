mod file_type;
mod lsp_edits;

use crate::cli::commands::lsp::meta::ServerMetadata;

fn format_file<
    TASTNodeWrapper: crate::cli::commands::format::shared::ASTNodeWrapper,
    TBufferToASTNodes: for<'buffer> Fn(
        &std::path::PathBuf,
        &'buffer str,
    )
        -> Result<Vec<TASTNodeWrapper::ASTNode<'buffer>>, String>,
    TASTNodesToHIRNodes: for<'buffer> Fn(
        &'buffer str,
        Vec<TASTNodeWrapper::ASTNode<'buffer>>,
    ) -> Vec<codeform::ir::hir::node::Node<'buffer>>,
>(
    shared_configs: &SharedConfigs,
    buffer_to_ast_nodes: TBufferToASTNodes,
    ast_nodes_to_hir_nodes: TASTNodesToHIRNodes,
    local_path: &std::path::PathBuf,
) -> Result<Vec<lsp_types::TextEdit>, String> {
    let buffer = std::fs::read_to_string(local_path).unwrap();
    let mut writer = std::io::BufWriter::new(Vec::<u8>::new());
    crate::cli::commands::format::shared::format_buffer::<
        TASTNodeWrapper,
        TBufferToASTNodes,
        TASTNodesToHIRNodes,
        _,
    >(
        local_path,
        &buffer,
        buffer_to_ast_nodes,
        ast_nodes_to_hir_nodes,
        &shared_configs.hir_to_lir_config,
        &shared_configs.lir_printer_config,
        &mut writer,
    )?;
    let formatted_string =
        String::from_utf8(writer.into_inner().unwrap()).unwrap();
    Ok(lsp_edits::generate(&buffer, &formatted_string))
}

fn format_server_file(
    shared_configs: SharedConfigs,
    formatting_server_config: &crate::cli::config::GraphqlFormattingServerConfig,
    local_path: &std::path::PathBuf,
) -> Result<Vec<lsp_types::TextEdit>, String> {
    let libgql_formatting_config = libgql::formatter::server::config::Config {
        shared: &shared_configs.shared_formatter_config,
    };
    format_file::<
        crate::cli::commands::format::shared::ServerASTNodeWrapper,
        _,
        _,
    >(
        &shared_configs,
        crate::cli::shared::buffer_to_server_ast,
        |_, nodes| {
            libgql::formatter::server::nodes::format_nodes(
                &libgql_formatting_config,
                &nodes,
            )
            .to_vec()
        },
        local_path,
    )
}

fn format_client_file(
    shared_configs: SharedConfigs,
    formatting_client_config: &crate::cli::config::GraphqlFormattingClientConfig,
    local_path: &std::path::PathBuf,
) -> Result<Vec<lsp_types::TextEdit>, String> {
    let libgql_formatting_config = libgql::formatter::client::config::Config {
        shared: &shared_configs.shared_formatter_config,
    };
    format_file::<
        crate::cli::commands::format::shared::ClientASTNodeWrapper,
        _,
        _,
    >(
        &shared_configs,
        crate::cli::shared::buffer_to_client_ast,
        |_, nodes| {
            libgql::formatter::client::nodes::format_nodes(
                &libgql_formatting_config,
                &nodes,
            )
            .to_vec()
        },
        local_path,
    )
}

pub struct SharedConfigs {
    shared_formatter_config: libgql::formatter::shared::config::Config,
    hir_to_lir_config: codeform::hir_to_lir::config::Config,
    lir_printer_config: codeform::lir_printer::Config,
}

fn get_shared_configs(
    formatting_shared_config: &crate::cli::config::GraphqlFormattingSharedConfig,
) -> SharedConfigs {
    let shared_formatter_config = libgql::formatter::shared::config::Config {
        indent_width: codeform::ir::shared::IndentWidth::from_u8(
            formatting_shared_config.indent_width.into(),
        )
        .unwrap(),
    };
    let hir_to_lir_config = codeform::hir_to_lir::config::Config {
        indent_width: shared_formatter_config.indent_width,
        max_width: formatting_shared_config.max_line_width,
    };
    let lir_printer_config = codeform::lir_printer::Config {
        indent_width: shared_formatter_config.indent_width,
        new_line_control_sequence: b"\n",
    };
    SharedConfigs {
        shared_formatter_config,
        hir_to_lir_config,
        lir_printer_config,
    }
}

fn format_file_with_type(
    formatting_config: &crate::cli::config::GraphqlFormattingConfig,
    local_path: &std::path::PathBuf,
    file_type: file_type::FileType,
) -> Result<Vec<lsp_types::TextEdit>, String> {
    match file_type {
        file_type::FileType::Server => formatting_config
            .server
            .as_ref()
            .map(|formatting_server_config| {
                format_server_file(
                    get_shared_configs(&formatting_config.shared),
                    formatting_server_config,
                    local_path,
                )
            })
            .unwrap_or(Ok(Vec::new())),
        file_type::FileType::Client => formatting_config
            .client
            .as_ref()
            .map(|formatting_client_config| {
                format_client_file(
                    get_shared_configs(&formatting_config.shared),
                    formatting_client_config,
                    local_path,
                )
            })
            .unwrap_or(Ok(Vec::new())),
    }
}

fn format(
    config_directory_path: &std::path::Path,
    config: &crate::cli::config::Config,
    params: lsp_types::DocumentFormattingParams,
) -> Result<Vec<lsp_types::TextEdit>, String> {
    let uri = params.text_document.uri;
    if uri
        .scheme()
        .map(|scheme| scheme.as_str() != "file")
        .unwrap_or(true)
    {
        return Ok(Vec::new());
    }
    let local_path = std::path::Path::new("./").join(
        <std::path::PathBuf as std::str::FromStr>::from_str(
            uri.path().as_str(),
        )
        .unwrap()
        .strip_prefix(config_directory_path)
        .unwrap(),
    );
    return file_type::get_file_type(&config, &local_path)
        .and_then(|file_type| {
            config.formatting.as_ref().map(|formatting_config| {
                format_file_with_type(formatting_config, &local_path, file_type)
            })
        })
        .unwrap_or(Ok(Vec::new()));
}

pub async fn handler(
    params: jsonrpc_core::Params,
    meta: ServerMetadata,
) -> jsonrpc_core::Result<serde_json::Value> {
    let format_params: lsp_types::DocumentFormattingParams =
        params.parse::<serde_json::value::Value>().and_then(|v| {
            serde_json::from_value(v).map_err(|error| {
                jsonrpc_core::Error::invalid_params(error.to_string())
            })
        })?;
    let context = meta.0.as_ref();

    format(
        &context.config_directory_path,
        &context.config,
        format_params,
    )
    .map_err(|_| jsonrpc_core::Error::internal_error())
    .and_then(|result| {
        serde_json::to_value(result)
            .map_err(|_| jsonrpc_core::Error::internal_error())
    })
}
