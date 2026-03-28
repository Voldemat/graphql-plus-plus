use crate::parsers::{
    file,
    schema::{
        client::{errors, fragment},
        server, shared,
    },
};

use super::type_registry::TypeRegistry;

pub fn parse<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    server_registry: &T,
    registry: &mut TypeRegistry<S>,
    node: &file::client::ast::OperationDefinition<'buffer>,
) -> Result<(), errors::Error<'buffer, S>> {
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
        .insert(S::from_str(node.name.name), operation);
    return Ok(());
}
