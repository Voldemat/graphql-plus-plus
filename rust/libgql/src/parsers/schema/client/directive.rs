use std::sync::Arc;

use crate::parsers::{
    file,
    schema::{
        client::{ast, errors},
        shared,
        type_registry::TypeRegistry,
    },
};

pub fn parse<'buffer>(
    registry: &TypeRegistry,
    node: &file::client::ast::DirectiveDefinition<'buffer>,
) -> Result<Arc<ast::ClientDirective>, errors::Error<'buffer>> {
    Ok(Arc::new(ast::ClientDirective {
        name: node.name.name.to_string(),
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
