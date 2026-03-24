use std::sync::{Arc, RwLock};

use crate::parsers::{
    file,
    schema::{server::errors, shared, type_registry::TypeRegistry},
};

pub fn parse_definition<'buffer>(
    node: &file::server::ast::DirectiveDefinitionNode<'buffer>,
    registry: &mut TypeRegistry,
) -> Result<Arc<RwLock<shared::ast::ServerDirective>>, errors::Error<'buffer>> {
    let directive = registry.server_directives.get(node.name.name).unwrap();
    for arg in &node.arguments {
        directive.write().unwrap().arguments.insert(
            arg.name.name.to_string(),
            shared::input::parse_field_definition(&arg, registry)?,
        );
    }
    directive.write().unwrap().locations = node
        .targets
        .iter()
        .map(|v| v.directive_location)
        .collect::<Vec<_>>();
    return Ok(directive.clone());
}

pub fn parse_invocation<'buffer>(
    node: &file::shared::ast::DirectiveInvocationNode<'buffer>,
    registry: &TypeRegistry,
) -> Result<shared::ast::ServerDirectiveInvocation, errors::Error<'buffer>> {
    let Some(directive) = registry.server_directives.get(node.name.name) else {
        return Err(errors::Error::UnknownServerDirective(node.name.clone()));
    };
    let arguments =
        shared::arguments::parse_arguments(&node.arguments, directive)?;
    return Ok(shared::ast::ServerDirectiveInvocation {
        directive: directive.clone(),
        arguments,
    });
}

pub fn parse_invocations<'buffer>(
    nodes: &[file::shared::ast::DirectiveInvocationNode<'buffer>],
    registry: &TypeRegistry,
) -> Result<Vec<shared::ast::ServerDirectiveInvocation>, errors::Error<'buffer>>
{
    return nodes
        .iter()
        .map(|v| parse_invocation(v, registry))
        .collect();
}
