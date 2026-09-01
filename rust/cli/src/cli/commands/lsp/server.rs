use super::meta::ServerMetadata;

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
        super::handlers::text_document::formatting::handler,
    );
    io
}
