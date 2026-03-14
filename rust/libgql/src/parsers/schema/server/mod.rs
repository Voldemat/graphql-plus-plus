pub mod ast;
pub mod directive;
pub mod errors;
pub mod input;
pub mod interface;
pub mod nodes;
pub mod object;
pub mod schema;
pub mod union;

pub use errors::Error;
use crate::parsers::{file, schema::type_registry::TypeRegistry};

pub fn parse_server_schema(
    mut registry: &mut TypeRegistry,
    ast_nodes: &[file::server::ast::ASTNode],
) -> Result<schema::Schema, errors::Error> {
    let type_definition_nodes = || {
        ast_nodes.iter().filter_map(|node| match node {
            file::server::ast::ASTNode::TypeDefinitionNode(n) => Some(n),
            _ => None,
        })
    };
    type_definition_nodes().for_each(|node| {
        registry.add_server_node(nodes::parse_server_node_first_pass(node));
    });
    let server_nodes = type_definition_nodes()
        .map(|node| nodes::parse_server_node_second_pass(node, &mut registry))
        .collect::<Result<Vec<_>, errors::Error>>()?;
    ast_nodes
        .iter()
        .filter_map(|node| match node {
            file::server::ast::ASTNode::ExtendTypeNode(n) => Some(n),
            _ => None,
        })
        .map(|node| nodes::parse_server_extend_node(node, &mut registry))
        .collect::<Result<Vec<_>, errors::Error>>()?
        .iter()
        .for_each(|(type_node, new_fields)| {
            registry.patch_object(type_node.clone(), new_fields)
        });
    return Ok(schema::Schema::from_nodes(&server_nodes));
}
