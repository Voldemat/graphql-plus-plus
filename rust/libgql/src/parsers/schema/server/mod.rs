pub mod arguments;
pub mod ast;
pub mod directive;
pub mod errors;
pub mod input;
pub mod interface;
pub mod nodes;
pub mod object;
pub mod scalars;
pub mod type_registry;
pub mod union;

use crate::parsers::file;
pub use errors::Error;

use self::type_registry::TypeRegistry;

pub fn parse_server_schema<'buffer>(
    mut registry: &mut TypeRegistry,
    ast_nodes: &[file::server::ast::ASTNode<'buffer>],
) -> Result<(), errors::Error<'buffer>> {
    let type_definition_nodes = || {
        ast_nodes.iter().filter_map(|node| match node {
            file::server::ast::ASTNode::TypeDefinitionNode(n) => Some(n),
            _ => None,
        })
    };
    type_definition_nodes().try_for_each(|node| {
        nodes::parse_server_node_first_pass(registry, node)
    })?;
    type_definition_nodes().try_for_each(|node| {
        nodes::parse_server_node_second_pass(node, &mut registry)
    })?;
    ast_nodes
        .iter()
        .filter_map(|node| match node {
            file::server::ast::ASTNode::ExtendTypeNode(n) => Some(n),
            _ => None,
        })
        .map(|node| nodes::parse_server_extend_node(node, &mut registry))
        .collect::<Result<Vec<_>, errors::Error>>()?
        .into_iter()
        .for_each(|(type_node, new_fields)| {
            registry.patch_object(type_node, new_fields)
        });
    return Ok(());
}
