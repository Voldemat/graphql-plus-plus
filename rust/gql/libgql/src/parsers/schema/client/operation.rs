use crate::parsers::{
    file,
    schema::{client::errors, type_registry::TypeRegistry},
};

pub fn parse(
    registry: &mut TypeRegistry,
    node: &file::client::ast::OperationDefinition,
) -> Result<(), errors::Error> {
    todo!();
}
