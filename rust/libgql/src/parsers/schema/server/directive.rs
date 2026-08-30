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

pub fn parse_arguments<'buffer>(
    arguments: &Vec<file::shared::ast::Argument<'buffer>>,
    directive: &shared::ast::runtime::ServerDirective,
    registry: &HashMapTypeRegistry,
) -> Result<
    indexmap::IndexMap<String, shared::ast::runtime::FieldSelectionArgument>,
    errors::Error<'buffer>,
> {
    let mut final_arguments = indexmap::IndexMap::<
        String,
        shared::ast::runtime::FieldSelectionArgument,
    >::new();
    for argument in arguments {
        let Some(arg_type) = directive.arguments.get(argument.name.name) else {
            return Err(super::type_registry::Error::UnknownArgument(
                argument.name.clone(),
            )
            .into());
        };
        final_arguments.insert(
            argument.name.name.to_string(),
            shared::ast::runtime::FieldSelectionArgument {
                name: argument.name.name.to_string(),
                value: super::arguments::parse_argument_value(
                    &argument.value,
                    arg_type,
                    registry,
                )?,
                r#type: arg_type.clone(),
            },
        );
    }
    return Ok(final_arguments);
}

pub fn parse_invocation<'buffer>(
    node: &file::shared::ast::DirectiveInvocationNode<'buffer>,
    registry: &HashMapTypeRegistry,
) -> Result<
    shared::ast::runtime::ServerDirectiveInvocation,
    errors::Error<'buffer>,
> {
    let Some(directive) = registry.directives.get(node.name.name) else {
        return Err(errors::Error::UnknownServerDirective(node.name.clone()));
    };
    let arguments = parse_arguments(&node.arguments, directive, registry)?;
    return Ok(shared::ast::runtime::ServerDirectiveInvocation {
        directive: node.name.name.to_string(),
        arguments,
    });
}

pub fn parse_invocations<'buffer>(
    nodes: &[file::shared::ast::DirectiveInvocationNode<'buffer>],
    registry: &HashMapTypeRegistry,
) -> Result<
    Vec<shared::ast::runtime::ServerDirectiveInvocation>,
    errors::Error<'buffer>,
> {
    return nodes
        .iter()
        .map(|v| parse_invocation(v, registry))
        .collect();
}
