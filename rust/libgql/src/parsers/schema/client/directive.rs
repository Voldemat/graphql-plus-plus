use crate::parsers::{
    file,
    schema::{
        client::{ast, errors},
        server,
    },
};

pub fn parse<'buffer, T: server::type_registry::TypeRegistry>(
    registry: &T,
    node: &file::client::ast::DirectiveDefinition<'buffer>,
) -> Result<ast::ClientDirective, errors::Error<'buffer>> {
    Ok(ast::ClientDirective {
        name: node.name.name.to_string(),
        arguments: server::input::parse_field_definitions(
            registry,
            &node.arguments,
        )?,
        locations: node
            .targets
            .iter()
            .map(|target| target.directive_location)
            .collect(),
    })
}
