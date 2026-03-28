use indexmap::IndexMap;

use super::{
    errors,
    type_registry::{self, TypeRegistry},
};
use crate::parsers::{
    file::{self, server::ast::InputObjectDefinitionNode},
    schema::shared,
};

fn parse_input_field_spec<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: TypeRegistry<'buffer, S>,
>(
    registry: &T,
    node: &file::shared::ast::InputFieldDefinitionNode<'buffer>,
) -> Result<(shared::ast::InputFieldSpec<S>, bool), type_registry::Error<'buffer>>
{
    return parse_noncallable_input_field_spec(
        registry,
        &node.r#type,
        node.default_value
            .as_ref()
            .map(shared::literal::parse_literal),
    )
    .map(|(return_type, nullable)| (return_type.into(), nullable));
}

fn parse_noncallable_input_field_spec<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: TypeRegistry<'buffer, S>,
>(
    registry: &T,
    node: &file::shared::ast::TypeNode<'buffer>,
    default_value: Option<shared::ast::Literal>,
) -> Result<
    (
        shared::ast::NonCallableFieldSpec<shared::ast::InputTypeSpec<S>, S>,
        bool,
    ),
    type_registry::Error<'buffer>,
> {
    match node {
        file::shared::ast::TypeNode::List(l) => {
            return Ok(
                (
                    shared::ast::ArrayFieldSpec::<
                        shared::ast::InputTypeSpec<S>,
                        S,
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
                ),
            );
        }
        file::shared::ast::TypeNode::Named(n) => {
            return Ok((
                shared::ast::LiteralFieldSpec::<
                    shared::ast::InputTypeSpec<S>,
                    S,
                > {
                    r#type: registry
                        .get_input_type_spec_by_name(n.name.name)
                        .ok_or(type_registry::Error::UnknownType(
                        n.name.clone(),
                    ))?,
                    default_value: Some(default_value),
                    directive_invocations: IndexMap::new(),
                }
                .into(),
                n.nullable,
            ));
        }
    }
}

pub fn parse_field_definition<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: TypeRegistry<'buffer, S>,
>(
    registry: &T,
    node: &file::shared::ast::InputFieldDefinitionNode<'buffer>,
) -> Result<
    shared::ast::FieldDefinition<shared::ast::InputFieldSpec<S>, S>,
    type_registry::Error<'buffer>,
> {
    let (spec, nullable) = parse_input_field_spec(registry, node)?;
    return Ok(shared::ast::FieldDefinition {
        name: S::from_str(node.name.name),
        spec,
        nullable,
    });
}

pub fn parse_field_definitions<
    'buffer,
    S: shared::ast::AsStr<'buffer>,
    T: TypeRegistry<'buffer, S>,
>(
    registry: &T,
    nodes: &[file::shared::ast::InputFieldDefinitionNode<'buffer>],
) -> Result<
    IndexMap<
        S,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec<S>, S>,
    >,
    type_registry::Error<'buffer>,
> {
    let mut arguments = IndexMap::<
        S,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec<S>, S>,
    >::new();
    for field_definition_node in nodes {
        arguments.insert(
            S::from_str(field_definition_node.name.name),
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
