use crate::cli::{
    commands::lsp::{
        codec::LspCodec,
        context::ServerContext,
        shared::{get_buffer, publish_workspace_diagnostics},
    },
    shared::OpenBuffer,
};

pub async fn handler(
    context: &ServerContext,
    writer: std::sync::Arc<
        tokio::sync::Mutex<
            tokio_util::codec::FramedWrite<tokio::io::Stdout, LspCodec>,
        >,
    >,
    params: lsp_types::DidOpenTextDocumentParams,
) -> Result<(), String> {
    let uri = params.text_document.uri;
    if uri
        .scheme()
        .map(|scheme| scheme.as_str() != "file")
        .unwrap_or(true)
    {
        return Ok(());
    }
    let local_path = std::path::Path::new("./").join(
        <std::path::PathBuf as std::str::FromStr>::from_str(
            uri.path().as_str(),
        )
        .unwrap()
        .strip_prefix(&context.config_directory_path)
        .unwrap(),
    );
    {
        let mut write_buffers = context.open_buffers.write().await;
        write_buffers.insert(
            local_path.clone(),
            OpenBuffer {
                content: std::fs::read_to_string(&local_path)
                    .map_err(|e| e.to_string())?,
                uri: uri.clone(),
                version: params.text_document.version,
            },
        );
    }
    publish_workspace_diagnostics(
        context,
        writer,
        &local_path,
        &context.open_buffers.read().await,
    )
    .await
}
