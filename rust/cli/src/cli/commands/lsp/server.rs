#[derive(Debug)]
enum FileType {
    Server,
    Client,
}

fn get_file_type(
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

fn format_server_file(
    formatting_shared_config: &crate::cli::config::GraphqlFormattingSharedConfig,
    formatting_server_config: &crate::cli::config::GraphqlFormattingServerConfig,
    local_path: &std::path::PathBuf,
) -> Result<Vec<lsp_types::TextEdit>, String> {
    let buffer = std::fs::read_to_string(local_path).unwrap();
    let ast_nodes =
        match crate::cli::shared::buffer_to_server_ast(local_path, &buffer) {
            Ok(nodes) => nodes,
            Err(error) => {
                return Err(error);
            }
        };
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
    let libgql_formatting_config = libgql::formatter::server::config::Config {
        shared: &shared_formatter_config,
    };
    let hir_nodes = libgql::formatter::server::nodes::format_nodes(
        &libgql_formatting_config,
        &ast_nodes,
    )
    .to_vec();
    let mut hir_to_lir_state = codeform::hir_to_lir::state::State::default();
    let lir_nodes = codeform::hir_to_lir::mappers::nodes::lower(
        &hir_to_lir_config,
        &mut hir_to_lir_state,
        hir_nodes,
    );
    let mut writer = std::io::BufWriter::new(Vec::<u8>::new());
    let mut printer_state = codeform::lir_printer::State::default();
    codeform::lir_printer::print_nodes(
        &mut writer,
        &lir_printer_config,
        &mut printer_state,
        &lir_nodes,
    )
    .unwrap();
    let formatted_string =
        String::from_utf8(writer.into_inner().unwrap()).unwrap();
    if formatted_string == buffer {
        Ok(Vec::new())
    } else {
        Ok(vec![lsp_types::TextEdit {
            range: lsp_types::Range {
                start: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
                end: lsp_types::Position {
                    line: u32::MAX,
                    character: u32::MAX,
                },
            },
            new_text: formatted_string,
        }])
    }
}

fn format_client_file(
    formatting_shared_config: &crate::cli::config::GraphqlFormattingSharedConfig,
    formatting_client_config: &crate::cli::config::GraphqlFormattingClientConfig,
    local_path: &std::path::PathBuf,
) -> Result<Vec<lsp_types::TextEdit>, String> {
    let buffer = std::fs::read_to_string(local_path).unwrap();
    let ast_nodes =
        match crate::cli::shared::buffer_to_client_ast(local_path, &buffer) {
            Ok(nodes) => nodes,
            Err(error) => {
                return Err(error);
            }
        };
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
    let libgql_formatting_config = libgql::formatter::client::config::Config {
        shared: &shared_formatter_config,
    };
    let hir_nodes = libgql::formatter::client::nodes::format_nodes(
        &libgql_formatting_config,
        &ast_nodes,
    )
    .to_vec();
    let mut hir_to_lir_state = codeform::hir_to_lir::state::State::default();
    let lir_nodes = codeform::hir_to_lir::mappers::nodes::lower(
        &hir_to_lir_config,
        &mut hir_to_lir_state,
        hir_nodes,
    );
    let mut writer = std::io::BufWriter::new(Vec::<u8>::new());
    let mut printer_state = codeform::lir_printer::State::default();
    codeform::lir_printer::print_nodes(
        &mut writer,
        &lir_printer_config,
        &mut printer_state,
        &lir_nodes,
    )
    .unwrap();
    let formatted_string =
        String::from_utf8(writer.into_inner().unwrap()).unwrap();
    if formatted_string == buffer {
        Ok(Vec::new())
    } else {
        Ok(vec![lsp_types::TextEdit {
            range: lsp_types::Range {
                start: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
                end: lsp_types::Position {
                    line: u32::MAX,
                    character: u32::MAX,
                },
            },
            new_text: formatted_string,
        }])
    }
}

fn format_file_with_type(
    formatting_config: &crate::cli::config::GraphqlFormattingConfig,
    local_path: &std::path::PathBuf,
    file_type: FileType,
) -> Result<Vec<lsp_types::TextEdit>, String> {
    match file_type {
        FileType::Server => formatting_config
            .server
            .as_ref()
            .map(|formatting_server_config| {
                format_server_file(
                    &formatting_config.shared,
                    formatting_server_config,
                    local_path,
                )
            })
            .unwrap_or(Ok(Vec::new())),
        FileType::Client => formatting_config
            .client
            .as_ref()
            .map(|formatting_client_config| {
                format_client_file(
                    &formatting_config.shared,
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
    return get_file_type(&config, &local_path)
        .and_then(|file_type| {
            config.formatting.as_ref().map(|formatting_config| {
                format_file_with_type(formatting_config, &local_path, file_type)
            })
        })
        .unwrap_or(Ok(Vec::new()));
}

pub struct ServerContext {
    pub config_directory_path: std::path::PathBuf,
    pub config: crate::cli::config::Config,
}

#[derive(Clone)]
pub struct ServerMetadata(pub std::sync::Arc<ServerContext>);

impl jsonrpc_core::Metadata for ServerMetadata {}

pub fn build_jsonrpc_server() -> jsonrpc_core::MetaIoHandler<ServerMetadata> {
    let mut io = jsonrpc_core::MetaIoHandler::<ServerMetadata>::default();
    io.add_method("initialize", |params: jsonrpc_core::Params| async {
        let _init_params: lsp_types::InitializeParams = params.parse()?;
        let result = lsp_types::InitializeResult {
            capabilities: lsp_types::ServerCapabilities {
                document_formatting_provider: Some(lsp_types::OneOf::Left(
                    true,
                )),
                ..Default::default()
            },
            server_info: None,
        };
        serde_json::to_value(result)
            .map_err(|_| jsonrpc_core::Error::internal_error())
    });
    io.add_method("shutdown", |_: jsonrpc_core::Params| async move {
        serde_json::to_value(None::<Option<()>>)
            .map_err(|_| jsonrpc_core::Error::internal_error())
    });
    io.add_method_with_meta(
        "textDocument/formatting",
        |params: jsonrpc_core::Params, meta: ServerMetadata| async move {
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
        },
    );
    io
}
