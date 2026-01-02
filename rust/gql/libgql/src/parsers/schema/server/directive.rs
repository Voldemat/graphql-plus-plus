use std::{cell::RefCell, rc::Rc};

use crate::parsers::{
    file,
    schema::{server::errors, shared, type_registry::TypeRegistry},
};

pub fn parse_definition(
    node: &file::server::ast::DirectiveDefinitionNode,
    registry: &mut TypeRegistry,
) -> Result<Rc<RefCell<shared::ast::ServerDirective>>, errors::Error> {
    let directive = registry.server_directives.get(&node.name.name).unwrap();
    for arg in &node.arguments {
        directive.borrow_mut().arguments.insert(
            arg.name.name.clone(),
            shared::input::parse_field_definition(&arg, registry)?,
        );
    }
    directive.borrow_mut().locations = node
        .targets
        .iter()
        .map(|v| v.directive_location)
        .collect::<Vec<_>>();
    return Ok(directive.clone());
}

pub fn parse_invocation(
    node: &file::shared::ast::DirectiveInvocationNode,
    registry: &TypeRegistry,
) -> Result<shared::ast::ServerDirectiveInvocation, errors::Error> {
    let Some(directive) = registry.server_directives.get(&node.name.name)
    else {
        return Err(errors::Error::UnknownServerDirective(node.name.clone()));
    };
    let arguments =
        shared::arguments::parse_arguments(&node.arguments, directive)?;
    return Ok(shared::ast::ServerDirectiveInvocation {
        directive: directive.clone(),
        arguments,
    });
}

pub fn parse_invocations(
    nodes: &[file::shared::ast::DirectiveInvocationNode],
    registry: &TypeRegistry,
) -> Result<Vec<shared::ast::ServerDirectiveInvocation>, errors::Error> {
    return nodes
        .iter()
        .map(|v| parse_invocation(v, registry))
        .collect();
}
