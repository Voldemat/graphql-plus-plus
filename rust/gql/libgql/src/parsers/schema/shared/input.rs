use indexmap::IndexMap;

use crate::parsers::{
    file,
    schema::{
        shared::ast,
        type_registry::{self, TypeRegistry},
    },
};

fn parse_input_field_spec(
    node: &file::shared::ast::InputFieldDefinitionNode,
    registry: &TypeRegistry,
) -> Result<(ast::InputFieldSpec, bool), type_registry::Error> {
    return parse_noncallable_input_field_spec(&node.r#type, None, registry)
        .map(|(return_type, nullable)| (return_type.into(), nullable));
}

fn parse_noncallable_input_field_spec(
    node: &file::shared::ast::TypeNode,
    default_value: Option<ast::Literal>,
    registry: &TypeRegistry,
) -> Result<
    (ast::NonCallableFieldSpec<ast::InputTypeSpec>, bool),
    type_registry::Error,
> {
    match node {
        file::shared::ast::TypeNode::List(l) => {
            return Ok((
                ast::ArrayFieldSpec::<ast::InputTypeSpec> {
                    r#type: registry.get_type_for_input(&l.r#type.name)?,
                    default_value: None,
                    directive_invocations: Vec::new(),
                    nullable: l.r#type.nullable,
                }
                .into(),
                l.nullable,
            ));
        }
        file::shared::ast::TypeNode::Named(n) => {
            return Ok((
                ast::LiteralFieldSpec::<ast::InputTypeSpec> {
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

pub fn parse_field_definition(
    node: &file::shared::ast::InputFieldDefinitionNode,
    registry: &TypeRegistry,
) -> Result<ast::FieldDefinition<ast::InputFieldSpec>, type_registry::Error> {
    let (spec, nullable) = parse_input_field_spec(node, registry)?;
    return Ok(ast::FieldDefinition {
        name: node.name.name.clone(),
        spec,
        nullable,
    });
}

pub fn parse_field_definitions(
    nodes: &[file::shared::ast::InputFieldDefinitionNode],
    registry: &TypeRegistry,
) -> Result<
    IndexMap<String, ast::FieldDefinition<ast::InputFieldSpec>>,
    type_registry::Error,
> {
    let mut arguments =
        IndexMap::<String, ast::FieldDefinition<ast::InputFieldSpec>>::new();
    for field_definition_node in nodes {
        arguments.insert(
            field_definition_node.name.name.clone(),
            parse_field_definition(field_definition_node, registry)?,
        );
    }
    return Ok(arguments);
}
