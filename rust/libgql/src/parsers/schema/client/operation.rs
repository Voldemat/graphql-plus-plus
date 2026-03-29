use crate::parsers::{
    file,
    schema::{
        client::{errors, fragment},
        server, shared,
    },
};

use super::type_registry::TypeRegistry;

pub fn parse<
    'client_buffer,
    'server_buffer: 'client_buffer,
    ClientStringType: shared::ast::AsStr<'client_buffer>,
    ServerStringType: shared::ast::AsStr<'server_buffer> + 'server_buffer,
    T: server::type_registry::TypeRegistry<'server_buffer, ServerStringType>,
>(
    server_registry: &'server_buffer T,
    registry: &mut TypeRegistry<ClientStringType>,
    node: &file::client::ast::OperationDefinition<'client_buffer>,
) -> Result<(), errors::Error<'client_buffer, ClientStringType>> {
    let mut operation =
        registry.operations.swap_remove(node.name.name).unwrap();
    fragment::parse_selections(
        server_registry,
        registry,
        &mut operation.fragment_spec,
        &node.fragment.selections,
    )?;
    registry
        .operations
        .insert(ClientStringType::from_str(node.name.name), operation);
    return Ok(());
}
