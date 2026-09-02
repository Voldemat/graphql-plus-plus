pub fn buffer_to_client_ast<'buffer>(
    graphql_path: &std::path::PathBuf,
    buffer: &'buffer str,
) -> Result<Vec<libgql::parsers::file::client::ast::ASTNode<'buffer>>, String> {
    let source_file =
        std::sync::Arc::new(libgql::parsers::file::shared::ast::SourceFile {
            filepath: graphql_path.clone(),
            buffer: buffer,
        });
    let tokens =
        libgql::lexer::utils::parse_buffer_into_tokens(&source_file.buffer)
            .unwrap();
    libgql::parsers::file::client::Parser::new(
        libgql::parsers::file::tokens_sources::VecTokensSource::new(
            tokens,
            source_file.clone(),
        ),
    )
    .parse_ast_nodes()
    .map_err(|e| {
        super::format_error::format_parse_error(
            &format!("{}", e),
            &e.get_location(),
            &source_file,
        )
    })
}

pub fn buffer_to_server_ast<'buffer>(
    graphql_path: &std::path::PathBuf,
    buffer: &'buffer str,
) -> Result<Vec<libgql::parsers::file::server::ast::ASTNode<'buffer>>, String> {
    let source_file =
        std::sync::Arc::new(libgql::parsers::file::shared::ast::SourceFile {
            filepath: graphql_path.clone(),
            buffer: buffer,
        });
    let tokens =
        libgql::lexer::utils::parse_buffer_into_tokens(&source_file.buffer)
            .unwrap();
    libgql::parsers::file::server::Parser::new(
        libgql::parsers::file::tokens_sources::VecTokensSource::new(
            tokens,
            source_file.clone(),
        ),
    )
    .parse_ast_nodes()
    .map_err(|e| {
        super::format_error::format_parse_error(
            &format!("{}", e),
            e.get_location(),
            &source_file,
        )
    })
}
