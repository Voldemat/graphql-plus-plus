use crate::parsers::file;

fn get_source_text<'buffer>(
    source_buffer: &'buffer str,
    location: &file::shared::ast::NodeLocation,
) -> &'buffer str {
    &source_buffer[location.start..location.end]
}

pub fn extract_from_fragment<'buffer>(
    node: &file::client::ast::FragmentDefinition<'buffer>,
) -> &'buffer str {
    get_source_text(&node.location.source.buffer, &node.location)
}

pub fn extract_from_operation<'buffer>(
    node: &file::client::ast::OperationDefinition<'buffer>,
) -> &'buffer str {
    get_source_text(&node.location.source.buffer, &node.location)
}
