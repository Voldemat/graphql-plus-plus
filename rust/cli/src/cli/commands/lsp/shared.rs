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
    match file_type {
        FileType::Server => {
            let source_file = std::sync::Arc::new(
                libgql::parsers::file::shared::ast::SourceFile {
                    filepath: local_path.clone(),
                    buffer: &buffer,
                },
            );
            let result = crate::cli::shared::buffer_to_server_ast(&source_file);
            let mut diagnostics = Vec::new();
            for lexer_error in &result.lexer_errors {
                let location = lexer_error.get_location();
                diagnostics.push(lsp_types::Diagnostic {
                    range: token_location_to_range(
                        &result.new_line_positions,
                        location,
                    ),
                    code: None,
                    code_description: None,
                    message: lexer_error.to_string(),
                    data: None,
                    related_information: None,
                    severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                    source: None,
                    tags: None,
                })
            }
            for parser_error in &result.parser_errors {
                let location =
                    libgql::parsers::file::shared::error::Error::get_location(
                        parser_error,
                    );
                diagnostics.push(lsp_types::Diagnostic {
                    range: token_location_to_range(
                        &result.new_line_positions,
                        location,
                    ),
                    code: None,
                    code_description: None,
                    message: parser_error.to_string(),
                    data: None,
                    related_information: None,
                    severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                    source: None,
                    tags: None,
                })
            }
            Ok(diagnostics)
        }
        FileType::Client => Ok(Vec::new()),
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
