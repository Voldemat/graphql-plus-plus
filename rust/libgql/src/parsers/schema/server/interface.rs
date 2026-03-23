use std::sync::{Arc, RwLock};

use crate::parsers::{
    file,
    schema::{
        server::{ast, directive, errors, object},
        shared,
        type_registry::TypeRegistry,
    },
};

pub fn parse_definition(
    node: &file::server::ast::InterfaceDefinitionNode,
    registry: &TypeRegistry,
) -> Result<Arc<RwLock<ast::Interface>>, errors::Error> {
    let obj_rc = registry.interfaces.get(&node.name.name).unwrap();
    let mut obj = obj_rc.write().unwrap();
    for field_definition_node in node.fields.iter() {
        let (spec, nullable) =
            object::parse_object_field_spec(&field_definition_node, registry)?;
        obj.fields.insert(
            field_definition_node.name.name.clone(),
            Arc::new(shared::ast::FieldDefinition {
                name: field_definition_node.name.name.clone(),
                spec,
                nullable,
            }),
        );
    }
    obj.directives = directive::parse_invocations(&node.directives, registry)?;
    return Ok(obj_rc.clone());
}
