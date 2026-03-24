use crate::{lexer, parsers::file};

fn get_source_text(
    source_buffer: &str,
    start_token_location: &lexer::tokens::TokenLocation,
    end_token_location: &lexer::tokens::TokenLocation,
) -> String {
    source_buffer[start_token_location.start..end_token_location.end]
        .to_string()
}

pub fn extract_from_fragment(
    node: &file::client::ast::FragmentDefinition,
) -> String {
    get_source_text(
        &node.location.source.buffer,
        &node.location.start_token.location,
        &node.location.end_token.location,
    )
}

pub fn extract_from_operation(
    node: &file::client::ast::OperationDefinition,
) -> String {
    get_source_text(
        &node.location.source.buffer,
        &node.location.start_token.location,
        &node.location.end_token.location,
    )
}
