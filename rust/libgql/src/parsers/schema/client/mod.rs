use crate::parsers::file;

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
) -> Result<(), errors::Error<'client_buffer, ClientStringType>> {
    ast_nodes.iter().try_for_each(|node| {
        nodes::parse_first_pass(server_registry, registry, node)
    })?;
    ast_nodes.iter().try_for_each(|node| {
        nodes::parse_second_pass(server_registry, registry, node)
    })?;
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
