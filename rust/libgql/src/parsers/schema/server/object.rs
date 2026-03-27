use indexmap::IndexMap;

use crate::parsers::{
    file,
    schema::{
        server::{ast, directive, errors},
        shared,
    },
};

use super::type_registry::HashMapTypeRegistry;

pub fn parse_definition<'buffer>(
    node: &file::server::ast::ObjectDefinitionNode<'buffer>,
    registry: &mut HashMapTypeRegistry,
) -> Result<(), errors::Error<'buffer>> {
    let fields = parse_fields(&node.fields, registry)?;
    let directives = directive::parse_invocations(&node.directives, registry)?;
    let obj = registry.objects.get_mut(node.name.name).unwrap();
    obj.fields = fields;
    for name in node.interfaces.iter() {
        if let None = registry.interfaces.get(name.name) {
            return Err(errors::Error::UnknownInterface(name.clone()));
        };
        obj.implements.insert(name.name.to_string());
    }
    obj.directives = directives;
    return Ok(());
}

pub fn parse_fields<'buffer>(
    fields: &[file::server::ast::FieldDefinitionNode<'buffer>],
    registry: &HashMapTypeRegistry,
) -> Result<
    IndexMap<String, shared::ast::FieldDefinition<ast::ObjectFieldSpec>>,
    errors::Error<'buffer>,
> {
    let mut m = IndexMap::<
        String,
        shared::ast::FieldDefinition<ast::ObjectFieldSpec>,
    >::new();
    for field_definition_node in fields.iter() {
        let (spec, nullable) =
            parse_object_field_spec(&field_definition_node, registry)?;
        m.insert(
            field_definition_node.name.name.to_string(),
            shared::ast::FieldDefinition {
                name: field_definition_node.name.name.to_string(),
                spec,
                nullable,
            },
        );
    }
    return Ok(m);
}

pub fn parse_object_field_spec<'buffer>(
    node: &file::server::ast::FieldDefinitionNode<'buffer>,
    registry: &HashMapTypeRegistry,
) -> Result<(ast::ObjectFieldSpec, bool), errors::Error<'buffer>> {
    let directives = directive::parse_invocations(&node.directives, registry)?;
    let (return_type, nullable) = parse_noncallable_object_field_spec(
        &node.r#type,
        &directives,
        registry,
    )?;
    if node.arguments.is_empty() {
        return Ok((return_type.into(), nullable));
    };
    return Ok((
        ast::CallableFieldSpec {
            return_type,
            arguments: super::input::parse_field_definitions(
                registry,
                &node.arguments,
            )?,
        }
        .into(),
        nullable,
    ));
}

fn parse_noncallable_object_field_spec<'buffer>(
    node: &file::shared::ast::TypeNode<'buffer>,
    directives: &[shared::ast::ServerDirectiveInvocation],
    registry: &HashMapTypeRegistry,
) -> Result<
    (shared::ast::NonCallableFieldSpec<ast::ObjectTypeSpec>, bool),
    errors::Error<'buffer>,
> {
    match node {
        file::shared::ast::TypeNode::List(l) => {
            return Ok((
                shared::ast::ArrayFieldSpec::<ast::ObjectTypeSpec> {
                    r#type: Box::new(
                        parse_noncallable_object_field_spec(
                            &l.r#type,
                            &[],
                            registry,
                        )?
                        .0,
                    ),
                    default_value: None,
                    directive_invocations: directives.to_vec(),
                    nullable: l.r#type.get_nullable(),
                }
                .into(),
                l.nullable,
            ));
        }
        file::shared::ast::TypeNode::Named(n) => {
            return Ok((
                shared::ast::LiteralFieldSpec::<ast::ObjectTypeSpec> {
                    r#type: registry.get_type_for_object(&n.name)?,
                    default_value: None,
                    directive_invocations: directives
                        .iter()
                        .map(|d| (d.directive.clone(), d.clone()))
                        .collect(),
                }
                .into(),
                n.nullable,
            ));
        }
    }
}
