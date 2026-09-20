use crate::cli::commands::lsp::{codec::LspCodec, context::ServerContext};

#[derive(serde::Serialize)]
pub struct GQLSourcePatterns {
    client: Vec<std::path::PathBuf>,
    server: Vec<std::path::PathBuf>,
}

pub async fn handler(
    context: &ServerContext,
    _: std::sync::Arc<
        tokio::sync::Mutex<
            tokio_util::codec::FramedWrite<tokio::io::Stdout, LspCodec>,
        >,
    >,
    params: lsp_types::ExecuteCommandParams,
) -> Result<GQLSourcePatterns, String> {
    if params.command != "gql.GetGQLSourcePatterns" {
        return Err(format!("Unexpected command: {}", params.command));
    }
    Ok(GQLSourcePatterns {
        client: context
            .config
            .client
            .as_ref()
            .map(|c| c.inputs.graphql.clone())
            .unwrap_or_else(|| Vec::new()),
        server: context
            .config
            .server
            .as_ref()
            .map(|c| c.inputs.graphql.clone())
            .unwrap_or_else(|| Vec::new()),
    })
}
