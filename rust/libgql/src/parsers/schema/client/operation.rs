use crate::parsers::{
    file,
    schema::{
        client::{errors, fragment},
        server,
    },
};

use super::type_registry::TypeRegistry;

pub fn parse<'buffer, T: server::type_registry::TypeRegistry>(
    server_registry: &T,
    registry: &mut TypeRegistry,
    node: &file::client::ast::OperationDefinition<'buffer>,
) -> Result<(), errors::Error<'buffer>> {
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
        .insert(node.name.name.to_string(), operation);
    return Ok(());
}
