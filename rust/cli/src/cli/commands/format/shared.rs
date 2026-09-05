use crate::cli::{format_error::format_error, shared::TokensToASTResult};

pub fn print_lir_nodes<TWriter: std::io::Write>(
    writer: &mut TWriter,
    config: &impl codeform::lir_printer::Config,
    lir_nodes: Vec<codeform::ir::lir::node::Node<'_>>,
) -> std::io::Result<()> {
    let mut printer_state = codeform::lir_printer::State::default();
    codeform::lir_printer::print_nodes(
        writer,
        config,
        &mut printer_state,
        &lir_nodes,
    )
}

pub fn format_print_action<
    'buffer,
    TASTNodeWrapper: ASTNodeWrapper,
    TTokensToASTNodes: Fn(
        &std::sync::Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
        Vec<libgql::lexer::tokens::Token<'buffer>>,
    ) -> TokensToASTResult<
        TASTNodeWrapper::ASTNode<'buffer>,
        TASTNodeWrapper::ParserError<'buffer>,
    >,
    TASTNodesToHIRNodes: Fn(
        &'buffer str,
        Vec<TASTNodeWrapper::ASTNode<'buffer>>,
    ) -> Vec<codeform::ir::hir::node::Node<'buffer>>,
>(
    source_file: &std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'buffer>,
    >,
    tokens: Vec<libgql::lexer::tokens::Token<'buffer>>,
    tokens_to_ast_nodes: TTokensToASTNodes,
    ast_nodes_to_hir_nodes: TASTNodesToHIRNodes,
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
) -> Result<(), Vec<String>> {
    let lir_nodes = format_buffer_to_lir_nodes::<
        TASTNodeWrapper,
        TTokensToASTNodes,
        TASTNodesToHIRNodes,
    >(
        source_file,
        tokens,
        tokens_to_ast_nodes,
        ast_nodes_to_hir_nodes,
        shared_formatting_config,
    )?;
    let mut writer = std::io::BufWriter::new(
        std::fs::File::create(&source_file.filepath).unwrap(),
    );
    print_lir_nodes(&mut writer, shared_formatting_config, lir_nodes)
        .map_err(|e| vec![format!("LIR printer error: {}", e)])
}

fn format_check_action<
    'buffer,
    TASTNodeWrapper: ASTNodeWrapper,
    TTokensToASTNodes: Fn(
        &std::sync::Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
        Vec<libgql::lexer::tokens::Token<'buffer>>,
    ) -> TokensToASTResult<
        TASTNodeWrapper::ASTNode<'buffer>,
        TASTNodeWrapper::ParserError<'buffer>,
    >,
    TASTNodesToHIRNodes: Fn(
        &'buffer str,
        Vec<TASTNodeWrapper::ASTNode<'buffer>>,
    ) -> Vec<codeform::ir::hir::node::Node<'buffer>>,
>(
    source_file: &std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'buffer>,
    >,
    tokens: Vec<libgql::lexer::tokens::Token<'buffer>>,
    tokens_to_ast_nodes: TTokensToASTNodes,
    ast_nodes_to_hir_nodes: TASTNodesToHIRNodes,
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
) -> Result<(), Vec<String>> {
    let mut writer = std::io::BufWriter::new(Vec::<u8>::new());
    let lir_nodes = format_buffer_to_lir_nodes::<
        TASTNodeWrapper,
        TTokensToASTNodes,
        TASTNodesToHIRNodes,
    >(
        source_file,
        tokens,
        tokens_to_ast_nodes,
        ast_nodes_to_hir_nodes,
        shared_formatting_config,
    )?;
    print_lir_nodes(&mut writer, shared_formatting_config, lir_nodes)
        .map_err(|e| vec![format!("LIR printer error: {}", e)])?;
    let formatted_string =
        String::from_utf8(writer.into_inner().unwrap()).unwrap();
    match super::text_diff::get_diff_string(
        source_file.buffer,
        &formatted_string,
    ) {
        None => Ok(()),
        Some(diff_string) => Err(vec![format!(
            "{}\n{}",
            console::style(format!(
                "{}:",
                source_file.filepath.to_string_lossy()
            ))
            .blue(),
            diff_string
        )]),
    }
}

pub fn format_action<
    'buffer,
    TASTNodeWrapper: ASTNodeWrapper,
    TTokensToASTNodes: Fn(
        &std::sync::Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
        Vec<libgql::lexer::tokens::Token<'buffer>>,
    ) -> TokensToASTResult<
        TASTNodeWrapper::ASTNode<'buffer>,
        TASTNodeWrapper::ParserError<'buffer>,
    >,
    TASTNodesToHIRNodes: Fn(
        &'buffer str,
        Vec<TASTNodeWrapper::ASTNode<'buffer>>,
    ) -> Vec<codeform::ir::hir::node::Node<'buffer>>,
