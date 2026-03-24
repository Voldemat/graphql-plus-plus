use indexmap::IndexMap;

use crate::parsers::{
    file,
    schema::{
        shared::ast,
        type_registry::{self, TypeRegistry},
    },
};

fn parse_literal(node: &file::shared::ast::LiteralNode) -> ast::Literal {
    match node {
        file::shared::ast::LiteralNode::Int(i) => ast::Literal::Int(i.value),
        file::shared::ast::LiteralNode::Float(i) => {
            ast::Literal::Float(i.value)
        }
        file::shared::ast::LiteralNode::Boolean(i) => {
            ast::Literal::Boolean(i.value)
        }
        file::shared::ast::LiteralNode::String(i) => {
            ast::Literal::String(i.value.clone())
        }
        file::shared::ast::LiteralNode::EnumValue(i) => {
            ast::Literal::String(i.value.clone())
        }
    }
}

fn parse_input_field_spec<'buffer>(
    node: &file::shared::ast::InputFieldDefinitionNode<'buffer>,
    registry: &TypeRegistry,
) -> Result<(ast::InputFieldSpec, bool), type_registry::Error<'buffer>> {
    return parse_noncallable_input_field_spec(
        &node.r#type,
        node.default_value.as_ref().map(parse_literal),
        registry,
    )
    .map(|(return_type, nullable)| (return_type.into(), nullable));
}

fn parse_noncallable_input_field_spec<'buffer>(
    node: &file::shared::ast::TypeNode<'buffer>,
    default_value: Option<ast::Literal>,
    registry: &TypeRegistry,
) -> Result<
    (ast::NonCallableFieldSpec<ast::InputTypeSpec>, bool),
    type_registry::Error<'buffer>,
> {
    match node {
        file::shared::ast::TypeNode::List(l) => {
            return Ok((
                ast::ArrayFieldSpec::<ast::InputTypeSpec> {
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

pub fn parse_field_definition<'buffer>(
    node: &file::shared::ast::InputFieldDefinitionNode<'buffer>,
    registry: &TypeRegistry,
) -> Result<
    ast::FieldDefinition<ast::InputFieldSpec>,
    type_registry::Error<'buffer>,
> {
    let (spec, nullable) = parse_input_field_spec(node, registry)?;
    return Ok(ast::FieldDefinition {
        name: node.name.name.to_string(),
        spec,
        nullable,
    });
}

pub fn parse_field_definitions<'buffer>(
    nodes: &[file::shared::ast::InputFieldDefinitionNode<'buffer>],
    registry: &TypeRegistry,
) -> Result<
    IndexMap<String, ast::FieldDefinition<ast::InputFieldSpec>>,
    type_registry::Error<'buffer>,
> {
    let mut arguments =
        IndexMap::<String, ast::FieldDefinition<ast::InputFieldSpec>>::new();
    for field_definition_node in nodes {
        arguments.insert(
            field_definition_node.name.name.to_string(),
            parse_field_definition(field_definition_node, registry)?,
        );
    }
    return Ok(arguments);
}
