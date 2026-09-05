pub struct TokensToASTResult<TASTNode, TParserError> {
    pub parser_errors: Vec<TParserError>,
    pub ast_nodes: Vec<TASTNode>,
}

pub fn buffer_to_client_ast<'buffer>(
    source_file: &std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'buffer>,
    >,
    tokens: Vec<libgql::lexer::tokens::Token<'buffer>>,
) -> TokensToASTResult<
    libgql::parsers::file::client::ast::ASTNode<'buffer>,
    libgql::parsers::file::client::Error<'buffer>,
> {
    let mut parser_errors = Vec::new();
    let mut ast_nodes = Vec::new();
    match libgql::parsers::file::client::Parser::new(
        libgql::parsers::file::tokens_sources::VecTokensSource::new(
            tokens,
            source_file.clone(),
        ),
    )
    .parse_ast_nodes()
    {
        Ok(nodes) => ast_nodes.extend(nodes),
        Err(error) => parser_errors.push(error),
    };

    TokensToASTResult {
        parser_errors: parser_errors,
        ast_nodes: ast_nodes,
    }
}

pub fn buffer_to_server_ast<'buffer>(
    source_file: &std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'buffer>,
    >,
    tokens: Vec<libgql::lexer::tokens::Token<'buffer>>,
) -> TokensToASTResult<
    libgql::parsers::file::server::ast::ASTNode<'buffer>,
    libgql::parsers::file::server::Error<'buffer>,
> {
    let mut parser_errors = Vec::new();
    let mut ast_nodes = Vec::new();
    match libgql::parsers::file::server::Parser::new(
        libgql::parsers::file::tokens_sources::VecTokensSource::new(
            tokens,
            source_file.clone(),
        ),
    )
    .parse_ast_nodes()
    {
        Ok(nodes) => ast_nodes.extend(nodes),
        Err(error) => parser_errors.push(error),
    };

    TokensToASTResult {
        parser_errors: parser_errors,
        ast_nodes: ast_nodes,
    }
}

#[derive(Debug)]
pub struct OpenBuffer {
    pub content: String,
    pub uri: lsp_types::Uri,
    pub version: i32,
}

pub type OpenBuffers =
    std::collections::HashMap<std::path::PathBuf, OpenBuffer>;
