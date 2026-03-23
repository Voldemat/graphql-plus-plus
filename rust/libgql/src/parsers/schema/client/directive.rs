use std::sync::Arc;

use crate::parsers::{
    file,
    schema::{
        client::{ast, errors},
        shared,
        type_registry::TypeRegistry,
    },
};

pub fn parse(
    registry: &TypeRegistry,
    node: &file::client::ast::DirectiveDefinition,
) -> Result<Arc<ast::ClientDirective>, errors::Error> {
    Ok(Arc::new(ast::ClientDirective {
        name: node.name.name.clone(),
        arguments: shared::input::parse_field_definitions(
            &node.arguments,
            registry,
        )?,
        locations: node
            .targets
            .iter()
            .map(|target| target.directive_location)
            .collect(),
    }))
}
