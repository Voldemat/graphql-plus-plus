use indexmap::IndexMap;

use super::{
    errors,
    type_registry::{self, TypeRegistry},
};
use crate::parsers::{
    file::{self, server::ast::InputObjectDefinitionNode},
    schema::shared,
};

fn parse_input_field_spec<'buffer>(
    node: &file::shared::ast::InputFieldDefinitionNode<'buffer>,
    registry: &TypeRegistry,
) -> Result<(shared::ast::InputFieldSpec, bool), type_registry::Error<'buffer>>
{
    return parse_noncallable_input_field_spec(
        &node.r#type,
        node.default_value
            .as_ref()
            .map(shared::literal::parse_literal),
        registry,
    )
    .map(|(return_type, nullable)| (return_type.into(), nullable));
}

fn parse_noncallable_input_field_spec<'buffer>(
    node: &file::shared::ast::TypeNode<'buffer>,
    default_value: Option<shared::ast::Literal>,
    registry: &TypeRegistry,
) -> Result<
    (shared::ast::NonCallableFieldSpec<shared::ast::InputTypeSpec>, bool),
    type_registry::Error<'buffer>,
> {
    match node {
        file::shared::ast::TypeNode::List(l) => {
            return Ok((
                shared::ast::ArrayFieldSpec::<shared::ast::InputTypeSpec> {
                    r#type: Box::new(
                        parse_noncallable_input_field_spec(
                            &l.r#type, None, registry,
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
            return Ok((
                shared::ast::LiteralFieldSpec::<shared::ast::InputTypeSpec> {
                    r#type: registry.get_type_for_input(&n.name)?,
                    default_value: Some(default_value),
                    directive_invocations: IndexMap::new(),
                }
                .into(),
                n.nullable,
            ));
        }
    }
}

pub fn parse_field_definition<'buffer>(
    node: &file::shared::ast::InputFieldDefinitionNode<'buffer>,
    registry: &TypeRegistry,
) -> Result<
    shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    type_registry::Error<'buffer>,
> {
    let (spec, nullable) = parse_input_field_spec(node, registry)?;
    return Ok(shared::ast::FieldDefinition {
        name: node.name.name.to_string(),
        spec,
        nullable,
    });
}

pub fn parse_field_definitions<'buffer>(
    nodes: &[file::shared::ast::InputFieldDefinitionNode<'buffer>],
    registry: &TypeRegistry,
) -> Result<
    IndexMap<String, shared::ast::FieldDefinition<shared::ast::InputFieldSpec>>,
    type_registry::Error<'buffer>,
> {
    let mut arguments =
        IndexMap::<String, shared::ast::FieldDefinition<shared::ast::InputFieldSpec>>::new();
    for field_definition_node in nodes {
        arguments.insert(
            field_definition_node.name.name.to_string(),
            parse_field_definition(field_definition_node, registry)?,
        );
    }
    return Ok(arguments);
}

pub fn parse_definition<'buffer>(
    input: &InputObjectDefinitionNode<'buffer>,
    registry: &mut TypeRegistry,
) -> Result<(), errors::Error<'buffer>> {
    let fields = super::input::parse_field_definitions(&input.fields, registry)?;
    let obj = registry.inputs.get_mut(input.name.name).unwrap();
    obj.fields = fields;
    return Ok(());
}
