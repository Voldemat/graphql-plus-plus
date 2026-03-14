use crate::parsers::{
    file,
    schema::{
        client::{errors, fragment},
        type_registry::TypeRegistry,
    },
};

pub fn parse(
    registry: &mut TypeRegistry,
    node: &file::client::ast::OperationDefinition,
) -> Result<(), errors::Error> {
    let operation_rc = registry.operations.get(&node.name.name).unwrap();
    let mut operation = operation_rc.borrow_mut();
    fragment::parse_selections(
        registry,
        &mut operation.fragment_spec,
        &node.fragment.selections,
    )?;
    return Ok(());
}
