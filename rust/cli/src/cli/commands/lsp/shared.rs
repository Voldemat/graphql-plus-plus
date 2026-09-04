use super::{
    codec::LspCodec,
    context::ServerContext,
    file_type::{FileType, get_file_type},
    location::token_location_to_range,
    server::send_notification,
};

fn get_file_diagnostics(
    local_path: &std::path::PathBuf,
    file_type: FileType,
    buffer: impl std::ops::Deref<Target = str>,
) -> Result<Vec<lsp_types::Diagnostic>, String> {
    let source_file =
        std::sync::Arc::new(libgql::parsers::file::shared::ast::SourceFile {
            filepath: local_path.clone(),
            buffer: &buffer,
        });
    match file_type {
        FileType::Server => {
            let result = crate::cli::shared::buffer_to_server_ast(&source_file);
            Ok(result
                .lexer_errors
                .into_iter()
                .map(|error| {
                    lexer_error_to_diagnostic(&result.new_line_positions, error)
                })
                .chain(result.parser_errors.into_iter().map(|error| {
                    file_parser_error_to_diagnostic(
                        &result.new_line_positions,
                        error,
                    )
                }))
                .collect::<Vec<_>>())
        }
        FileType::Client => {
            let result = crate::cli::shared::buffer_to_client_ast(&source_file);
            Ok(result
                .lexer_errors
                .into_iter()
                .map(|error| {
                    lexer_error_to_diagnostic(&result.new_line_positions, error)
                })
                .chain(result.parser_errors.into_iter().map(|error| {
                    file_parser_error_to_diagnostic(
                        &result.new_line_positions,
                        error,
                    )
                }))
                .collect::<Vec<_>>())
        }
    }
}

pub async fn get_buffer<'buffer>(
    buffers: &'buffer tokio::sync::RwLock<
        std::collections::HashMap<std::path::PathBuf, String>,
    >,
    local_path: &std::path::PathBuf,
) -> Result<tokio::sync::RwLockReadGuard<'buffer, str>, String> {
    let read_buffers = buffers.read().await;
    tokio::sync::RwLockReadGuard::try_map(read_buffers, |map| {
        map.get(local_path).map(|s| s.as_str())
    })
    .map_err(|_| "Buffer for path is not found".to_string())
}

pub async fn publish_file_diagnostics(
    context: &ServerContext,
    writer: std::sync::Arc<
        tokio::sync::Mutex<
            tokio_util::codec::FramedWrite<tokio::io::Stdout, LspCodec>,
        >,
    >,
    local_path: &std::path::PathBuf,
    buffer: impl std::ops::Deref<Target = str>,
    uri: lsp_types::Uri,
    version: i32,
) -> Result<(), String> {
    let Some(file_type) = get_file_type(&context.config, &local_path) else {
        return Ok(());
    };
    let diagnostics = get_file_diagnostics(&local_path, file_type, buffer)?;
    send_notification(
        &writer,
        "textDocument/publishDiagnostics",
        lsp_types::PublishDiagnosticsParams {
            uri: uri,
            diagnostics,
            version: Some(version),
        },
    )
    .await
}

pub fn lexer_error_to_diagnostic(
    new_line_positions: &[usize],
    error: libgql::lexer::types::Error,
) -> lsp_types::Diagnostic {
    lsp_types::Diagnostic {
        range: token_location_to_range(
            &new_line_positions,
            error.get_location(),
        ),
        code: None,
        code_description: None,
        message: error.to_string(),
        data: None,
        related_information: None,
        severity: Some(lsp_types::DiagnosticSeverity::ERROR),
        source: None,
        tags: None,
    }
}

pub fn file_parser_error_to_diagnostic<'buffer>(
    new_line_positions: &[usize],
    error: impl libgql::parsers::file::shared::error::Error,
) -> lsp_types::Diagnostic {
    let location =
        libgql::parsers::file::shared::error::Error::get_location(&error);
    lsp_types::Diagnostic {
        range: token_location_to_range(&new_line_positions, location),
        code: None,
        code_description: None,
        message: error.to_string(),
        data: None,
        related_information: None,
        severity: Some(lsp_types::DiagnosticSeverity::ERROR),
        source: None,
        tags: None,
    }
}
