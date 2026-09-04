use libgql::lexer::utils::NewLinePositions;

pub struct BufferToASTResult<TASTNode, TParserError> {
    pub new_line_positions: NewLinePositions,
    pub lexer_errors: Vec<libgql::lexer::types::Error>,
    pub parser_errors: Vec<TParserError>,
    pub ast_nodes: Vec<TASTNode>,
}

pub fn buffer_to_client_ast<'buffer>(
    source_file: &std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'buffer>,
    >,
) -> BufferToASTResult<
    libgql::parsers::file::client::ast::ASTNode<'buffer>,
    libgql::parsers::file::client::Error<'buffer>,
> {
    let lexing_result = libgql::lexer::utils::parse_buffer(&source_file.buffer);
    let mut parser_errors = Vec::new();
    let mut ast_nodes = Vec::new();
    match libgql::parsers::file::client::Parser::new(
        libgql::parsers::file::tokens_sources::VecTokensSource::new(
            lexing_result.tokens,
            source_file.clone(),
        ),
    )
    .parse_ast_nodes()
    {
        Ok(nodes) => ast_nodes.extend(nodes),
        Err(error) => parser_errors.push(error),
    };

    BufferToASTResult {
        new_line_positions: lexing_result.new_line_positions,
        lexer_errors: lexing_result.errors,
        parser_errors: parser_errors,
        ast_nodes: ast_nodes,
    }
}

pub fn buffer_to_server_ast<'buffer>(
    source_file: &std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'buffer>,
    >,
) -> BufferToASTResult<
    libgql::parsers::file::server::ast::ASTNode<'buffer>,
    libgql::parsers::file::server::Error<'buffer>,
> {
    let lexing_result = libgql::lexer::utils::parse_buffer(&source_file.buffer);
    let mut parser_errors = Vec::new();
    let mut ast_nodes = Vec::new();
    match libgql::parsers::file::server::Parser::new(
        libgql::parsers::file::tokens_sources::VecTokensSource::new(
            lexing_result.tokens,
            source_file.clone(),
        ),
    )
    .parse_ast_nodes()
    {
        Ok(nodes) => ast_nodes.extend(nodes),
        Err(error) => parser_errors.push(error),
    };

    BufferToASTResult {
        new_line_positions: lexing_result.new_line_positions,
        lexer_errors: lexing_result.errors,
        parser_errors: parser_errors,
        ast_nodes: ast_nodes,
    }
}
