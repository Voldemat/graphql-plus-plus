use indexmap::IndexMap;

use super::{
    errors,
    type_registry::{self, TypeRegistry},
};
use crate::parsers::{
    file::{self, server::ast::InputObjectDefinitionNode},
    schema::shared::{self, ast::traits::InputTypeSpec},
};

fn parse_input_field_spec<
    'input_buffer,
    'server_buffer,
    InputStringType: shared::ast::AsStr<'input_buffer>,
    ServerStringType: shared::ast::AsStr<'server_buffer>,
    T: TypeRegistry<'server_buffer, ServerStringType>,
>(
    registry: &T,
    node: &file::shared::ast::InputFieldDefinitionNode<'input_buffer>,
) -> Result<
    (shared::ast::runtime::InputFieldSpec<InputStringType>, bool),
    type_registry::Error<'input_buffer>,
> {
    return parse_noncallable_input_field_spec(
        registry,
        &node.r#type,
        node.default_value.as_ref(),
    )
    .map(|(return_type, nullable)| (return_type.into(), nullable));
}

fn parse_noncallable_input_field_spec<
    'input_buffer,
    'server_buffer,
    InputStringType: shared::ast::AsStr<'input_buffer>,
    ServerStringType: shared::ast::AsStr<'server_buffer>,
    T: TypeRegistry<'server_buffer, ServerStringType>,
>(
    registry: &T,
    node: &file::shared::ast::TypeNode<'input_buffer>,
    default_value: Option<&file::shared::ast::LiteralNode<'input_buffer>>,
) -> Result<
    (
        shared::ast::runtime::NonCallableFieldSpec<
            shared::ast::runtime::InputTypeSpec<InputStringType>,
            InputStringType,
        >,
        bool,
    ),
    type_registry::Error<'input_buffer>,
> {
    match node {
        file::shared::ast::TypeNode::List(l) => {
            return Ok((
                shared::ast::runtime::ArrayFieldSpec::<
                    shared::ast::runtime::InputTypeSpec<InputStringType>,
                    InputStringType,
                > {
                    r#type: Box::new(
                        parse_noncallable_input_field_spec(
                            registry, &l.r#type, None,
                        )?
                        .0,
                    ),
                    default_value: None,
                    directive_invocations: Vec::new(),
                    nullable: l.r#type.get_nullable(),
                }
                .into(),
                l.nullable,
            ));
        }
        file::shared::ast::TypeNode::Named(n) => {
            let r#type = registry
                .get_input_type_spec_by_name(&n.name)
                .ok_or(type_registry::Error::UnknownType(n.name.clone()))?;
            let dvalue = default_value
                .map(|v| {
                    shared::default_value::parse_default_value(
                        r#type.get_ref(),
                        n.nullable,
                        v,
                    )
                })
                .transpose()?
                .flatten();
            return Ok((
                shared::ast::runtime::LiteralFieldSpec::<
                    shared::ast::runtime::InputTypeSpec<InputStringType>,
                    InputStringType,
                > {
                    r#type,
                    default_value: dvalue,
                    directive_invocations: IndexMap::new(),
                }
                .into(),
                n.nullable,
            ));
        }
    }
}

pub fn parse_field_definition<
    'input_buffer,
    'server_buffer,
    InputStringType: shared::ast::AsStr<'input_buffer>,
    ServerStringType: shared::ast::AsStr<'server_buffer>,
    T: TypeRegistry<'server_buffer, ServerStringType>,
>(
    registry: &T,
    node: &file::shared::ast::InputFieldDefinitionNode<'input_buffer>,
) -> Result<
    shared::ast::runtime::FieldDefinition<
        shared::ast::runtime::InputFieldSpec<InputStringType>,
        InputStringType,
    >,
    type_registry::Error<'input_buffer>,
> {
    let (spec, nullable) = parse_input_field_spec(registry, node)?;
    return Ok(shared::ast::runtime::FieldDefinition {
        name: InputStringType::from_str(node.name.name),
        spec,
        nullable,
    });
}

pub fn parse_field_definitions<
    'input_buffer,
    'server_buffer,
    InputStringType: shared::ast::AsStr<'input_buffer>,
    ServerStringType: shared::ast::AsStr<'server_buffer>,
    T: TypeRegistry<'server_buffer, ServerStringType>,
>(
    registry: &T,
    nodes: &[file::shared::ast::InputFieldDefinitionNode<'input_buffer>],
) -> Result<
    IndexMap<
        InputStringType,
        shared::ast::runtime::FieldDefinition<
            shared::ast::runtime::InputFieldSpec<InputStringType>,
            InputStringType,
        >,
    >,
    type_registry::Error<'input_buffer>,
> {
    let mut arguments = IndexMap::<
        InputStringType,
        shared::ast::runtime::FieldDefinition<
            shared::ast::runtime::InputFieldSpec<InputStringType>,
            InputStringType,
        >,
    >::new();
    for field_definition_node in nodes {
        arguments.insert(
            InputStringType::from_str(field_definition_node.name.name),
            parse_field_definition(registry, field_definition_node)?,
        );
    }
    return Ok(arguments);
}

pub fn parse_definition<'buffer>(
    registry: &mut type_registry::HashMapTypeRegistry,
    input: &InputObjectDefinitionNode<'buffer>,
) -> Result<(), errors::Error<'buffer>> {
    registry.inputs.get_mut(input.name.name).unwrap().fields =
        super::input::parse_field_definitions(registry, &input.fields)?;
    return Ok(());
}
