use crate::parsers::{file, schema::type_registry::TypeRegistry};

pub mod ast;
pub mod directive;
pub mod errors;
pub mod fragment;
pub mod hash;
pub mod nodes;
pub mod operation;
pub mod schema;

pub fn parse_client_schema<'buffer>(
    registry: &mut TypeRegistry,
    ast_nodes: &[file::client::ast::ASTNode<'buffer>],
) -> Result<schema::ClientSchema, errors::Error<'buffer>> {
    let client_nodes = ast_nodes
        .iter()
        .map(|node| nodes::parse_first_pass(registry, node))
        .collect::<Result<Vec<_>, errors::Error>>()?;
    client_nodes.iter().for_each(|node| {
        registry.add_client_node(node);
    });
    ast_nodes
        .iter()
        .try_for_each(|node| nodes::parse_second_pass(registry, node))?;
    for operation in client_nodes.iter().filter_map(|node| match node {
        ast::ClientSchemaNode::Operation(operation) => Some(operation),
        _ => None,
    }) {
        let parameters_hash = hash::get_operation_parameters_hash(
            &operation.read().unwrap().parameters,
        );
        operation.write().unwrap().parameters_hash = parameters_hash;
        let fragment_spec_hash = hash::get_fragment_spec_hash(
            registry,
            &operation.read().unwrap().fragment_spec,
            true,
        );
        operation.write().unwrap().fragment_spec_hash = fragment_spec_hash;
        let used_fragments = hash::get_used_fragments_from_fragment_spec(
            registry,
            &operation.read().unwrap().fragment_spec,
        );
        operation.write().unwrap().used_fragments = used_fragments;
    }
    return Ok(schema::ClientSchema::from_nodes(&client_nodes));
}
