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
    format_error::format_error,
    shared::TokensToASTResult,
};

fn format_file<
    TASTNodeWrapper: crate::cli::commands::format::shared::ASTNodeWrapper,
    TTokensToASTNodes: for<'buffer> Fn(
        &std::sync::Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
        Vec<libgql::lexer::tokens::Token<'buffer>>,
    ) -> TokensToASTResult<
        TASTNodeWrapper::ASTNode<'buffer>,
        TASTNodeWrapper::ParserError<'buffer>,
    >,
    TASTNodesToHIRNodes: for<'buffer> Fn(
        &'buffer str,
        Vec<TASTNodeWrapper::ASTNode<'buffer>>,
    ) -> Vec<codeform::ir::hir::node::Node<'buffer>>,
>(
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
    tokens_to_ast_nodes: TTokensToASTNodes,
    ast_nodes_to_hir_nodes: TASTNodesToHIRNodes,
    local_path: &std::path::PathBuf,
    buffer: &str,
) -> Result<Vec<lsp_types::TextEdit>, Vec<String>> {
    let mut writer = std::io::BufWriter::new(Vec::<u8>::new());
    let parse_result = libgql::lexer::utils::parse_buffer(buffer);
    if parse_result.errors.len() > 0 {
        return Err(parse_result
            .errors
            .into_iter()
            .map(|error| {
                format_error(
                    &error.to_string(),
                    error.get_location(),
                    local_path,
                    buffer,
                )
            })
            .collect());
    }
    let source_file =
        std::sync::Arc::new(libgql::parsers::file::shared::ast::SourceFile {
            filepath: local_path.clone(),
            buffer: buffer,
            new_line_positions: parse_result.new_line_positions,
        });
    let lir_nodes = format_buffer_to_lir_nodes::<
        TASTNodeWrapper,
        TTokensToASTNodes,
        TASTNodesToHIRNodes,
    >(
        &source_file,
        parse_result.tokens,
        tokens_to_ast_nodes,
        ast_nodes_to_hir_nodes,
        shared_formatting_config,
    )?;
    print_lir_nodes(&mut writer, shared_formatting_config, lir_nodes)
        .map_err(|e| vec![format!("LIR printer error: {}", e)])?;
    let formatted_string =
        String::from_utf8(writer.into_inner().unwrap()).unwrap();
    Ok(lsp_edits::generate(source_file.buffer, &formatted_string))
}

fn format_server_file<'buffer>(
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
    server_formatting_config: &crate::cli::config::GraphqlFormattingServerConfig,
    local_path: &std::path::PathBuf,
    buffer: &str,
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
        local_path,
        buffer,
    )
}

fn format_client_file<'buffer>(
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
    client_formatting_config: &crate::cli::config::GraphqlFormattingClientConfig,
    local_path: &std::path::PathBuf,
    buffer: &str,
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
        local_path,
        buffer,
    )
}

fn format_file_with_type<'buffer>(
    formatting_config: &crate::cli::config::GraphqlFormattingConfig,
    local_path: &std::path::PathBuf,
    buffer: &str,
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
                    local_path,
                    buffer,
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
                    buffer,
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
    let buffer = get_buffer(&context.open_buffers, &local_path).await?;
    get_file_type(&context.config, &local_path)
        .and_then(|file_type| {
            context.config.formatting.as_ref().map(|formatting_config| {
                format_file_with_type(
                    formatting_config,
                    &local_path,
                    &buffer,
                    file_type,
                )
                .map_err(|_| format!("Parsing errors"))
            })
        })
        .unwrap_or(Ok(Vec::new()))
}
