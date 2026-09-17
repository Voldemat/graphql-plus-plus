use crate::cli::{
    shared::OpenBuffers,
    utils::{
        SchemaASTWrapper, SchemaErrorSerializer,
        load_client_schema_from_inputs, load_server_schema_from_inputs,
    },
};

use super::{
    codec::LspCodec, context::ServerContext, file_type::get_file_type,
    location::token_location_to_range, server::send_notification,
};

pub async fn publish_workspace_diagnostics(
    context: &ServerContext,
    writer: std::sync::Arc<
        tokio::sync::Mutex<
            tokio_util::codec::FramedWrite<tokio::io::Stdout, LspCodec>,
        >,
    >,
    local_path: &std::path::PathBuf,
    open_buffers: &impl std::ops::Deref<Target = OpenBuffers>,
) -> Result<(), String> {
    let Some(_) = get_file_type(&context.config, &local_path) else {
        return Ok(());
    };
    let publish_diagnostic_params = get_workspace_diagnostics(
        &context.config_directory_path,
        &context.config,
        open_buffers,
    )?;
    for params in publish_diagnostic_params {
        send_notification(&writer, "textDocument/publishDiagnostics", params)
            .await?;
    }
    Ok(())
}

pub struct LspDiagnosticErrorSerializer {}

impl<TASTWrapper: SchemaASTWrapper> SchemaErrorSerializer<TASTWrapper>
    for LspDiagnosticErrorSerializer
{
    type Error = lsp_types::Diagnostic;

    fn lexer_error(
        source_file: &libgql::parsers::file::shared::ast::SourceFile,
        error: libgql::lexer::types::Error,
    ) -> Self::Error {
        lsp_types::Diagnostic {
            range: token_location_to_range(
                &source_file.new_line_positions,
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

    fn file_parser_error(
        source_file: &libgql::parsers::file::shared::ast::SourceFile,
        error: TASTWrapper::FileParserError<'_>,
    ) -> Self::Error {
        let location =
            libgql::parsers::file::shared::error::Error::get_location(&error);
        lsp_types::Diagnostic {
            range: token_location_to_range(
                &source_file.new_line_positions,
                location,
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

    fn schema_parser_error(
        error: TASTWrapper::SchemaParserError<'_>,
    ) -> Self::Error {
        let location =
            libgql::parsers::schema::shared::error::Error::get_location(&error);
        lsp_types::Diagnostic {
            range: token_location_to_range(
                &location.source.new_line_positions,
                &location.location,
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
}

pub async fn get_buffer<'buffer>(
    buffers: &'buffer tokio::sync::RwLock<OpenBuffers>,
    local_path: &std::path::PathBuf,
) -> Result<tokio::sync::RwLockReadGuard<'buffer, str>, String> {
    let read_buffers = buffers.read().await;
    tokio::sync::RwLockReadGuard::try_map(read_buffers, |map| {
        map.get(local_path).map(|s| s.content.as_str())
    })
    .map_err(|_| "Buffer for path is not found".to_string())
}

pub fn get_workspace_diagnostics(
    config_directory_path: &std::path::Path,
    config: &crate::cli::config::Config,
    open_buffers: &impl std::ops::Deref<Target = OpenBuffers>,
) -> Result<Vec<lsp_types::PublishDiagnosticsParams>, String> {
    let mut server_registry =
        libgql::parsers::schema::server::type_registry::HashMapTypeRegistry::new();
    let Some(config_server) = config.server.as_ref() else {
        return Err("config.server is not defined".to_string());
    };
    let mut diagnostics_map = std::collections::HashMap::<
        std::path::PathBuf,
        Vec<lsp_types::Diagnostic>,
    >::new();
    for (filepath, diagnostics) in
        load_server_schema_from_inputs::<LspDiagnosticErrorSerializer>(
            &mut server_registry,
            config_directory_path,
            &config_server.inputs,
            open_buffers,
        )
    {
        if !diagnostics_map.contains_key(&filepath) {
            diagnostics_map.insert(filepath.clone(), Vec::new());
        }
        diagnostics_map
            .get_mut(&filepath)
            .unwrap()
            .extend(diagnostics);
    }
    if let Some(client_config) = config.client.as_ref() {
        let mut c_registry =
            libgql::parsers::schema::client::type_registry::TypeRegistry::new();
        for (filepath, diagnostics) in
            load_client_schema_from_inputs::<LspDiagnosticErrorSerializer>(
                &server_registry,
                &mut c_registry,
                config_directory_path,
                &client_config.inputs,
                open_buffers,
            )
        {
            if !diagnostics_map.contains_key(&filepath) {
                diagnostics_map.insert(filepath.clone(), Vec::new());
            }
            diagnostics_map
                .get_mut(&filepath)
                .unwrap()
                .extend(diagnostics);
        }
    };
    for filepath in open_buffers.deref().keys() {
        if !diagnostics_map.contains_key(filepath) {
            diagnostics_map.insert(filepath.clone(), Vec::new());
        }
    }
    Ok(diagnostics_map
        .into_iter()
        .map(|(filepath, diagnostics)| {
            let (uri, version) = open_buffers
                .get(&filepath)
                .map(|open_buffer| {
                    (open_buffer.uri.clone(), Some(open_buffer.version))
                })
                .unwrap_or_else(|| {
                    (
                        <lsp_types::Uri as std::str::FromStr>::from_str(
                            &format!(
                                "file://{}",
                                config_directory_path
                                    .join(filepath)
                                    .to_string_lossy()
                            ),
                        )
                        .unwrap(),
                        None,
                    )
                });
            lsp_types::PublishDiagnosticsParams {
                uri,
                diagnostics,
                version,
            }
        })
        .collect::<Vec<_>>())
}
