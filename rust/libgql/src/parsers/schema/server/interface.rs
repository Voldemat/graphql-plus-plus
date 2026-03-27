use crate::parsers::{
    file,
    schema::{
        server::{directive, errors, object},
        shared,
    },
};

use super::type_registry::HashMapTypeRegistry;

pub fn parse_definition<'buffer>(
    node: &file::server::ast::InterfaceDefinitionNode<'buffer>,
    registry: &mut HashMapTypeRegistry,
) -> Result<(), errors::Error<'buffer>> {
    let mut intermediate = Vec::new();
    for field_definition_node in node.fields.iter() {
        intermediate.push(object::parse_object_field_spec(
            &field_definition_node,
            registry,
        )?);
    }
    let directives = directive::parse_invocations(&node.directives, registry)?;
    let obj = registry.interfaces.get_mut(node.name.name).unwrap();
    for (field_definition_node, (spec, nullable)) in
        node.fields.iter().zip(intermediate)
    {
        obj.fields.insert(
            field_definition_node.name.name.to_string(),
            shared::ast::FieldDefinition {
                name: field_definition_node.name.name.to_string(),
                spec,
                nullable,
            },
        );
    }
    obj.directives = directives;
    return Ok(());
}
