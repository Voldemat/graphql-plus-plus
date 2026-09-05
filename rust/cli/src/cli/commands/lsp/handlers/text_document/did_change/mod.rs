use crate::cli::commands::lsp::{
    codec::LspCodec, context::ServerContext,
    location::lsp_range_to_index_range, shared::publish_workspace_diagnostics,
};

pub async fn handler(
    context: &ServerContext,
    writer: std::sync::Arc<
        tokio::sync::Mutex<
            tokio_util::codec::FramedWrite<tokio::io::Stdout, LspCodec>,
        >,
    >,
    params: lsp_types::DidChangeTextDocumentParams,
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
        let buffer = write_buffers.get_mut(&local_path).unwrap();
        buffer.version = params.text_document.version;
        for change in params.content_changes {
            let Some(range) = change.range else {
                buffer.content = change.text;
                continue;
            };
            let Some(_) = change.range_length else {
                buffer.content = change.text;
                continue;
            };
            let new_line_positions = buffer
                .content
                .bytes()
                .enumerate()
                .filter_map(|(index, c)| match c {
                    b'\n' => Some(index),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let index_range =
                lsp_range_to_index_range(&new_line_positions, range);
            buffer.content.replace_range(index_range, &change.text);
        }
    };
    publish_workspace_diagnostics(
        context,
        writer,
        &local_path,
        &context.open_buffers.read().await,
    )
    .await
}
