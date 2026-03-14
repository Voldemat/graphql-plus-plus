use std::rc::Rc;

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
) -> Result<Rc<ast::ClientDirective>, errors::Error> {
    Ok(Rc::new(ast::ClientDirective {
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
