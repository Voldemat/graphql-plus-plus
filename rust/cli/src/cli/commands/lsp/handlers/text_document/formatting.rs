use crate::cli::commands::lsp::meta::ServerMetadata;

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

pub fn generate_lsp_edits(
    old_text: &str,
    new_text: &str,
) -> Vec<lsp_types::TextEdit> {
    let diff = similar::TextDiff::from_lines(old_text, new_text);
    let mut edits = Vec::new();

    // Grouping operations isolates clusters of changes for efficient edits
    for hunk in diff.grouped_ops(3) {
        for op in hunk {
            match op {
                similar::DiffOp::Equal { .. } => {}
                similar::DiffOp::Delete {
                    old_index, old_len, ..
                } => {
                    edits.push(lsp_types::TextEdit {
                        range: lsp_types::Range::new(
                            lsp_types::Position::new(old_index as u32, 0),
                            lsp_types::Position::new(
                                (old_index + old_len) as u32,
                                0,
                            ),
                        ),
                        new_text: String::new(),
                    });
                }
                similar::DiffOp::Insert {
                    old_index,
                    new_index,
                    new_len,
                } => {
                    let text: String = new_text
                        .lines()
                        .skip(new_index)
                        .take(new_len)
                        .map(|l| format!("{l}\n"))
                        .collect(); // careful with final newline / CRLF
                    edits.push(lsp_types::TextEdit {
                        range: lsp_types::Range::new(
                            lsp_types::Position::new(old_index as u32, 0),
                            lsp_types::Position::new(old_index as u32, 0),
                        ),
                        new_text: text,
                    });
                }
                similar::DiffOp::Replace {
                    old_index,
                    old_len,
                    new_index,
                    new_len,
                } => {
                    let text: String = new_text
                        .lines()
                        .skip(new_index)
                        .take(new_len)
                        .map(|l| format!("{l}\n"))
                        .collect();
                    edits.push(lsp_types::TextEdit {
                        range: lsp_types::Range::new(
                            lsp_types::Position::new(old_index as u32, 0),
                            lsp_types::Position::new(
                                (old_index + old_len) as u32,
                                0,
                            ),
                        ),
                        new_text: text,
                    });
                }
            }
        }
    }

    edits
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
    return Ok(generate_lsp_edits(&buffer, &formatted_string));
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
    return Ok(generate_lsp_edits(&buffer, &formatted_string));
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
