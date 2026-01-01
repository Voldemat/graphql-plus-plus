use std::{cell::RefCell, rc::Rc};

use crate::parsers::{
    file,
    schema::{
        server::{
            ast,
            directive,
            errors, input,
        },
        shared,
        type_registry::TypeRegistry,
    },
};

pub fn parse_definition(
    node: &file::server::ast::ObjectDefinitionNode,
    registry: &TypeRegistry,
) -> Result<Rc<RefCell<ast::ObjectType>>, errors::Error> {
    let obj_rc = registry.objects.get(&node.name.name).unwrap();
    let mut obj = obj_rc.borrow_mut();
    for field_definition_node in node.fields.iter() {
        let (spec, nullable) =
            parse_object_field_spec(&field_definition_node, registry)?;
        obj.fields.insert(
            field_definition_node.name.name.clone(),
            Rc::new(shared::ast::FieldDefinition {
                name: field_definition_node.name.name.clone(),
                spec,
                nullable,
            }),
        );
    }
    for name in node.interfaces.iter() {
        let Some(interface) = registry.interfaces.get(&name.name) else {
            return Err(errors::Error::UnknownInterface(name.clone()))
        };
        obj.implements.insert(name.name.clone(), interface.clone());
    }
    obj.directives = directive::parse_invocations(&node.directives, registry)?;
    return Ok(obj_rc.clone());
}

pub fn parse_object_field_spec(
    node: &file::server::ast::FieldDefinitionNode,
    registry: &TypeRegistry,
) -> Result<(ast::ObjectFieldSpec, bool), errors::Error> {
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
            arguments: input::parse_field_definitions(
                &node.arguments,
                registry,
            )?,
        }
        .into(),
        nullable,
    ));
}

fn parse_noncallable_object_field_spec(
    node: &file::shared::ast::TypeNode,
    directives: &[shared::ast::ServerDirectiveInvocation],
    registry: &TypeRegistry,
) -> Result<
    (shared::ast::NonCallableFieldSpec<ast::ObjectTypeSpec>, bool),
    errors::Error,
> {
    match node {
        file::shared::ast::TypeNode::List(l) => {
            return Ok((
                shared::ast::ArrayFieldSpec::<ast::ObjectTypeSpec> {
                    r#type: registry.get_type_for_object(&l.r#type.name)?,
                    default_value: None,
                    directive_invocations: directives.to_vec(),
                    nullable: l.r#type.nullable,
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
                    directive_invocations: directives.to_vec(),
                }
                .into(),
                n.nullable,
            ));
        }
    }
}
