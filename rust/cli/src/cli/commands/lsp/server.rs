use super::context::ServerContext;

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + 'a>>;
pub type Handler<'c, TContext> = dyn Fn(
        &'c TContext,
        std::sync::Arc<
            tokio::sync::Mutex<
                tokio_util::codec::FramedWrite<
                    tokio::io::Stdout,
                    super::codec::LspCodec,
                >,
            >,
        >,
        serde_json::Value,
    ) -> BoxFuture<'c, serde_json::Value>
    + 'c;

#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

#[derive(Debug, serde::Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, serde::Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcError {
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self {
            code: -32700,
            message: message.into(),
            data: None,
        }
    }

    pub fn method_not_found(method: &str) -> Self {
        Self {
            code: -32601,
            message: format!("Method not found: {}", method),
            data: None,
        }
    }

    pub fn internal_error(msg: impl Into<String>) -> Self {
        Self {
            code: -32603,
            message: msg.into(),
            data: None,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct JsonRpcNotification<T> {
    pub jsonrpc: &'static str,
    pub method: String,
    pub params: T,
}

pub async fn send_notification<T: serde::Serialize>(
    writer: &std::sync::Arc<
        tokio::sync::Mutex<
            tokio_util::codec::FramedWrite<
                tokio::io::Stdout,
                super::codec::LspCodec,
            >,
        >,
    >,
    method: impl Into<String>,
    params: T,
) -> Result<(), String> {
    let notification = JsonRpcNotification {
        jsonrpc: "2.0",
        method: method.into(),
        params,
    };
    send_message(
        writer,
        serde_json::to_string(&notification).map_err(|e| e.to_string())?,
    )
    .await
}

pub async fn send_message(
    writer: &std::sync::Arc<
        tokio::sync::Mutex<
            tokio_util::codec::FramedWrite<
                tokio::io::Stdout,
                super::codec::LspCodec,
            >,
        >,
    >,
    message: String,
) -> Result<(), String> {
    let arc = writer.clone();
    let mut writer = arc.lock().await;
    let a = std::ops::DerefMut::deref_mut(&mut writer);
    futures_util::SinkExt::send(a, message)
        .await
        .map_err(|e| e.to_string())
}

pub async fn send_response(
    writer: &std::sync::Arc<
        tokio::sync::Mutex<
            tokio_util::codec::FramedWrite<
                tokio::io::Stdout,
                super::codec::LspCodec,
            >,
        >,
    >,
    response: JsonRpcResponse,
) -> Result<(), String> {
    send_message(
        writer,
        serde_json::to_string(&response).map_err(|error| error.to_string())?,
    )
    .await
}

pub struct Server<'c, TContext> {
    pub context: &'c TContext,
    pub writer: std::sync::Arc<
        tokio::sync::Mutex<
            tokio_util::codec::FramedWrite<
                tokio::io::Stdout,
                super::codec::LspCodec,
            >,
        >,
    >,
    pub handlers: std::collections::HashMap<String, Box<Handler<'c, TContext>>>,
}

impl<'c, TContext> Server<'c, TContext> {
    pub fn add_method<
        F: Fn(
                &'c TContext,
                std::sync::Arc<
                    tokio::sync::Mutex<
                        tokio_util::codec::FramedWrite<
                            tokio::io::Stdout,
                            super::codec::LspCodec,
                        >,
                    >,
                >,
                serde_json::Value,
            ) -> BoxFuture<'c, serde_json::Value>
            + 'c,
    >(
        self: &mut Self,
        name: impl Into<String>,
        handler: F,
    ) {
        self.handlers.insert(name.into(), Box::new(handler));
    }

    pub fn add_handler<
        TParams: for<'p> serde::Deserialize<'p>,
        TResult: serde::Serialize,
        Fut: Future<Output = Result<TResult, String>> + 'c,
        F: Fn(
                &'c TContext,
                std::sync::Arc<
                    tokio::sync::Mutex<
                        tokio_util::codec::FramedWrite<
                            tokio::io::Stdout,
                            super::codec::LspCodec,
                        >,
                    >,
                >,
                TParams,
            ) -> Fut
            + 'c,
    >(
        self: &mut Self,
        name: impl Into<String>,
        handler: F,
    ) {
        self.add_method(name, move |context, writer, rpc_params| {
            let fut = handler(
                context,
                writer,
                match serde_json::from_value(rpc_params) {
                    Ok(params) => params,
                    Err(error) => {
                        let err = JsonRpcError::parse_error(error.to_string());
                        return Box::pin(async move {
                            serde_json::to_value(err).unwrap_or_default()
                        });
                    }
                },
            );
            Box::pin(futures_util::FutureExt::map(fut, |result| match result {
                Ok(r) => serde_json::to_value(r).unwrap(),
                Err(error) => {
                    let err = JsonRpcError::internal_error(error);
                    serde_json::to_value(err).unwrap_or_default()
                }
            }))
        });
    }

    pub async fn handle_request(
        self: &mut Self,
        request_str: &str,
    ) -> Result<(), String> {
        let request: JsonRpcRequest = match serde_json::from_str(request_str) {
            Ok(parsed) => parsed,
            Err(error) => {
                let err_resp = JsonRpcResponse {
                    jsonrpc: "2.0",
                    id: None,
                    result: None,
                    error: Some(JsonRpcError::parse_error(error.to_string())),
                };
                send_response(&self.writer, err_resp).await?;
                return Ok(());
            }
        };
        let is_notification = request.id.is_none();
        match self.handlers.get(&request.method) {
            Some(handler) => {
                let result =
                    handler(self.context, self.writer.clone(), request.params)
                        .await;
                if is_notification {
                    return Ok(());
                }

                let response = JsonRpcResponse {
                    jsonrpc: "2.0",
                    id: request.id,
                    result: Some(result),
                    error: None,
                };
                send_response(&self.writer, response).await
            }
            None => {
                if is_notification {
                    return Ok(());
                }

                let response = JsonRpcResponse {
                    jsonrpc: "2.0",
                    id: request.id,
                    result: None,
                    error: Some(JsonRpcError::method_not_found(
                        &request.method,
                    )),
                };
                send_response(&self.writer, response).await
            }
        }
    }
}

pub fn build_jsonrpc_server<'c>(
    context: &'c ServerContext,
    writer: std::sync::Arc<
        tokio::sync::Mutex<
            tokio_util::codec::FramedWrite<
                tokio::io::Stdout,
                super::codec::LspCodec,
            >,
        >,
    >,
) -> Server<'c, ServerContext> {
    let mut server = Server {
        context,
        writer,
        handlers: Default::default(),
    };
    server.add_handler(
        "initialize",
        |_context, _writer, _params: lsp_types::InitializeParams| async {
            Ok(lsp_types::InitializeResult {
                capabilities: lsp_types::ServerCapabilities {
                    document_formatting_provider: Some(lsp_types::OneOf::Left(
                        true,
                    )),
                    text_document_sync: Some(
                        lsp_types::TextDocumentSyncOptions {
                            open_close: Some(true),
                            change: Some(
                                lsp_types::TextDocumentSyncKind::INCREMENTAL,
                            ),
                            ..Default::default()
                        }
                        .into(),
                    ),
                    ..Default::default()
                },
                server_info: None,
            })
        },
    );
    server.add_handler("shutdown", |_context, _writer, _params: ()| async {
        Ok(())
    });
    server.add_handler(
        "textDocument/didOpen",
        super::handlers::text_document::did_open::handler,
    );
    server.add_handler(
        "textDocument/didChange",
        super::handlers::text_document::did_change::handler,
    );
    server.add_handler(
        "textDocument/formatting",
        super::handlers::text_document::formatting::handler,
    );
    server
}