>(
    is_check: bool,
    source_file: &std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'buffer>,
    >,
    tokens: Vec<libgql::lexer::tokens::Token<'buffer>>,
    tokens_to_ast_nodes: TTokensToASTNodes,
    ast_nodes_to_hir_nodes: TASTNodesToHIRNodes,
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
) -> Result<(), Vec<String>> {
    if is_check {
        format_check_action::<TASTNodeWrapper, _, _>(
            source_file,
            tokens,
            tokens_to_ast_nodes,
            ast_nodes_to_hir_nodes,
            shared_formatting_config,
        )
    } else {
        format_print_action::<TASTNodeWrapper, _, _>(
            source_file,
            tokens,
            tokens_to_ast_nodes,
            ast_nodes_to_hir_nodes,
            shared_formatting_config,
        )
    }
}

pub trait ASTNodeWrapper {
    type ASTNode<'buffer>;
    type ParserError<'buffer>: libgql::parsers::file::shared::error::Error;
}

pub struct ClientASTNodeWrapper {}

impl ASTNodeWrapper for ClientASTNodeWrapper {
    type ASTNode<'buffer> =
        libgql::parsers::file::client::ast::ASTNode<'buffer>;
    type ParserError<'buffer> = libgql::parsers::file::client::Error<'buffer>;
}

pub struct ServerASTNodeWrapper {}

impl ASTNodeWrapper for ServerASTNodeWrapper {
    type ASTNode<'buffer> =
        libgql::parsers::file::server::ast::ASTNode<'buffer>;
    type ParserError<'buffer> = libgql::parsers::file::server::Error<'buffer>;
}

pub fn format_buffer_to_lir_nodes<
    'buffer,
    TASTNodeWrapper: ASTNodeWrapper,
    TTokensToASTNodes: Fn(
        &std::sync::Arc<libgql::parsers::file::shared::ast::SourceFile<'buffer>>,
        Vec<libgql::lexer::tokens::Token<'buffer>>,
    ) -> TokensToASTResult<
        TASTNodeWrapper::ASTNode<'buffer>,
        TASTNodeWrapper::ParserError<'buffer>,
    >,
    TASTNodesToHIRNodes: Fn(
        &'buffer str,
        Vec<TASTNodeWrapper::ASTNode<'buffer>>,
    ) -> Vec<codeform::ir::hir::node::Node<'buffer>>,
>(
    source_file: &std::sync::Arc<
        libgql::parsers::file::shared::ast::SourceFile<'buffer>,
    >,
    tokens: Vec<libgql::lexer::tokens::Token<'buffer>>,
    tokens_to_ast_nodes: TTokensToASTNodes,
    ast_nodes_to_hir_nodes: TASTNodesToHIRNodes,
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
) -> Result<Vec<codeform::ir::lir::node::Node<'buffer>>, Vec<String>> {
    let result = tokens_to_ast_nodes(source_file, tokens);
    if result.parser_errors.len() > 0 {
        let mut errors = Vec::new();
        errors.extend(result.parser_errors.into_iter().map(|parser_error| {
            format_error(
                &parser_error.to_string(),
                libgql::parsers::file::shared::error::Error::get_location(
                    &parser_error,
                ),
                &source_file.filepath,
                source_file.buffer,
            )
        }));
        return Err(errors);
    }
    let hir_nodes =
        ast_nodes_to_hir_nodes(source_file.buffer, result.ast_nodes);
    let mut hir_to_lir_state = codeform::hir_to_lir::state::State::default();
    return Ok(codeform::hir_to_lir::mappers::nodes::lower(
        shared_formatting_config,
        &mut hir_to_lir_state,
        hir_nodes,
    ));
}

pub fn format_config<
    TASTNodeWrapper: ASTNodeWrapper,
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
    graphql_paths: &[std::path::PathBuf],
    tokens_to_ast_nodes: TTokensToASTNodes,
    ast_nodes_to_hir_nodes: TASTNodesToHIRNodes,
    shared_formatting_config: &crate::cli::config::GraphqlFormattingSharedConfig,
    is_check: bool,
) -> Vec<String> {
    graphql_paths
        .into_iter()
        .map(|graphql_path| -> Vec<String> {
            let buffer = std::fs::read_to_string(&graphql_path).unwrap();
            let parse_result = libgql::lexer::utils::parse_buffer(&buffer);
            let source_file = std::sync::Arc::new(
                libgql::parsers::file::shared::ast::SourceFile {
                    filepath: graphql_path.clone(),
                    buffer: &buffer,
                    new_line_positions: parse_result.new_line_positions,
                },
            );
            format_action::<TASTNodeWrapper, _, _>(
                is_check,
                &source_file,
                parse_result.tokens,
                &tokens_to_ast_nodes,
                &ast_nodes_to_hir_nodes,
                shared_formatting_config,
            )
            .err()
            .unwrap_or(Vec::new())
            .into_iter()
            .chain(parse_result.errors.into_iter().map(|error| {
                format_error(
                    &error.to_string(),
                    error.get_location(),
                    &source_file.filepath,
                    source_file.buffer,
                )
            }))
            .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}
