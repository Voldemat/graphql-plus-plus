mod lsp_edits;

use crate::cli::{
    commands::{
        format::shared::{format_buffer_to_lir_nodes, print_lir_nodes},
        lsp::{
            codec::LspCodec,
            context::ServerContext,
            file_type::{FileType, get_file_type},
            shared::get_buffer,
        },
    },
    shared::BufferToASTResult,
};

fn format_file<
    TASTNodeWrapper: crate::cli::commands::format::shared::ASTNodeWrapper,
    TBufferToASTNodes: for<'buffer> Fn(
        &std::sync::Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
    ) -> BufferToASTResult<
        TASTNodeWrapper::ASTNode<'buffer>,
        TASTNodeWrapper::ParserError<'buffer>,
    >,
    TASTNodesToHIRNodes: for<'buffer> Fn(
        &'buffer str,
        Vec<TASTNodeWrapper::ASTNode<'buffer>>,
    ) -> Vec<codeform::ir::hir::node::Node<'buffer>>,
>(
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
    buffer_to_ast_nodes: TBufferToASTNodes,
    ast_nodes_to_hir_nodes: TASTNodesToHIRNodes,
    source_file: std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'_>,
    >,
) -> Result<Vec<lsp_types::TextEdit>, Vec<String>> {
    let mut writer = std::io::BufWriter::new(Vec::<u8>::new());
    let lir_nodes = format_buffer_to_lir_nodes::<
        TASTNodeWrapper,
        TBufferToASTNodes,
        TASTNodesToHIRNodes,
    >(
        &source_file,
        buffer_to_ast_nodes,
        ast_nodes_to_hir_nodes,
        shared_formatting_config,
    )?;
    print_lir_nodes(&mut writer, shared_formatting_config, lir_nodes)
        .map_err(|e| vec![format!("LIR printer error: {}", e)])?;
    let formatted_string =
        String::from_utf8(writer.into_inner().unwrap()).unwrap();
    Ok(lsp_edits::generate(source_file.buffer, &formatted_string))
}

fn format_server_file(
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
    server_formatting_config: &crate::cli::config::GraphqlFormattingServerConfig,
    source_file: std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'_>,
    >,
) -> Result<Vec<lsp_types::TextEdit>, Vec<String>> {
    format_file::<
        crate::cli::commands::format::shared::ServerASTNodeWrapper,
        _,
        _,
    >(
        shared_formatting_config,
        crate::cli::shared::buffer_to_server_ast,
        |_, nodes| {
            libgql::formatter::server::nodes::format_nodes(
                shared_formatting_config,
                server_formatting_config,
                &nodes,
            )
            .to_vec()
        },
        source_file,
    )
}

fn format_client_file(
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
    client_formatting_config: &crate::cli::config::GraphqlFormattingClientConfig,
    source_file: std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'_>,
    >,
) -> Result<Vec<lsp_types::TextEdit>, Vec<String>> {
    format_file::<
        crate::cli::commands::format::shared::ClientASTNodeWrapper,
        _,
        _,
    >(
        &shared_formatting_config,
        crate::cli::shared::buffer_to_client_ast,
        |_, nodes| {
            libgql::formatter::client::nodes::format_nodes(
                shared_formatting_config,
                client_formatting_config,
                &nodes,
            )
            .to_vec()
        },
        source_file,
    )
}

fn format_file_with_type(
    formatting_config: &crate::cli::config::GraphqlFormattingConfig,
    source_file: std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'_>,
    >,
    file_type: FileType,
) -> Result<Vec<lsp_types::TextEdit>, Vec<String>> {
    match file_type {
        FileType::Server => formatting_config
            .server
            .as_ref()
            .map(|formatting_server_config| {
                format_server_file(
                    &formatting_config.shared,
                    formatting_server_config,
                    source_file,
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
                    source_file,
                )
            })
            .unwrap_or(Ok(Vec::new())),
    }
}

pub async fn handler(
    context: &ServerContext,
    _: std::sync::Arc<
        tokio::sync::Mutex<
            tokio_util::codec::FramedWrite<tokio::io::Stdout, LspCodec>,
        >,
    >,
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
        .strip_prefix(&context.config_directory_path)
        .unwrap(),
    );
    let buffer = get_buffer(&context.buffers, &local_path).await?;
    let source_file =
        std::sync::Arc::new(libgql::parsers::file::shared::ast::SourceFile {
            filepath: local_path.clone(),
            buffer: &buffer,
        });
    get_file_type(&context.config, &local_path)
        .and_then(|file_type| {
            context.config.formatting.as_ref().map(|formatting_config| {
                format_file_with_type(formatting_config, source_file, file_type)
                    .map_err(|_| format!("Parsing errors"))
            })
        })
        .unwrap_or(Ok(Vec::new()))
}
