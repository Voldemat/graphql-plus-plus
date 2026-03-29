use crate::parsers::{
    file,
    schema::{
        client::{ast, errors},
        server, shared,
    },
};

pub fn parse<
    'client_buffer,
    'server_buffer,
    ClientStringType: shared::ast::AsStr<'client_buffer>,
    ServerStringType: shared::ast::AsStr<'server_buffer>,
    T: server::type_registry::TypeRegistry<'server_buffer, ServerStringType>,
>(
    registry: &T,
    node: &file::client::ast::DirectiveDefinition<'client_buffer>,
) -> Result<
    ast::ClientDirective<ClientStringType>,
    errors::Error<'client_buffer, ClientStringType>,
> {
    Ok(ast::ClientDirective {
        name: ClientStringType::from_str(node.name.name),
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
