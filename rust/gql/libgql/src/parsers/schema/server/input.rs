use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;

use super::errors;
use crate::parsers::{
    file::{self, server::ast::InputObjectDefinitionNode},
    schema::{shared, type_registry::TypeRegistry},
};

pub fn parse_definition(
    input: &InputObjectDefinitionNode,
    registry: &mut TypeRegistry,
) -> Result<Rc<RefCell<shared::ast::InputType>>, errors::Error> {
    let obj_rc = registry.inputs.get(&input.name.name).unwrap();
    let mut obj = obj_rc.borrow_mut();
    obj.fields = parse_field_definitions(&input.fields, registry)?;
    return Ok(obj_rc.clone());
}

fn parse_input_field_spec(
    node: &file::shared::ast::InputFieldDefinitionNode,
    registry: &TypeRegistry,
) -> Result<(shared::ast::InputFieldSpec, bool), errors::Error> {
    return parse_noncallable_input_field_spec(&node.r#type, None, registry)
        .map(|(return_type, nullable)| (return_type.into(), nullable));
}

fn parse_noncallable_input_field_spec(
    node: &file::shared::ast::TypeNode,
    default_value: Option<shared::ast::Literal>,
    registry: &TypeRegistry,
) -> Result<
    (
        shared::ast::NonCallableFieldSpec<shared::ast::InputTypeSpec>,
        bool,
    ),
    errors::Error,
> {
    match node {
        file::shared::ast::TypeNode::List(l) => {
            return Ok((
                shared::ast::ArrayFieldSpec::<shared::ast::InputTypeSpec> {
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
                shared::ast::LiteralFieldSpec::<shared::ast::InputTypeSpec> {
                    r#type: registry.get_type_for_input(&n.name)?,
                    default_value,
                    directive_invocations: Vec::new(),
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
) -> Result<
    shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    errors::Error,
> {
    let (spec, nullable) = parse_input_field_spec(node, registry)?;
    return Ok(shared::ast::FieldDefinition {
        name: node.name.name.clone(),
        spec,
        nullable,
    });
}

pub fn parse_field_definitions(
    nodes: &[file::shared::ast::InputFieldDefinitionNode],
    registry: &TypeRegistry,
) -> Result<
    IndexMap<String, shared::ast::FieldDefinition<shared::ast::InputFieldSpec>>,
    errors::Error,
> {
    let mut arguments = IndexMap::<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >::new();
    for field_definition_node in nodes {
        arguments.insert(
            field_definition_node.name.name.clone(),
            parse_field_definition(field_definition_node, registry)?,
        );
    }
    return Ok(arguments);
}
