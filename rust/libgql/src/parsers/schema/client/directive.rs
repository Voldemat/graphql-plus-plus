use crate::parsers::{
    file,
    schema::{
        client::{ast, errors},
        server,
    },
};

pub fn parse<'buffer>(
    registry: &server::type_registry::TypeRegistry,
    node: &file::client::ast::DirectiveDefinition<'buffer>,
) -> Result<ast::ClientDirective, errors::Error<'buffer>> {
    Ok(ast::ClientDirective {
        name: node.name.name.to_string(),
        arguments: server::input::parse_field_definitions(
            &node.arguments,
            registry,
        )?,
        locations: node
            .targets
            .iter()
            .map(|target| target.directive_location)
            .collect(),
    })
}
