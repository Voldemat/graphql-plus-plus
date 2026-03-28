use crate::parsers::{
    file,
    schema::{
        client::{ast, errors},
        server, shared,
    },
};

pub fn parse<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: server::type_registry::TypeRegistry<'buffer, S>,
>(
    registry: &T,
    node: &file::client::ast::DirectiveDefinition<'buffer>,
) -> Result<ast::ClientDirective<S>, errors::Error<'buffer, S>> {
    Ok(ast::ClientDirective {
        name: S::from_str(node.name.name),
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
