use crate::parsers::file;

fn get_source_text(
    source_buffer: &str,
    location: &file::shared::ast::NodeLocation,
) -> String {
    source_buffer[location.start..location.end].to_string()
}

pub fn extract_from_fragment(
    node: &file::client::ast::FragmentDefinition,
) -> String {
    get_source_text(&node.location.source.buffer, &node.location)
}

pub fn extract_from_operation(
    node: &file::client::ast::OperationDefinition,
) -> String {
    get_source_text(&node.location.source.buffer, &node.location)
}
