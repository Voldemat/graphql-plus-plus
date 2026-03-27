use indexmap::IndexMap;

use crate::parsers::{
    file,
    schema::{server::errors, shared},
};

use super::type_registry::HashMapTypeRegistry;

pub fn parse_definition<'buffer>(
    node: &file::server::ast::DirectiveDefinitionNode<'buffer>,
    registry: &mut HashMapTypeRegistry,
) -> Result<(), errors::Error<'buffer>> {
    let mut arguments = IndexMap::new();
    for arg in &node.arguments {
        arguments.insert(
            arg.name.name.to_string(),
            super::input::parse_field_definition(registry, &arg)?,
        );
    }
    let directive = registry.directives.get_mut(node.name.name).unwrap();
    directive.arguments = arguments;
    directive.locations = node
        .targets
        .iter()
        .map(|v| v.directive_location)
        .collect::<Vec<_>>();
    return Ok(());
}

pub fn parse_invocation<'buffer>(
    node: &file::shared::ast::DirectiveInvocationNode<'buffer>,
    registry: &HashMapTypeRegistry,
) -> Result<shared::ast::ServerDirectiveInvocation, errors::Error<'buffer>> {
    let Some(directive) = registry.directives.get(node.name.name) else {
        return Err(errors::Error::UnknownServerDirective(node.name.clone()));
    };
    let arguments = super::arguments::parse_arguments(
        &node.arguments,
        directive,
        registry,
    )?;
    return Ok(shared::ast::ServerDirectiveInvocation {
        directive: node.name.name.to_string(),
        arguments,
    });
}

pub fn parse_invocations<'buffer>(
    nodes: &[file::shared::ast::DirectiveInvocationNode<'buffer>],
    registry: &HashMapTypeRegistry,
) -> Result<Vec<shared::ast::ServerDirectiveInvocation>, errors::Error<'buffer>>
{
    return nodes
        .iter()
        .map(|v| parse_invocation(v, registry))
        .collect();
}
