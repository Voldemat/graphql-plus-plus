use std::collections::HashSet;

use crate::parsers::file;

pub mod arguments;
pub mod ast;
pub mod directive;
pub mod errors;
pub mod fragment;
pub mod hash;
pub mod nodes;
pub mod operation;
pub mod server_uses_map;
pub mod type_registry;
pub mod visitor;
use type_registry::TypeRegistry;

use super::server;
use super::shared;

pub fn parse_client_schema<
    'client_buffer,
    'server_buffer: 'client_buffer,
    ClientStringType: shared::ast::AsStr<'client_buffer>,
    ServerStringType: shared::ast::AsStr<'server_buffer> + 'server_buffer,
    T: server::type_registry::TypeRegistry<'server_buffer, ServerStringType>,
>(
    server_registry: &'server_buffer T,
    registry: &mut TypeRegistry<ClientStringType>,
    ast_nodes: &[file::client::ast::ASTNode<'client_buffer>],
) -> Result<(), Vec<errors::Error<'client_buffer, ClientStringType>>> {
    let mut skip_indices = HashSet::new();
    let mut errors = ast_nodes
        .iter()
        .enumerate()
        .filter_map(|(index, node)| {
            match nodes::parse_first_pass(server_registry, registry, node) {
                Ok(_) => None,
                Err(error) => {
                    skip_indices.insert(index);
                    Some(error)
                }
            }
        })
        .collect::<Vec<_>>();
    errors.extend(
        ast_nodes
            .iter()
            .enumerate()
            .filter_map(|(index, node)| {
                if skip_indices.contains(&index) {
                    None
                } else {
                    Some(node)
                }
            })
            .filter_map(|node| {
                nodes::parse_second_pass(server_registry, registry, node).err()
            })
            .collect::<Vec<_>>(),
    );
    if errors.len() > 0 {
        return Err(errors);
    }
    let mut intermediate = Vec::new();
    for operation in registry.operations.values() {
        let parameters_hash =
            hash::get_operation_parameters_hash(&operation.parameters);
        let fragment_spec_hash = hash::get_fragment_spec_hash(
            registry,
            &operation.fragment_spec,
            true,
        );
        let used_fragments = hash::get_used_fragments_from_fragment_spec(
            registry,
            &operation.fragment_spec,
        );
        intermediate.push((
            parameters_hash,
            fragment_spec_hash,
            used_fragments,
        ));
    }
    for (operation, (parameters_hash, fragment_spec_hash, used_fragments)) in
        registry.operations.values_mut().zip(intermediate)
    {
        operation.used_fragments = used_fragments;
        operation.parameters_hash = parameters_hash;
        operation.fragment_spec_hash = fragment_spec_hash;
    }
    return Ok(());
}
